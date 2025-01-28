pub mod commands;
pub mod constants;
pub mod error;

pub use error::{CliError, CliResult};

// re-export for external use
pub use commands::git::GitCommands;
pub use commands::manage::ManageCommands;

// trait for all commands
pub trait Command {
    fn execute(&self) -> CliResult<()>;
}
