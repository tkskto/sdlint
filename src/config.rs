use std::{
    collections::BTreeMap,
    env, fs, io,
    path::{Path, PathBuf},
};

use glob::{MatchOptions, Pattern};
use serde::Deserialize;
use thiserror::Error;

use crate::{cli::FailOn, diagnostic::Severity, input::SourceOrigin};

use input_filter::InputFilter;
use rule_settings::{RuleLevel, RuleSettings};

mod input_filter;
mod rule_settings;

const CONFIG_FILE_NAME: &str = "sdlint.toml";

#[derive(Debug)]
pub struct LoadedConfig {
    working_directory: PathBuf,
    input_filter: InputFilter,
    rule_settings: RuleSettings,
    override_list: Vec<PathOverride>,
    fail_on: Option<FailOn>,
}

impl LoadedConfig {
    pub fn load() -> Result<Self, ConfigError> {
        let working_directory = env::current_dir().map_err(ConfigError::WorkingDirectory)?;
        Self::load_from(&working_directory)
    }

    fn load_from(working_directory: &Path) -> Result<Self, ConfigError> {
        let config_path = find_config(working_directory)?;
        let config_file = match &config_path {
            Some(path) => {
                let source = fs::read_to_string(path).map_err(|source| ConfigError::Read {
                    path: path.clone(),
                    source,
                })?;
                toml::from_str::<ConfigFile>(&source).map_err(|source| ConfigError::Parse {
                    path: path.clone(),
                    source,
                })?
            }
            None => ConfigFile::default(),
        };

        let config_directory = config_path
            .as_deref()
            .and_then(Path::parent)
            .unwrap_or(working_directory);
        let input_filter =
            InputFilter::build(working_directory, config_directory, &config_file.files)?;
        let rule_settings = RuleSettings::build(config_file.rules, "[rules]")?;
        let override_list = config_file
            .overrides
            .into_iter()
            .enumerate()
            .map(|(index, path_override)| PathOverride::build(path_override, index + 1))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            working_directory: working_directory.to_path_buf(),
            input_filter,
            rule_settings,
            override_list,
            fail_on: config_file.exit.fail_on.map(ConfiguredFailOn::into_fail_on),
        })
    }

    pub fn should_ignore(&self, path: &Path) -> bool {
        self.input_filter.excludes(path)
    }

    pub fn fail_on(&self) -> Option<FailOn> {
        self.fail_on
    }

    pub fn matching_override(&self, origin: &SourceOrigin) -> Result<Option<usize>, ConfigError> {
        let SourceOrigin::Path(path) = origin else {
            return Ok(None);
        };
        let relative_path = path
            .strip_prefix(&self.working_directory)
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/");
        let matching_override_list = self
            .override_list
            .iter()
            .enumerate()
            .filter(|(_, path_override)| path_override.matches(&relative_path))
            .map(|(index, _)| index)
            .collect::<Vec<_>>();

        match matching_override_list.as_slice() {
            [] => Ok(None),
            [index] => Ok(Some(*index)),
            _ => Err(ConfigError::AmbiguousOverrides {
                path: path.clone(),
                override_number_list: matching_override_list
                    .into_iter()
                    .map(|index| index + 1)
                    .collect(),
            }),
        }
    }

    pub fn resolve_rule_severity(
        &self,
        override_index: Option<usize>,
        rule_id: &str,
        built_in_severity: Severity,
    ) -> Option<Severity> {
        let path_rule_settings =
            override_index.map(|index| &self.override_list[index].rule_settings);
        let configured_level = path_rule_settings
            .and_then(|settings| settings.find(rule_id))
            .or_else(|| self.rule_settings.find(rule_id));

        match configured_level {
            Some(level) => level.severity(),
            None => Some(built_in_severity),
        }
    }
}

fn find_config(working_directory: &Path) -> Result<Option<PathBuf>, ConfigError> {
    for directory in working_directory.ancestors() {
        let candidate = directory.join(CONFIG_FILE_NAME);
        match candidate.try_exists() {
            Ok(true) => return Ok(Some(candidate)),
            Ok(false) => {}
            Err(source) => {
                return Err(ConfigError::Search {
                    path: candidate,
                    source,
                });
            }
        }
    }
    Ok(None)
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct ConfigFile {
    #[serde(default)]
    files: FilesConfig,
    #[serde(default)]
    rules: BTreeMap<String, RuleLevel>,
    #[serde(default)]
    exit: ExitConfig,
    #[serde(default)]
    overrides: Vec<OverrideConfig>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct FilesConfig {
    #[serde(default)]
    ignore: Vec<String>,
    #[serde(default)]
    respect_gitignore: bool,
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct ExitConfig {
    fail_on: Option<ConfiguredFailOn>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct OverrideConfig {
    files: Vec<String>,
    #[serde(default)]
    rules: BTreeMap<String, RuleLevel>,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
enum ConfiguredFailOn {
    Error,
    Warning,
    Info,
    None,
}

impl ConfiguredFailOn {
    fn into_fail_on(self) -> FailOn {
        match self {
            Self::Error => FailOn::Error,
            Self::Warning => FailOn::Warning,
            Self::Info => FailOn::Info,
            Self::None => FailOn::None,
        }
    }
}

#[derive(Debug)]
struct PathOverride {
    file_pattern_list: Vec<Pattern>,
    rule_settings: RuleSettings,
}

impl PathOverride {
    fn build(path_override: OverrideConfig, override_number: usize) -> Result<Self, ConfigError> {
        if path_override.files.is_empty() {
            return Err(ConfigError::EmptyOverrideFiles { override_number });
        }
        let file_pattern_list = path_override
            .files
            .into_iter()
            .map(|pattern| {
                Pattern::new(&pattern).map_err(|source| ConfigError::OverridePattern {
                    pattern,
                    override_number,
                    source,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        let rule_settings = RuleSettings::build(
            path_override.rules,
            &format!("[[overrides]] entry {override_number}"),
        )?;
        Ok(Self {
            file_pattern_list,
            rule_settings,
        })
    }

    fn matches(&self, relative_path: &str) -> bool {
        self.file_pattern_list.iter().any(|pattern| {
            pattern.matches_with(
                relative_path,
                MatchOptions {
                    case_sensitive: false,
                    require_literal_separator: true,
                    require_literal_leading_dot: true,
                },
            )
        })
    }
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("cannot determine the working directory: {0}")]
    WorkingDirectory(io::Error),
    #[error("cannot search for configuration at {path}: {source}")]
    Search { path: PathBuf, source: io::Error },
    #[error("cannot read configuration {path}: {source}")]
    Read { path: PathBuf, source: io::Error },
    #[error("cannot parse configuration {path}: {source}")]
    Parse {
        path: PathBuf,
        source: toml::de::Error,
    },
    #[error("invalid ignore pattern '{pattern}': {source}")]
    IgnorePattern {
        pattern: String,
        source: ignore::Error,
    },
    #[error("cannot load ignore file {path}: {source}")]
    IgnoreFile {
        path: PathBuf,
        source: ignore::Error,
    },
    #[error("cannot build ignore matcher: {0}")]
    IgnoreMatcher(ignore::Error),
    #[error("unknown Rule ID '{rule_id}' in {location}")]
    UnknownRule { rule_id: String, location: String },
    #[error("[[overrides]] entry {override_number} must contain at least one file pattern")]
    EmptyOverrideFiles { override_number: usize },
    #[error("invalid file pattern '{pattern}' in [[overrides]] entry {override_number}: {source}")]
    OverridePattern {
        pattern: String,
        override_number: usize,
        source: glob::PatternError,
    },
    #[error("multiple [[overrides]] entries match {path}: {override_number_list:?}")]
    AmbiguousOverrides {
        path: PathBuf,
        override_number_list: Vec<usize>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn finds_configuration_in_parent_directory() {
        let directory = tempdir().unwrap();
        let nested_directory = directory.path().join("nested");
        fs::create_dir(&nested_directory).unwrap();
        fs::write(directory.path().join(CONFIG_FILE_NAME), "").unwrap();

        assert_eq!(
            find_config(&nested_directory).unwrap(),
            Some(directory.path().join(CONFIG_FILE_NAME))
        );
    }

    #[test]
    fn rejects_unknown_rule_id() {
        let error = RuleSettings::build(
            BTreeMap::from([("google/article/unknown".to_owned(), RuleLevel::Off)]),
            "[rules]",
        )
        .unwrap_err();

        assert!(matches!(error, ConfigError::UnknownRule { .. }));
    }

    #[test]
    fn rejects_wildcard_rule_id() {
        let error = RuleSettings::build(
            BTreeMap::from([("google/article/*".to_owned(), RuleLevel::Warning)]),
            "[rules]",
        )
        .unwrap_err();

        assert!(matches!(error, ConfigError::UnknownRule { .. }));
    }

    #[test]
    fn resolves_disabled_and_overridden_rule_severity() {
        let directory = tempdir().unwrap();
        fs::write(
            directory.path().join(CONFIG_FILE_NAME),
            r#"
                [rules]
                "google/article/headline-recommended" = "off"
                "google/article/image-recommended" = "error"
            "#,
        )
        .unwrap();
        let config = LoadedConfig::load_from(directory.path()).unwrap();

        assert_eq!(
            config.resolve_rule_severity(
                None,
                "google/article/headline-recommended",
                Severity::Warning,
            ),
            None
        );
        assert_eq!(
            config.resolve_rule_severity(
                None,
                "google/article/image-recommended",
                Severity::Warning,
            ),
            Some(Severity::Error)
        );
    }

    #[test]
    fn override_file_patterns_are_case_insensitive() {
        let path_override = PathOverride::build(
            OverrideConfig {
                files: vec!["Fixtures/*.HTML".to_owned()],
                rules: BTreeMap::new(),
            },
            1,
        )
        .unwrap();

        assert!(path_override.matches("fixtures/page.html"));
    }

    #[test]
    fn rejects_multiple_matching_overrides() {
        let directory = tempdir().unwrap();
        fs::write(
            directory.path().join(CONFIG_FILE_NAME),
            r#"
                [[overrides]]
                files = ["*.json"]

                [[overrides]]
                files = ["input.json"]
            "#,
        )
        .unwrap();
        let config = LoadedConfig::load_from(directory.path()).unwrap();
        let input_origin = SourceOrigin::Path(directory.path().join("input.json"));

        let error = config.matching_override(&input_origin).unwrap_err();

        assert!(matches!(
            error,
            ConfigError::AmbiguousOverrides {
                override_number_list,
                ..
            } if override_number_list == vec![1, 2]
        ));
    }

    #[test]
    fn applies_configured_ignore_patterns() {
        let directory = tempdir().unwrap();
        fs::write(
            directory.path().join(CONFIG_FILE_NAME),
            r#"
                [files]
                ignore = ["generated/**"]
            "#,
        )
        .unwrap();
        let config = LoadedConfig::load_from(directory.path()).unwrap();

        assert!(config.should_ignore(&directory.path().join("generated/page.html")));
        assert!(!config.should_ignore(&directory.path().join("pages/page.html")));
    }
}
