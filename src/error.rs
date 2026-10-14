use thiserror::Error;

#[derive(Error, Debug)]
pub enum OperationError {
    #[error("File does not exist.")]
    FileDoesNotExist,

    #[error("Could not find the directory for files with wildcard '{0}'.")]
    CouldNotFindDirForFileWithWildcard(String),

    #[error("Failed to run command: '{0}'.")]
    FailedToRunCommand(String),

    #[error("Nothing to do.")]
    NothingToZip,

    #[error("Archive is a directory.")]
    ArchiveIsDirectory,

    #[error("Unknown argument '{0}'.")]
    InvalidArgument(String),

    #[error("Could not split the path: '{0}'.")]
    CouldNotSplitPath(String),

    #[error(
        "Wildcard is only supported at the end of a path - e.g. './my/path/file*' or './my/path/.*txt'"
    )]
    InvalidWildcardIndex,

    #[error("Windows is not supported.")]
    UnsupportedPlatform,
}

pub fn print_error(message: impl std::fmt::Display) {
    let prefix = console::style(" ✖ ERROR ").white().on_red().bold();
    eprintln!("\n{prefix} {}\n", console::style(message).red().bold());
}

pub fn print_warning(message: impl std::fmt::Display) {
    let prefix = console::style(" ⚠ ABORTED ").black().on_yellow().bold();
    eprintln!("\n{prefix} {}\n", console::style(message).yellow());
}

pub fn print_result_location(path: &std::path::Path) {
    let prefix = console::style(" ✔ DONE ").black().on_green().bold();
    println!(
        "\n{prefix} Result location: {}\n",
        console::style(path.display()).green().bold()
    );
}
