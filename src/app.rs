use std::io::{self, Read, Write};

use crate::{cli::Cli, diagnostic::Diagnostic, input, lint, parse, report};

/// The result of a library run. The CLI maps this value to a process exit code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunOutcome {
    Success,
    Diagnostics,
    ExecutionError,
}

impl RunOutcome {
    pub const fn exit_code(self) -> u8 {
        match self {
            Self::Success => 0,
            Self::Diagnostics => 1,
            Self::ExecutionError => 2,
        }
    }
}

/// Acquires, parses, lints, and reports all requested inputs.
pub fn run(
    cli: &Cli,
    stdin: &mut dyn Read,
    stdout: &mut dyn Write,
    stderr: &mut dyn Write,
    color: bool,
) -> io::Result<RunOutcome> {
    let specs = input::resolve(&cli.inputs);
    let mut had_execution_error = false;
    let mut diagnostics = Vec::new();

    for result in input::read_all(specs, stdin) {
        had_execution_error |= process_input(result, &mut diagnostics, stderr)?;
    }

    let visible = diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.severity >= cli.severity)
        .cloned()
        .collect::<Vec<Diagnostic>>();
    report::write(&visible, cli.format, color && !cli.no_color, stdout)?;

    Ok(if had_execution_error {
        RunOutcome::ExecutionError
    } else if diagnostics
        .iter()
        .any(|diagnostic| cli.fail_on.matches(diagnostic.severity))
    {
        RunOutcome::Diagnostics
    } else {
        RunOutcome::Success
    })
}

fn process_input(
    result: Result<input::SourceDocument, input::InputError>,
    diagnostics: &mut Vec<Diagnostic>,
    stderr: &mut dyn Write,
) -> io::Result<bool> {
    let document = match result {
        Ok(document) => document,
        Err(error) => {
            writeln!(stderr, "sdlint: {error}")?;
            return Ok(true);
        }
    };

    let parsed_documents = match parse::parse(&document) {
        Ok(parsed_documents) => parsed_documents,
        Err(error) => {
            writeln!(stderr, "sdlint: {error}")?;
            return Ok(true);
        }
    };

    diagnostics.extend(parsed_documents.iter().flat_map(lint::lint));
    Ok(false)
}
