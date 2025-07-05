use thiserror::Error;

#[derive(Error, Debug)]
pub enum OperationError {
    #[error("Invalid argument. Provide a valid number.")]
    InvalidCommand,
}
