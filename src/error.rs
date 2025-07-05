use thiserror::Error;

#[derive(Error, Debug)]
pub enum OperationError {
    #[error("Invalid argument. Provide a valid number.")]
    InvalidCommand,

    #[error("File does not exist.")]
    FileDoesNotExist,

    #[error("Failed to check file existence in the path {0}.")]
    CouldNotCheckFile(String),

    #[error("Failed to run command: '{0}'.")]
    FailedToRunCommand(String),
}
