use crate::error::CliResult;

use inquire::Select;

pub fn choose_command() -> CliResult<()> {
    let choices = vec!["apple", "banana", "cherry"];
    let choice = Select::new("pick a flavor:", choices).prompt()?;

    println!("selected choice: {}", choice);
    Ok(())
}
