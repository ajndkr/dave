use clap::{CommandFactory, Parser, Subcommand};
use std::process::{Command, Stdio};
use which::which;

#[derive(Parser)]
#[command(author, version, about = "cli for automating dev workflows")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "choose a gum flavor")]
    Gum {},
}

fn gum_command() -> Result<(), Box<dyn std::error::Error>> {
    // check if gum exists, if not: panic!
    which("gum").expect("gum not found. Please install gum first.");

    // spawn new process, required due to tty interaction
    let gum_proc = Command::new("gum")
        .args(["choose", "apple", "banana", "cherry"])
        .stdout(Stdio::piped())
        .spawn()?;

    // wait for process to finish and get output
    let gum_output = gum_proc.wait_with_output()?;

    // parse output
    let gum_choice = String::from_utf8_lossy(&gum_output.stdout)
        .trim()
        .to_string();
    println!("selected choice: {}", gum_choice);

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Gum {}) => gum_command()?,
        None => {
            let _ = Cli::command().print_help();
            std::process::exit(0);
        }
    }

    Ok(())
}
