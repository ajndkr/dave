pub mod commands;
pub mod constants;
pub mod error;

pub use error::{CliError, CliResult};

// re-export for external use
pub use commands::choose::choose_command;
pub use commands::uninstall::uninstall_command;
