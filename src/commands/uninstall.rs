use std::fs;
use which::which;

use crate::{constants::BINARY_NAME, CliError, CliResult};

// uninstalls devx binary
//
// errors:
// - CliError::Command: if the binary file cannot be found
// - CliError::IOError: if the binary file cannot be removed
pub fn uninstall_command() -> CliResult<()> {
    let binary_path = which(BINARY_NAME).map_err(|e| CliError::Command(e.to_string()))?;

    if binary_path.exists() {
        fs::remove_file(&binary_path).map_err(CliError::IOError)?;
        println!("devx uninstalled successfully");
    } else {
        println!("devx binary not found at {}", binary_path.display());
    }

    Ok(())
}
