use crate::command_builder::tar_path;
use crate::error::OperationError;
use crate::file_metadata::FileMetadata;
use crate::{command_builder, command_runner, file_handler};
use command_builder::{unpack_path, zip_path};
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
        "-z" | "--zip" => zip_path(metadata, false)?,
        "-ze" | "-ez" | "--zip_encrypt" => zip_path(metadata, true)?,
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

    const TEST_FILES: &str = "./resources/test";

    lazy_static! {
        static ref TEST_METADATA: FileMetadata = FileMetadata {
            path: TEST_FILES.to_owned(),
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
            assert!(cmd.starts_with("zip -r test_archive"));
            assert!(cmd.ends_with(&format!(".zip {TEST_FILES}/*")));
        }
    }

    #[test]
    fn parse_zip_encrypt() {
        for arg in ["-ze", "-ez", "--zip_encrypt"] {
            let cmd = parse_cmd(arg, &TEST_METADATA).unwrap();
            assert!(cmd.starts_with("zip -re test_archive"));
            assert!(cmd.ends_with(&format!(".zip {TEST_FILES}/*")));
        }
    }

    #[test]
    fn parse_tar() {
        for arg in ["-t", "--tar"] {
            let cmd = parse_cmd(arg, &TEST_METADATA).unwrap();
            assert!(cmd.starts_with("tar -cf test_archive"));
            assert!(cmd.ends_with(&format!(".tar {TEST_FILES}/*")));
        }
    }
}
