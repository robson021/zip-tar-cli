use crate::command_builder::tar_path;
use crate::error::OperationError;
use crate::{command_builder, command_runner, file_handler};
use command_builder::{unpack_path, zip_path};
use std::error::Error;

const EXPECTED_NUMER_OF_ARGS: usize = 3;

pub fn parse_and_run(cmd_args: Vec<String>) -> Result<(), Box<dyn Error>> {
    validate_number_of_args(cmd_args.len())?;

    let action = &cmd_args[1];
    let path = &cmd_args[2];

    let metadata = file_handler::get_file_metadata(path)?;
    println!("Found file/directory: {metadata:?}.");

    let cmd: String = parse_cmd(action, &metadata.to_string_path())?;
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
fn parse_cmd(action: &str, path: &str) -> Result<String, Box<dyn Error>> {
    let cmd = match action {
        "-x" | "-u" | "--unpack" | "-d" | "--decompress" => unpack_path(path)?,
        "-z" | "--zip" => zip_path(path, false)?,
        "-ze" | "-ez" | "--zip_encrypt" => zip_path(path, true)?,
        "-t" | "--tar" => tar_path(path)?,
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

    const TEST_FILES: &str = "./resources/test";

    #[test]
    fn parse_decompress() {
        for arg in ["-u", "--unpack", "-d", "--decompress", "-x"] {
            let cmd = parse_cmd(arg, TEST_FILES).unwrap();
            assert_eq!(cmd, format!("tar -xvf {TEST_FILES}"));
        }
    }

    #[test]
    fn parse_zip() {
        for arg in ["-z", "--zip"] {
            let cmd = parse_cmd(arg, TEST_FILES).unwrap();
            assert!(cmd.starts_with("zip -r archive_"));
            assert!(cmd.ends_with(&format!(".zip {TEST_FILES}")));
        }
    }

    #[test]
    fn parse_zip_encrypt() {
        for arg in ["-ze", "-ez", "--zip_encrypt"] {
            let cmd = parse_cmd(arg, TEST_FILES).unwrap();
            assert!(cmd.starts_with("zip -re archive_"));
            assert!(cmd.ends_with(&format!(".zip {TEST_FILES}")));
        }
    }

    #[test]
    fn parse_tar() {
        for arg in ["-t", "--tar"] {
            let cmd = parse_cmd(arg, TEST_FILES).unwrap();
            assert!(cmd.starts_with("tar -cf archive_"));
            assert!(cmd.ends_with(&format!(".tar {TEST_FILES}")));
        }
    }
}
