//! Deterministic expansion and acquisition of command-line inputs.

use std::{
    collections::HashSet,
    fmt, fs,
    io::Read,
    path::{Path, PathBuf},
};

use thiserror::Error;
use walkdir::WalkDir;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputSpec {
    File(PathBuf),
    Stdin,
    Error(InputError),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SourceId {
    Path(PathBuf),
    Stdin,
}

impl fmt::Display for SourceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Path(path) => write!(f, "{}", path.display()),
            Self::Stdin => f.write_str("<stdin>"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceDocument {
    pub source: SourceId,
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
    #[error("standard input was specified more than once")]
    DuplicateStdin,
}

/// Expands operands in their given order. An empty operand list means stdin.
pub fn resolve(operands: &[String]) -> Vec<InputSpec> {
    let defaults;
    let operands = if operands.is_empty() {
        defaults = vec!["-".to_owned()];
        &defaults
    } else {
        operands
    };
    let mut output = Vec::new();
    let mut paths = HashSet::new();
    let mut saw_stdin = false;

    for operand in operands {
        if operand == "-" {
            if saw_stdin {
                output.push(InputSpec::Error(InputError::DuplicateStdin));
            } else {
                saw_stdin = true;
                output.push(InputSpec::Stdin);
            }
            continue;
        }

        if is_glob(operand) {
            expand_glob(operand, &mut paths, &mut output);
        } else {
            let _ = expand_path(Path::new(operand), &mut paths, &mut output);
        }
    }
    output
}

fn is_glob(value: &str) -> bool {
    value.bytes().any(|byte| matches!(byte, b'*' | b'?' | b'['))
}

fn expand_glob(pattern: &str, seen: &mut HashSet<PathBuf>, output: &mut Vec<InputSpec>) {
    let entries = match glob::glob(pattern) {
        Ok(entries) => entries,
        Err(error) => {
            output.push(InputSpec::Error(InputError::InvalidGlob {
                pattern: pattern.to_owned(),
                message: error.to_string(),
            }));
            return;
        }
    };
    let mut matches = entries.filter_map(Result::ok).collect::<Vec<_>>();
    matches.sort_by_key(|a| normalized(a));
    let mut matched_supported = false;
    for path in matches {
        matched_supported |= expand_supported_path(&path, seen, output);
    }
    if !matched_supported {
        output.push(InputSpec::Error(InputError::GlobNoMatch {
            pattern: pattern.to_owned(),
        }));
    }
}

fn expand_path(path: &Path, seen: &mut HashSet<PathBuf>, output: &mut Vec<InputSpec>) -> bool {
    if path.is_dir() {
        let mut files = WalkDir::new(path)
            .follow_links(false)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().is_file() && supported(entry.path()))
            .map(|entry| entry.into_path())
            .collect::<Vec<_>>();
        files.sort_by_key(|a| normalized(a));
        let matched = !files.is_empty();
        for file in files {
            add_file(file, seen, output);
        }
        matched
    } else {
        // Explicit files are retained even with unsupported extensions; later
        // stages can give an appropriate format error. Acquisition still reports
        // missing and unreadable paths as typed read errors.
        add_file(path.to_path_buf(), seen, output);
        true
    }
}

fn expand_supported_path(
    path: &Path,
    seen: &mut HashSet<PathBuf>,
    output: &mut Vec<InputSpec>,
) -> bool {
    if path.is_dir() {
        expand_path(path, seen, output)
    } else if supported(path) {
        add_file(path.to_path_buf(), seen, output);
        true
    } else {
        false
    }
}

fn add_file(path: PathBuf, seen: &mut HashSet<PathBuf>, output: &mut Vec<InputSpec>) {
    let key = fs::canonicalize(&path).unwrap_or_else(|_| path.clone());
    if seen.insert(key) {
        output.push(InputSpec::File(path));
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

fn normalized(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

pub fn read_file(path: &Path) -> Result<SourceDocument, InputError> {
    let text = fs::read_to_string(path).map_err(|error| InputError::Read {
        path: path.to_path_buf(),
        message: error.to_string(),
    })?;
    Ok(SourceDocument {
        source: SourceId::Path(path.to_path_buf()),
        text: text.strip_prefix('\u{feff}').unwrap_or(&text).to_owned(),
    })
}

pub fn read_stdin(reader: &mut dyn Read) -> Result<SourceDocument, InputError> {
    let mut text = String::new();
    reader
        .read_to_string(&mut text)
        .map_err(|error| InputError::Read {
            path: PathBuf::from("<stdin>"),
            message: error.to_string(),
        })?;
    Ok(SourceDocument {
        source: SourceId::Stdin,
        text: text.strip_prefix('\u{feff}').unwrap_or(&text).to_owned(),
    })
}

pub fn read_all(
    specs: Vec<InputSpec>,
    stdin: &mut dyn Read,
) -> Vec<Result<SourceDocument, InputError>> {
    specs
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
    fn defaults_to_stdin_and_rejects_a_duplicate() {
        assert_eq!(resolve(&[]), vec![InputSpec::Stdin]);
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
        let result = resolve(&[
            pattern,
            directory.path().join("a.json").to_string_lossy().into(),
        ]);
        assert_eq!(result.len(), 2);
        assert!(matches!(&result[0], InputSpec::File(path) if path.ends_with("a.json")));
        assert!(matches!(&result[1], InputSpec::File(path) if path.ends_with("b.json")));
    }

    #[test]
    fn unmatched_glob_is_typed() {
        assert!(matches!(
            resolve(&["definitely-missing/*.json".into()]).as_slice(),
            [InputSpec::Error(InputError::GlobNoMatch { .. })]
        ));
    }

    #[test]
    fn readers_remove_a_utf8_bom() {
        let document = read_stdin(&mut "\u{feff}{}".as_bytes()).unwrap();
        assert_eq!(document.text, "{}");
        assert_eq!(document.source, SourceId::Stdin);
    }
}
