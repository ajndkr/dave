use crate::{CliResult, Command};
use clap::Subcommand;
use std;
use which::which;

// define subcommands for 'git' command
#[derive(Subcommand)]
pub enum GitCommands {
    #[command(about = "sync latest changes from remote branch")]
    Sync {},
}

// map 'git' subcommands to functions
impl Command for GitCommands {
    fn execute(&self) -> CliResult<()> {
        match self {
            GitCommands::Sync {} => sync(),
        }
    }
}

// sync latest changes from remote branch.
//
// workflow:
//  stage local -> fetch remote -> stash local -> pull changes -> restore stash
//
// errors:
// - CliError::Command: if the binary file cannot be found
// - CliError::IOError: if the binary file cannot be removed
pub fn sync() -> CliResult<()> {
    println!("running git sync workflow...");

    // check if git is installed
    which("git").expect("git not found. install git first and try again.");

    println!("staging local changes...");
    std::process::Command::new("git")
        .arg("add")
        .arg(".")
        .output()
        .expect("failed to stage local changes");

    println!("fetching remote changes...");
    std::process::Command::new("git")
        .arg("fetch")
        .arg("-p")
        .output()
        .expect("failed to fetch remote changes");

    println!("stashing local changes...");
    std::process::Command::new("git")
        .arg("stash")
        .output()
        .expect("failed to stash local changes");

    println!("pulling remote changes...");
    std::process::Command::new("git")
        .arg("pull")
        .arg("--rebase")
        .output()
        .expect("failed to pull remote changes");

    println!("restoring local changes...");
    std::process::Command::new("git")
        .arg("stash")
        .arg("pop")
        .output()
        .expect("failed to restore local changes");

    std::process::Command::new("git")
        .arg("stash")
        .arg("clear")
        .output()
        .expect("failed to clear stash");

    println!("git sync complete! latest commit:");

    let git_log_output = std::process::Command::new("git")
        .arg("log")
        .arg("-1")
        .output()
        .expect("failed to get latest commit");

    let latest_commit = String::from_utf8_lossy(&git_log_output.stdout)
        .trim()
        .to_string();

    println!("{}", latest_commit);

    Ok(())
}
