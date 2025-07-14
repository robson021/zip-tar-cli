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

    #[error("Unknown argument {0}.")]
    InvalidArgument(String),

    #[error("Could not spit the path: '{0}'.")]
    CouldNotSpitPath(String),

    #[error(
        "Wildcard is only supported at the end of a path - e.g. './my/path/file*' or './my/path/.*txt'"
    )]
    InvalidWildcardIndex,
}
