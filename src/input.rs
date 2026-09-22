//! Deterministic expansion and acquisition of command-line inputs.
use std::{
    collections::HashSet,
    ffi::OsStr,
    fmt, fs,
    io::Read,
    path::{Path, PathBuf},
};

use thiserror::Error;
use walkdir::WalkDir;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputSpec {
    // Req. URLを渡せるように拡張できるか
    File(PathBuf),
    Stdin,
    Error(InputError),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SourceOrigin {
    Path(PathBuf),
    Stdin,
}

impl fmt::Display for SourceOrigin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Path(path) => write!(f, "{}", path.display()),
            Self::Stdin => f.write_str("<stdin>"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceText {
    pub origin: SourceOrigin,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum InputError {
    #[error("cannot read {path}: {message}")]
    Read { path: PathBuf, message: String },
    #[error("glob '{pattern}' matched no supported files")]
    GlobNoMatch { pattern: String },
    #[error("invalid glob '{pattern}': {message}")]
    InvalidGlob { pattern: String, message: String },
    #[error("unsupported input file: {path}")]
    UnsupportedFile { path: PathBuf },
    #[error("standard input was specified more than once")]
    DuplicateStdin,
}

/// Expands operands in their given order.
#[cfg(test)]
pub fn resolve(input_operand_list: &[String]) -> Vec<InputSpec> {
    resolve_with_filter(input_operand_list, &|_| false)
}

pub fn resolve_with_filter(
    input_operand_list: &[String],
    should_exclude: &dyn Fn(&Path) -> bool,
) -> Vec<InputSpec> {
    let mut input_spec_list = Vec::new();
    let mut seen_path_set = HashSet::new();
    let mut stdin_was_specified = false;

    for operand in input_operand_list {
        if operand == "-" {
            if stdin_was_specified {
                input_spec_list.push(InputSpec::Error(InputError::DuplicateStdin));
            } else {
                stdin_was_specified = true;
                input_spec_list.push(InputSpec::Stdin);
            }
            continue;
        }

        if is_glob(operand) {
            expand_glob(
                operand,
                &mut seen_path_set,
                &mut input_spec_list,
                should_exclude,
            );
        } else {
            expand_explicit_path(
                Path::new(operand),
                &mut seen_path_set,
                &mut input_spec_list,
                should_exclude,
            );
        }
    }
    input_spec_list
}

fn is_glob(value: &str) -> bool {
    value.bytes().any(|byte| matches!(byte, b'*' | b'?' | b'['))
}

fn expand_glob(
    pattern: &str,
    seen_path_set: &mut HashSet<PathBuf>,
    input_spec_list: &mut Vec<InputSpec>,
    should_exclude: &dyn Fn(&Path) -> bool,
) {
    let entries = match glob::glob(pattern) {
        Ok(entries) => entries,
        Err(error) => {
            input_spec_list.push(InputSpec::Error(InputError::InvalidGlob {
                pattern: pattern.to_owned(),
                message: error.to_string(),
            }));
            return;
        }
    };
    let mut glob_match_list = entries.filter_map(Result::ok).collect::<Vec<_>>();
    glob_match_list.sort();
    let mut found_supported_file = false;
    for path in glob_match_list {
        found_supported_file |=
            expand_glob_match(&path, seen_path_set, input_spec_list, should_exclude);
    }
    if !found_supported_file {
        input_spec_list.push(InputSpec::Error(InputError::GlobNoMatch {
            pattern: pattern.to_owned(),
        }));
    }
}

fn expand_explicit_path(
    path: &Path,
    seen_path_set: &mut HashSet<PathBuf>,
    input_spec_list: &mut Vec<InputSpec>,
    should_exclude: &dyn Fn(&Path) -> bool,
) {
    if path.is_dir() {
        collect_directory_files(path, seen_path_set, input_spec_list, should_exclude);
    } else if path.exists() && !supported(path) {
        input_spec_list.push(InputSpec::Error(InputError::UnsupportedFile {
            path: path.to_path_buf(),
        }));
    } else if !should_exclude(path) {
        add_file(path.to_path_buf(), seen_path_set, input_spec_list);
    }
}

fn expand_glob_match(
    path: &Path,
    seen_path_set: &mut HashSet<PathBuf>,
    input_spec_list: &mut Vec<InputSpec>,
    should_exclude: &dyn Fn(&Path) -> bool,
) -> bool {
    if is_standard_discovery_excluded(path) {
        return true;
    }
    if path.is_dir() {
        collect_directory_files(path, seen_path_set, input_spec_list, should_exclude)
    } else if supported(path) {
        if !should_exclude(path) {
            add_file(path.to_path_buf(), seen_path_set, input_spec_list);
        }
        true
    } else {
        false
    }
}

fn collect_directory_files(
    path: &Path,
    seen_path_set: &mut HashSet<PathBuf>,
    input_spec_list: &mut Vec<InputSpec>,
    should_exclude: &dyn Fn(&Path) -> bool,
) -> bool {
    let mut supported_file_list = WalkDir::new(path)
        .follow_links(false)
        .into_iter()
        .filter_entry(|entry| entry.depth() == 0 || !is_standard_discovery_excluded(entry.path()))
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file() && supported(entry.path()))
        .map(|entry| entry.into_path())
        .collect::<Vec<_>>();
    supported_file_list.sort();
    let found_supported_file = !supported_file_list.is_empty();
    for file in supported_file_list {
        if !should_exclude(&file) {
            add_file(file, seen_path_set, input_spec_list);
        }
    }
    found_supported_file
}

fn is_standard_discovery_excluded(path: &Path) -> bool {
    path.components()
        .any(|component| component.as_os_str() == OsStr::new("node_modules"))
}

fn add_file(
    path: PathBuf,
    seen_path_set: &mut HashSet<PathBuf>,
    input_spec_list: &mut Vec<InputSpec>,
) {
    let key = fs::canonicalize(&path).unwrap_or_else(|_| path.clone());
    if seen_path_set.insert(key) {
        input_spec_list.push(InputSpec::File(path));
    }
}

fn supported(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| {
            matches!(
                extension.to_ascii_lowercase().as_str(),
                "html" | "htm" | "json" | "jsonld" | "json-ld"
            )
        })
}

pub fn read_file(path: &Path) -> Result<SourceText, InputError> {
    let text = fs::read_to_string(path).map_err(|error| InputError::Read {
        path: path.to_path_buf(),
        message: error.to_string(),
    })?;
    Ok(SourceText {
        origin: SourceOrigin::Path(path.to_path_buf()),
        text: text.strip_prefix('\u{feff}').unwrap_or(&text).to_owned(),
    })
}

pub fn read_stdin(reader: &mut dyn Read) -> Result<SourceText, InputError> {
    let mut text = String::new();
    reader
        .read_to_string(&mut text)
        .map_err(|error| InputError::Read {
            path: PathBuf::from("<stdin>"),
            message: error.to_string(),
        })?;
    Ok(SourceText {
        origin: SourceOrigin::Stdin,
        text: text.strip_prefix('\u{feff}').unwrap_or(&text).to_owned(),
    })
}

pub fn read_all(
    input_spec_list: Vec<InputSpec>,
    stdin: &mut dyn Read,
) -> Vec<Result<SourceText, InputError>> {
    input_spec_list
        .into_iter()
        .map(|spec| match spec {
            InputSpec::File(path) => read_file(&path),
            InputSpec::Stdin => read_stdin(stdin),
            InputSpec::Error(error) => Err(error),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn does_not_default_to_stdin_and_rejects_duplicate_stdin() {
        assert!(resolve(&[]).is_empty());
        assert_eq!(
            resolve(&["-".into(), "-".into()]),
            vec![
                InputSpec::Stdin,
                InputSpec::Error(InputError::DuplicateStdin)
            ]
        );
    }

    #[test]
    fn glob_is_sorted_and_deduplicated() {
        let directory = tempdir().unwrap();
        fs::write(directory.path().join("b.json"), "b").unwrap();
        fs::write(directory.path().join("a.json"), "a").unwrap();
        let pattern = format!("{}/*.json", directory.path().display());
        let expanded_inputs = resolve(&[
            pattern,
            directory.path().join("a.json").to_string_lossy().into(),
        ]);
        assert_eq!(expanded_inputs.len(), 2);
        assert!(matches!(&expanded_inputs[0], InputSpec::File(path) if path.ends_with("a.json")));
        assert!(matches!(&expanded_inputs[1], InputSpec::File(path) if path.ends_with("b.json")));
    }

    #[test]
    fn directory_discovery_skips_node_modules() {
        let directory = tempdir().unwrap();
        let node_modules_directory = directory.path().join("node_modules/package");
        let target_directory = directory.path().join("target");
        fs::create_dir_all(&node_modules_directory).unwrap();
        fs::create_dir(&target_directory).unwrap();
        fs::write(directory.path().join("page.json"), "{}").unwrap();
        fs::write(node_modules_directory.join("package.json"), "{}").unwrap();
        fs::write(target_directory.join("generated.json"), "{}").unwrap();

        let expanded_inputs = resolve(&[directory.path().to_string_lossy().into()]);

        assert_eq!(expanded_inputs.len(), 2);
        assert!(matches!(
            &expanded_inputs[0],
            InputSpec::File(path) if path.ends_with("page.json")
        ));
        assert!(matches!(
            &expanded_inputs[1],
            InputSpec::File(path) if path.ends_with("target/generated.json")
        ));
    }

    #[test]
    fn glob_discovery_skips_node_modules() {
        let directory = tempdir().unwrap();
        let node_modules_directory = directory.path().join("node_modules/package");
        fs::create_dir_all(&node_modules_directory).unwrap();
        fs::write(directory.path().join("page.json"), "{}").unwrap();
        fs::write(node_modules_directory.join("package.json"), "{}").unwrap();
        let pattern = format!("{}/**/*.json", directory.path().display());

        let expanded_inputs = resolve(&[pattern]);

        assert_eq!(expanded_inputs.len(), 1);
        assert!(matches!(
            &expanded_inputs[0],
            InputSpec::File(path) if path.ends_with("page.json")
        ));
    }

    #[test]
    fn explicitly_named_node_modules_file_is_included() {
        let directory = tempdir().unwrap();
        let node_modules_directory = directory.path().join("node_modules/package");
        fs::create_dir_all(&node_modules_directory).unwrap();
        let path = node_modules_directory.join("package.json");
        fs::write(&path, "{}").unwrap();

        assert_eq!(
            resolve(&[path.to_string_lossy().into()]),
            vec![InputSpec::File(path)]
        );
    }

    #[test]
    fn unmatched_glob_is_typed() {
        assert!(matches!(
            resolve(&["definitely-missing/*.json".into()]).as_slice(),
            [InputSpec::Error(InputError::GlobNoMatch { .. })]
        ));
    }

    #[test]
    fn explicitly_specified_unsupported_file_is_typed() {
        let directory = tempdir().unwrap();
        let path = directory.path().join("input.txt");
        fs::write(&path, "input").unwrap();

        assert_eq!(
            resolve(&[path.to_string_lossy().into()]),
            vec![InputSpec::Error(InputError::UnsupportedFile { path })]
        );
    }

    #[test]
    fn missing_supported_file_remains_a_read_error() {
        let directory = tempdir().unwrap();
        let path = directory.path().join("missing.json");

        let source_text_list = read_all(resolve(&[path.to_string_lossy().into()]), &mut &b""[..]);

        assert!(matches!(
            source_text_list.as_slice(),
            [Err(InputError::Read {path: error_path, .. })] if error_path == &path
        ));
    }

    #[test]
    fn readers_remove_a_utf8_bom() {
        let source_text = read_stdin(&mut "\u{feff}{}".as_bytes()).unwrap();
        assert_eq!(source_text.text, "{}");
        assert_eq!(source_text.origin, SourceOrigin::Stdin);
    }
}
