use clap::{CommandFactory, Parser, Subcommand};
use devx::{commands::choose::choose_command, CliResult};

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
}

fn main() -> CliResult<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Choose {}) => choose_command()?,
        None => {
            let _ = Cli::command().print_help();
            std::process::exit(0);
        }
    }

    Ok(())
}
