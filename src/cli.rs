use clap::{Parser, ValueEnum};

use crate::{diagnostic::Severity, input::SourceFormat};

/// Lint schema.org structured data.
#[derive(Debug, Clone, Parser)]
#[command(name = "sdlint", version, about)]
pub struct Cli {
    /// Files, directories, globs, or '-' for standard input.
    #[arg(value_name = "INPUT", required = true)]
    pub inputs: Vec<String>,

    /// Format of standard input.
    #[arg(long, value_enum, default_value_t = SourceFormat::Json)]
    pub stdin_format: SourceFormat,

    /// Output representation.
    #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
    pub format: OutputFormat,

    /// Lowest severity to display.
    #[arg(long, value_enum, default_value_t = Severity::Info)]
    pub severity: Severity,

    /// Lowest severity that makes linting fail; defaults to configured value, then error.
    #[arg(long, value_enum)]
    pub fail_on: Option<FailOn>,

    /// Disable colored output.
    #[arg(long)]
    pub no_color: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum OutputFormat {
    Text,
    Json,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum FailOn {
    Error,
    Warning,
    Info,
    None,
}

impl FailOn {
    pub fn matches(self, severity: Severity) -> bool {
        match self {
            Self::Error => severity == Severity::Error,
            Self::Warning => matches!(severity, Severity::Warning | Severity::Error),
            Self::Info => true,
            Self::None => false,
        }
    }
}
