use crate::command_builder::tar_path;
use crate::error::OperationError;
use crate::file_metadata::FileMetadata;
use crate::{command_builder, command_runner, file_handler};
use command_builder::{unpack_all_in_path, unpack_path, zip_path};
use std::error::Error;

const EXPECTED_NUMER_OF_ARGS: usize = 3;

pub fn parse_and_run(cmd_args: Vec<String>) -> Result<(), Box<dyn Error>> {
    validate_number_of_args(cmd_args.len())?;

    let action = &cmd_args[1];
    let path = &cmd_args[2];

    let metadata = file_handler::get_file_metadata(path)?;

    let cmd: String = parse_cmd(action, &metadata)?;
    command_runner::run_command(&cmd)?;
    Ok(())
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

#[inline]
fn parse_cmd(action: &str, metadata: &FileMetadata) -> Result<String, Box<dyn Error>> {
    let cmd = match action {
        "-x" | "-u" | "--unpack" | "-d" | "--decompress" => unpack_path(&metadata.path)?,
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
    use lazy_static::lazy_static;

    const TEST_FILES: &str = "./resources/test/files";
    const TEST_ARCHIVE_FILES: &str = "./resources/test/archives";

    lazy_static! {
        static ref TEST_METADATA: FileMetadata = FileMetadata {
            path: TEST_FILES.to_owned(),
            wildcard: None,
            is_directory: true,
        };
        static ref TEST_ARCHIVES_METADATA: FileMetadata = FileMetadata {
            path: TEST_ARCHIVE_FILES.to_owned(),
            wildcard: None,
            is_directory: true,
        };
    }

    #[test]
    fn parse_decompress() {
        for arg in ["-u", "--unpack", "-d", "--decompress", "-x"] {
            let cmd = parse_cmd(arg, &TEST_METADATA).unwrap();
            assert_eq!(cmd, format!("tar -xvf {TEST_FILES}"));
        }
    }

    #[test]
    fn parse_zip() {
        for arg in ["-z", "--zip"] {
            let cmd = parse_cmd(arg, &TEST_METADATA).unwrap();
            assert_eq!("zip -r files_archive.zip ./resources/test/files/*", cmd);
        }
    }

    #[test]
    fn parse_zip_encrypt() {
        for arg in ["-ze", "-ez", "--zip-encrypt"] {
            let cmd = parse_cmd(arg, &TEST_METADATA).unwrap();
            assert_eq!("zip -re files_archive.zip ./resources/test/files/*", cmd);
        }
    }

    #[test]
    fn parse_tar() {
        for arg in ["-t", "--tar"] {
            let cmd = parse_cmd(arg, &TEST_METADATA).unwrap();
            assert_eq!("tar -cf files_archive.tar ./resources/test/files/*", cmd);
        }
    }

    #[test]
    fn extract_multiple_archives() {
        for arg in ["-xa", "-ax", "--extract-all"] {
            let cmd = parse_cmd(arg, &TEST_ARCHIVES_METADATA).unwrap();
            assert!(cmd.contains("tar -xvf ./resources/test/archives/resources_archive.tar"));
            assert!(cmd.contains("tar -xvf ./resources/test/archives/resources_archive.zip"));
            assert!(cmd.contains(" && "));

            // may be collected in different order
            // assert_eq!(
            //     "tar -xvf ./resources/test/archives/resources_archive.tar \
            //     && tar -xvf ./resources/test/archives/resources_archive.zip",
            //     cmd
            // );
        }
    }
}
