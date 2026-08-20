use std::{io, io::IsTerminal, process::ExitCode};

use clap::Parser;
use sdlint::{RunOutcome, cli::Cli, run};

fn main() -> ExitCode {
    let cli = Cli::parse();
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();
    let mut stderr = io::stderr().lock();
    let color = stdout.is_terminal();
    let outcome = run(&cli, &mut stdin, &mut stdout, &mut stderr, color);
    let code = match outcome {
        Ok(outcome) => outcome.exit_code(),
        Err(error) => {
            eprintln!("sdlint: failed to write output: {error}");
            RunOutcome::ExecutionError.exit_code()
        }
    };
    ExitCode::from(code)
}
