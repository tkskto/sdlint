use std::io::{self, Read, Write};

use crate::{
    cli::{Cli, FailOn},
    config::LoadedConfig,
    diagnostic::Diagnostic,
    input, lint, parse, report,
};

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

pub fn run(
    cli: &Cli,
    stdin: &mut dyn Read,
    stdout: &mut dyn Write,
    stderr: &mut dyn Write,
    color: bool,
) -> Result<RunOutcome, io::Error> {
    let loaded_config = match LoadedConfig::load() {
        Ok(loaded_config) => loaded_config,
        Err(error) => {
            writeln!(stderr, "sdlint: {error}")?;
            return Ok(RunOutcome::ExecutionError);
        }
    };
    let input_spec_list =
        input::resolve_with_filter(&cli.inputs, &|path| loaded_config.should_ignore(path));
    let mut had_execution_error = false;
    let mut diagnostic_list = Vec::new();
    let source_text_list = input::read_all(input_spec_list, stdin, cli.stdin_format);

    for source_text in source_text_list {
        had_execution_error |=
            process_source_text(source_text, &loaded_config, &mut diagnostic_list, stderr)?;
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

    let fail_on = cli
        .fail_on
        .or(loaded_config.fail_on())
        .unwrap_or(FailOn::Error);
    Ok(if had_execution_error {
        RunOutcome::ExecutionError
    } else if diagnostic_list
        .iter()
        .any(|diagnostic| fail_on.matches(diagnostic.severity))
    {
        RunOutcome::Diagnostics
    } else {
        RunOutcome::Success
    })
}

fn process_source_text(
    source_text: Result<input::SourceText, input::InputError>,
    loaded_config: &LoadedConfig,
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
    let override_index = match loaded_config.matching_override(&source_text.origin) {
        Ok(override_index) => override_index,
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

        let structured_data_diagnostic_list =
            lint::lint_structured_data(&structured_data, &|rule_id, built_in_severity| {
                loaded_config.resolve_rule_severity(override_index, rule_id, built_in_severity)
            });
        diagnostic_list.extend(structured_data_diagnostic_list);
    }

    Ok(had_parse_error)
}
