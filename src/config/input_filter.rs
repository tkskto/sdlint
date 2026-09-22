use std::path::{Path, PathBuf};

use ignore::gitignore::{Gitignore, GitignoreBuilder};

use super::{ConfigError, FilesConfig};

#[derive(Debug)]
pub(super) struct InputFilter {
    working_directory: PathBuf,
    gitignore_matcher: Gitignore,
    configured_pattern_matcher: Gitignore,
}

impl InputFilter {
    pub(super) fn build(
        working_directory: &Path,
        config_directory: &Path,
        files: &FilesConfig,
    ) -> Result<Self, ConfigError> {
        let mut gitignore_builder = GitignoreBuilder::new(config_directory);
        if files.respect_gitignore {
            add_ignore_file(&mut gitignore_builder, &config_directory.join(".gitignore"))?;
        }
        let gitignore_matcher = gitignore_builder
            .build()
            .map_err(ConfigError::IgnoreMatcher)?;

        let mut configured_pattern_builder = GitignoreBuilder::new(working_directory);
        for pattern in &files.ignore {
            configured_pattern_builder
                .add_line(None, pattern)
                .map_err(|source| ConfigError::IgnorePattern {
                    pattern: pattern.clone(),
                    source,
                })?;
        }
        let configured_pattern_matcher = configured_pattern_builder
            .build()
            .map_err(ConfigError::IgnoreMatcher)?;
        Ok(Self {
            working_directory: working_directory.to_path_buf(),
            gitignore_matcher,
            configured_pattern_matcher,
        })
    }

    pub(super) fn excludes(&self, path: &Path) -> bool {
        let absolute_path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            self.working_directory.join(path)
        };
        let is_directory = path.is_dir();
        let mut ignored = false;
        for matcher in [&self.gitignore_matcher, &self.configured_pattern_matcher] {
            if !absolute_path.starts_with(matcher.path()) {
                continue;
            }
            let matched = matcher.matched_path_or_any_parents(&absolute_path, is_directory);
            if matched.is_ignore() {
                ignored = true;
            } else if matched.is_whitelist() {
                ignored = false;
            }
        }
        ignored
    }
}

fn add_ignore_file(builder: &mut GitignoreBuilder, path: &Path) -> Result<(), ConfigError> {
    if path.is_file() {
        if let Some(source) = builder.add(path) {
            return Err(ConfigError::IgnoreFile {
                path: path.to_path_buf(),
                source,
            });
        }
    }
    Ok(())
}
