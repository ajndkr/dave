use clap::{CommandFactory, Parser, Subcommand};
use devx::{choose, CliResult, Command, ManageCommands};

#[derive(Parser)]
#[command(author, version, about = "cli for automating dev workflows")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "choose a flavor")]
    Choose {},
    #[command(about = "manage devx configuration")]
    Manage {
        #[clap(subcommand)]
        command: Option<ManageCommands>,
    },
}

fn handle_invalid_subcommand(subcommand: &str) -> ! {
    let mut cmd = Cli::command();

    // build is required to print help of subcommands.
    // see https://github.com/clap-rs/clap/issues/4685
    cmd.build();

    let _ = cmd.find_subcommand_mut(subcommand).unwrap().print_help();
    std::process::exit(0);
}

fn main() -> CliResult<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Choose {}) => choose()?,
        Some(Commands::Manage { command }) => {
            command.map_or_else(|| handle_invalid_subcommand("manage"), |cmd| cmd.execute())?
        }
        None => {
            let _ = Cli::command().print_help();
            std::process::exit(0);
        }
    }

    Ok(())
}
