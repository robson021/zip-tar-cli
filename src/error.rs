use thiserror::Error;

#[derive(Error, Debug)]
pub enum OperationError {
    #[error("Invalid argument. Provide a valid number.")]
    InvalidCommand,

    #[error("File does not exist.")]
    FileDoesNotExist,

    #[error("Failed to check file existence in the path {0}.")]
    CouldNotCheckFile(String),

    #[error("Could not find the directory for files with wildcard {0}.")]
    CouldNotFindDirForFileWithWildcard(String),

    #[error("Failed to run command: '{0}'.")]
    FailedToRunCommand(String),

    #[error("Archive is a directory.")]
    ArchiveIsDirectory,
}
