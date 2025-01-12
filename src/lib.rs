pub mod commands;
pub mod constants;
pub mod error;

pub use error::{CliError, CliResult};

// re-export for external use
pub use commands::choose::choose;
pub use commands::manage::{uninstall, whereis, ManageCommands};

// trait for all commands
pub trait Command {
    fn execute(&self) -> CliResult<()>;
}
