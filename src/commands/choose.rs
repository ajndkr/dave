use crate::error::CliResult;

use inquire::Select;

// choose a flavor using 'inquire.Select'
// 'inquire.Select' creates an interactive prompt for selecting a choice from a list
pub fn choose_command() -> CliResult<()> {
    let choices = vec!["apple", "banana", "cherry"];
    let choice = Select::new("pick a flavor:", choices).prompt()?;

    println!("selected choice: {}", choice);
    Ok(())
}
