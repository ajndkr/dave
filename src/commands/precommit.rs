use crate::{CliResult, Command};
use clap::Subcommand;
use colored::Colorize;
use which::which;

// define subcommands for 'pre-commit' command
#[derive(Subcommand)]
pub enum PreCommitCommands {
    #[command(about = "run pre-commit")]
    Run {},
}

// map 'pre-commit' subcommands to functions
impl Command for PreCommitCommands {
    fn execute(&self) -> CliResult<()> {
        match self {
            PreCommitCommands::Run {} => run(),
        }
    }
}

// run pre-commit
//
// errors:
// - CliError::Command: if the pre-commit command fails
pub fn run() -> CliResult<()> {
    which("pre-commit").expect("pre-commit not found. install pre-commit and try again.");

    println!("{}", "oops. still under construction ^.^".bold());
    Ok(())
}
