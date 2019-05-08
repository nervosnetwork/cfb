#[macro_use]
extern crate clap;

mod cli_options;
mod exit_code;

use cli_options::CliOptions;
use exit_code::ExitCode;

fn run_app() -> Result<(), ExitCode> {
    CliOptions::parse()?;
    Ok(())
}

fn main() {
    if let Some(exit_code) = run_app().err() {
        exit_code.exit();
    }
}
