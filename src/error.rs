use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("interaction failed: {0}")]
    Interaction(#[from] inquire::error::InquireError),

    #[error("command failed: {0}")]
    Command(String),

    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),
}

pub type CliResult<T> = Result<T, CliError>;
