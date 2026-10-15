use crate::cmd::command_builder::{
    extraction_dir, tar_archive_file, tar_path, unpack_all_in_path, unpack_path, zip_archive_file,
    zip_path,
};
use crate::cmd::command_runner;
use crate::error::{OperationError, print_result_location};
use crate::file::file_metadata;
use crate::file::file_metadata::FileMetadata;
use std::error::Error;
use std::path::PathBuf;

const EXPECTED_NUMER_OF_ARGS: usize = 3;

pub fn parse_and_run(cmd_args: Vec<String>) -> Result<(), Box<dyn Error>> {
    validate_number_of_args(cmd_args.len())?;

    let action = &cmd_args[1];
    let path = &cmd_args[2];

    let metadata = file_metadata::get_file_metadata(path)?;

    let cmd: String = parse_cmd(action, &metadata)?;
    command_runner::run_command(&cmd)?;
    print_result_location(&resolve_target(action, &metadata)?);
    Ok(())
}

fn resolve_target(action: &str, metadata: &FileMetadata) -> Result<PathBuf, Box<dyn Error>> {
    match action {
        "-z" | "--zip" | "-ze" | "-ez" | "--zip-encrypt" => zip_archive_file(metadata),
        "-t" | "--tar" => tar_archive_file(metadata),
        _ => extraction_dir(),
    }
}

#[inline(always)]
fn validate_number_of_args(number_of_args: usize) -> Result<(), Box<dyn Error>> {
    if number_of_args != EXPECTED_NUMER_OF_ARGS {
        let expected = EXPECTED_NUMER_OF_ARGS - 1;
        let actual = number_of_args - 1;
        let msg = format!("Invalid number of arguments. Expected {expected}, but was {actual}.",);
        return Err(OperationError::FailedToRunCommand(msg).into());
    }
    Ok(())
}

fn parse_cmd(action: &str, metadata: &FileMetadata) -> Result<String, Box<dyn Error>> {
    let cmd = match action {
        "-x" | "-u" | "--extract" | "-d" | "--decompress" => unpack_path(&metadata.path)?,
        "-xa" | "-ax" | "--extract-all" => unpack_all_in_path(&metadata.path)?,
        "-z" | "--zip" => zip_path(metadata, false)?,
        "-ze" | "-ez" | "--zip-encrypt" => zip_path(metadata, true)?,
        "-t" | "--tar" => tar_path(metadata)?,
        _ => {
            return Err(
                OperationError::InvalidArgument(format!("Invalid argument {action}.")).into(),
            );
        }
    };
    Ok(cmd)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    const TEST_FILES: &str = "./resources/test/files";
    const TEST_ARCHIVE_FILES: &str = "./resources/test/archives";

    fn test_metadata() -> FileMetadata {
        FileMetadata {
            path: TEST_FILES.to_owned(),
            wildcard: None,
            is_directory: true,
        }
    }

    fn test_archives_metadata() -> FileMetadata {
        FileMetadata {
            path: TEST_ARCHIVE_FILES.to_owned(),
            wildcard: None,
            is_directory: true,
        }
    }

    #[test]
    fn parse_decompress() {
        let metadata = test_metadata();
        for arg in ["-u", "--extract", "-d", "--decompress", "-x"] {
            let expected_path = env::current_dir().unwrap().display().to_string();
            let cmd = parse_cmd(arg, &metadata).unwrap();

            assert_eq!(
                cmd,
                format!("tar -xvf '{TEST_FILES}' -C '{expected_path}/extracted_files'")
            );
        }
    }

    #[test]
    fn parse_zip() {
        let metadata = test_metadata();
        for arg in ["-z", "--zip"] {
            let cmd = parse_cmd(arg, &metadata).unwrap();
            assert_eq!("zip -r files_archive.zip ./resources/test/files/*", cmd);
        }
    }

    #[test]
    fn parse_zip_encrypt() {
        let metadata = test_metadata();
        for arg in ["-ze", "-ez", "--zip-encrypt"] {
            let cmd = parse_cmd(arg, &metadata).unwrap();
            assert_eq!("zip -re files_archive.zip ./resources/test/files/*", cmd);
        }
    }

    #[test]
    fn parse_tar() {
        let metadata = test_metadata();
        for arg in ["-t", "--tar"] {
            let cmd = parse_cmd(arg, &metadata).unwrap();
            assert_eq!("tar -cf files_archive.tar ./resources/test/files/*", cmd);
        }
    }

    #[test]
    fn extract_multiple_archives() {
        let metadata = test_archives_metadata();
        for arg in ["-xa", "-ax", "--extract-all"] {
            let cmd = parse_cmd(arg, &metadata).unwrap();
            assert!(cmd.contains("tar -xvf 'resources/test/archives/resources_archive.tar'"));
            assert!(cmd.contains("tar -xvf 'resources/test/archives/resources_archive.zip'"));
            assert!(cmd.contains(" && "));
        }
    }
}
