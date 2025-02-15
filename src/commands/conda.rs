use crate::{CliResult, Command};
use clap::Subcommand;
use colored::Colorize;
use which::which;

// define subcommands for 'conda' command
#[derive(Subcommand)]
pub enum CondaCommands {
    #[command(about = "create conda environment")]
    Create {},
    #[command(about = "activate conda environment")]
    Activate {},
    #[command(about = "delete conda environment(s)")]
    Delete {},
}

// map 'conda' subcommands to functions
impl Command for CondaCommands {
    fn execute(&self) -> CliResult<()> {
        match self {
            CondaCommands::Create {} => create(),
            CondaCommands::Activate {} => activate(),
            CondaCommands::Delete {} => delete(),
        }
    }
}

// create conda environment
//
// errors:
// - CliError::Command: if the conda command fails
pub fn create() -> CliResult<()> {
    which("conda").expect("conda not found. install conda and try again.");

    println!("{}", "oops. still under construction ^.^".bold());
    Ok(())
}

// activate conda environment
//
// errors:
// - CliError::Command: if the conda command fails
pub fn activate() -> CliResult<()> {
    which("conda").expect("conda not found. install conda and try again.");

    println!("{}", "oops. still under construction ^.^".bold());
    Ok(())
}

// delete conda environment(s)
//
// errors:
// - CliError::Command: if the conda command fails
pub fn delete() -> CliResult<()> {
    which("conda").expect("conda not found. install conda and try again.");

    println!("{}", "oops. still under construction ^.^".bold());
    Ok(())
}
