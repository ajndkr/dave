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

fn invalid_subcommand(subcommand: &str) -> ! {
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
        Some(Commands::Manage { command }) => match command {
            Some(ManageCommands::Where {}) => whereis()?,
            Some(ManageCommands::Uninstall {}) => uninstall()?,
            None => invalid_subcommand("manage"),
        },
        None => {
            let _ = Cli::command().print_help();
            std::process::exit(0);
        }
    }

    Ok(())
}
