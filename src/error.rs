use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("interaction failed: {0}")]
    Interaction(#[from] inquire::error::InquireError),

    #[error("command failed: {0}")]
    Command(String),
}

pub type CliResult<T> = Result<T, CliError>;
