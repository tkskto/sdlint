use std::io::{self, Read, Write};

use crate::{cli::Cli, diagnostic::Diagnostic, input, lint, parse, report};

/// The outcome of a library run. The CLI maps this value to a process exit code.
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
) -> Result<RunOutcome, io::Error> {
    let input_spec_list = input::resolve(&cli.inputs);
    let mut had_execution_error = false;
    let mut diagnostic_list = Vec::new();
    let source_text_list = input::read_all(input_spec_list, stdin);

    for source_text in source_text_list {
        had_execution_error |= process_source_text(source_text, &mut diagnostic_list, stderr)?;
    }

    let visible_diagnostic_list = diagnostic_list
        .iter()
        .filter(|diagnostic| diagnostic.severity >= cli.severity)
        .cloned()
        .collect::<Vec<Diagnostic>>();

    report::write(
        &visible_diagnostic_list,
        cli.format,
        color && !cli.no_color,
        stdout,
    )?;

    Ok(if had_execution_error {
        RunOutcome::ExecutionError
    } else if diagnostic_list
        .iter()
        .any(|diagnostic| cli.fail_on.matches(diagnostic.severity))
    {
        RunOutcome::Diagnostics
    } else {
        RunOutcome::Success
    })
}

fn process_source_text(
    source_text: Result<input::SourceText, input::InputError>,
    diagnostic_list: &mut Vec<Diagnostic>,
    stderr: &mut dyn Write,
) -> io::Result<bool> {
    let source_text = match source_text {
        Ok(source_text) => source_text,
        Err(error) => {
            writeln!(stderr, "sdlint: {error}")?;
            return Ok(true);
        }
    };

    let mut had_parse_error = false;
    for structured_data in parse::parse_source_text(&source_text) {
        let structured_data = match structured_data {
            Ok(structured_data) => structured_data,
            Err(error) => {
                writeln!(stderr, "sdlint: {error}")?;
                had_parse_error = true;
                continue;
            }
        };

        diagnostic_list.extend(lint::lint_structured_data(&structured_data));
    }

    Ok(had_parse_error)
}
