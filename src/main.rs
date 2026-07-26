use std::{io, process::ExitCode};

use clap::Parser;
use sdlint::{cli::Cli, run, RunOutcome};

fn main() -> ExitCode {
    let cli = Cli::parse();
    let outcome = run(&cli, &mut io::stdin().lock(), &mut io::stderr().lock());
    let code = match outcome {
        Ok(outcome) => outcome.exit_code(),
        Err(error) => {
            eprintln!("sdlint: failed to write output: {error}");
            RunOutcome::ExecutionError.exit_code()
        }
    };
    ExitCode::from(code)
}
