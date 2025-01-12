use std::fs;
use which::which;

use crate::{constants::BINARY_NAME, CliError, CliResult};

// uninstalls devx binary
//
// errors:
// - CliError::Command: if the binary file cannot be found
// - CliError::IOError: if the binary file cannot be removed
pub fn uninstall() -> CliResult<()> {
    let binary_path = which(BINARY_NAME).map_err(|e| CliError::Command(e.to_string()))?;
    fs::remove_file(&binary_path).map_err(CliError::IOError)?;
    println!("devx uninstalled successfully");
    Ok(())
}

// prints location of devx binary
//
// errors:
// - CliError::Command: if the binary file cannot be found
pub fn whereis() -> CliResult<()> {
    let binary_path = which(BINARY_NAME).map_err(|e| CliError::Command(e.to_string()))?;
    println!("{}", binary_path.display());
    Ok(())
}
