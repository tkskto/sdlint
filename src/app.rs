use std::io::{self, Read, Write};

use crate::{cli::Cli, input};

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

/// Acquires all requested inputs without terminating the process.
pub fn run(cli: &Cli, stdin: &mut dyn Read, stderr: &mut dyn Write) -> io::Result<RunOutcome> {
    let specs = input::resolve(&cli.inputs);
    let mut had_error = false;

    for result in input::read_all(specs, stdin) {
        if let Err(error) = result {
            had_error = true;
            writeln!(stderr, "sdlint: {error}")?;
        }
    }

    Ok(if had_error {
        RunOutcome::ExecutionError
    } else {
        RunOutcome::Success
    })
}
