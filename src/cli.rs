use std::path::PathBuf;

use clap::{Parser, ValueEnum};

/// Lint schema.org structured data.
#[derive(Debug, Clone, Parser)]
#[command(name = "sdlint", version, about)]
pub struct Cli {
    /// Files, directories, globs, or '-' for standard input.
    #[arg(value_name = "INPUT")]
    pub inputs: Vec<String>,

    /// Output representation.
    #[arg(long, value_enum, default_value_t = OutputFormat::Human)]
    pub format: OutputFormat,

    /// Lowest severity to display.
    #[arg(long, value_enum, default_value_t = Severity::Info)]
    pub severity: Severity,

    /// Lowest severity that makes linting fail.
    #[arg(long, value_enum, default_value_t = FailOn::Error)]
    pub fail_on: FailOn,

    /// Ruleset configuration file to load.
    #[arg(long, value_name = "FILE")]
    pub ruleset: Option<PathBuf>,

    /// Disable colored output.
    #[arg(long)]
    pub no_color: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum OutputFormat {
    Human,
    Json,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Severity {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum FailOn {
    Error,
    Warning,
    Info,
    None,
}
