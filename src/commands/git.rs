use crate::{CliResult, Command};
use clap::Subcommand;
use std::process;
use which::which;

use crate::CliError;

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

// run git command with arguments
//
// errors:
// - CliError::Command: if the git command fails
fn run_git_command(args: &[&str], error_msg: &str) -> Result<(), CliError> {
    process::Command::new("git")
        .args(args)
        .stdout(process::Stdio::inherit())
        .stderr(process::Stdio::inherit())
        .status()
        .map_err(|e| CliError::Command(format!("{}: {}", error_msg, e)))
        .map(|_| ())
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
    println!("🔄 running git sync workflow.");

    which("git").expect("❌ git not found. install git first and try again.");

    println!("📝 staging local changes.");
    run_git_command(&["add", "."], "failed to stage local changes")?;

    println!("🌐 fetching remote changes.");
    run_git_command(&["fetch", "-p"], "failed to fetch remote changes")?;

    println!("📦 stashing local changes.");
    run_git_command(&["stash"], "failed to stash local changes")?;

    println!("⬇️ pulling remote changes.");
    run_git_command(&["pull", "--rebase"], "failed to pull remote changes")?;

    println!("📤 restoring local changes.");
    run_git_command(&["stash", "pop"], "failed to restore local changes")?;
    run_git_command(&["stash", "clear"], "failed to clear stash")?;

    println!("✨ git sync complete! latest commit:");

    let git_log_output = process::Command::new("git")
        .arg("log")
        .arg("-1")
        .output()
        .expect("❌ failed to get latest commit");

    let latest_commit = String::from_utf8_lossy(&git_log_output.stdout)
        .trim()
        .to_string();

    println!("📜 {}", latest_commit);

    Ok(())
}
