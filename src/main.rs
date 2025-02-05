use clap::{value_parser, ArgAction, CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};
use devx::{CliResult, Command, GitCommands, ManageCommands};
use std::io;

#[derive(Parser)]
#[command(author, version, about = "cli for automating dev workflows")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,

    #[arg(long = "generate", action = ArgAction::Set, value_parser = value_parser!(Shell))]
    generator: Option<Shell>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "manage devx configuration")]
    Manage {
        #[clap(subcommand)]
        command: Option<ManageCommands>,
    },
    #[command(about = "run git workflows")]
    Git {
        #[clap(subcommand)]
        command: Option<GitCommands>,
    },
}

// prints help page for invalid subcommands
fn handle_invalid_subcommand(subcommand: &str) -> ! {
    let mut cmd = Cli::command();

    // build is required to print help of subcommands.
    // see https://github.com/clap-rs/clap/issues/4685
    cmd.build();

    let _ = cmd.find_subcommand_mut(subcommand).unwrap().print_help();
    std::process::exit(0);
}

// generates completions script for input shell
fn print_completions(generator: Shell, cmd: &mut clap::Command) {
    generate(
        generator,
        cmd,
        cmd.get_name().to_string(),
        &mut io::stdout(),
    );
}

fn main() -> CliResult<()> {
    let cli = Cli::parse();

    // define `--generate` flag
    if let Some(generator) = cli.generator {
        let mut cmd = Cli::command();
        print_completions(generator, &mut cmd);
        return Ok(());
    }

    // match commands and subcommands
    match cli.command {
        Some(Commands::Manage { command }) => {
            command.map_or_else(|| handle_invalid_subcommand("manage"), |cmd| cmd.execute())?
        }
        Some(Commands::Git { command }) => {
            command.map_or_else(|| handle_invalid_subcommand("git"), |cmd| cmd.execute())?
        }
        _ => {
            let _ = Cli::command().print_help();
            std::process::exit(0);
        }
    }

    Ok(())
}
