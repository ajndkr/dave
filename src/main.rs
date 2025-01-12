use clap::{CommandFactory, Parser, Subcommand};
use devx::{choose, uninstall, whereis, CliResult};

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
    #[command(about = "self operations")]
    Manage {
        #[clap(subcommand)]
        command: Option<ManageCommands>,
    },
}

#[derive(Subcommand)]
enum ManageCommands {
    #[command(about = "print location of devx binary")]
    Where {},
    #[command(about = "uninstall devx")]
    Uninstall {},
}

fn invalid_command() -> ! {
    let _ = Cli::command().print_help();
    std::process::exit(0);
}

fn main() -> CliResult<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Choose {}) => choose()?,
        Some(Commands::Manage { command }) => match command {
            Some(ManageCommands::Where {}) => whereis()?,
            Some(ManageCommands::Uninstall {}) => uninstall()?,
            None => invalid_command(),
        },
        None => invalid_command(),
    }

    Ok(())
}
