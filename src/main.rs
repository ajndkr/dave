use clap::{CommandFactory, Parser, Subcommand};
use inquire::Select;

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

fn choose_command() -> Result<(), Box<dyn std::error::Error>> {
    let choices = vec!["apple", "banana", "cherry"];
    let choice = Select::new("pick a flavor:", choices).prompt()?;

    println!("selected choice: {}", choice);
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
