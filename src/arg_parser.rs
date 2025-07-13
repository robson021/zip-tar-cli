use crate::command_builder::tar_path;
use crate::error::OperationError;
use crate::{command_builder, command_runner, file_handler};
use command_builder::{unpack_path, zip_path};
use std::error::Error;

const EXPECTED_NUMER_OF_ARGS: usize = 3;

pub fn parse_and_run(cmd_args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let number_of_args = cmd_args.len();
    if number_of_args != EXPECTED_NUMER_OF_ARGS {
        let msg = format!(
            "Invalid number of arguments. Expected {EXPECTED_NUMER_OF_ARGS}, but was {number_of_args}.",
        );
        return Err(OperationError::FailedToRunCommand(msg).into());
    }
    let action = &cmd_args[1];
    let path = &cmd_args[2];

    let metadata = file_handler::get_file_metadata(path)?;
    println!("Found file/directory: {metadata:?}.");

    let cmd: String = parse_cmd(action, &metadata.path)?;
    command_runner::run_command(&cmd)?;
    Ok(())
}

#[inline]
fn parse_cmd(action: &str, path: &str) -> Result<String, Box<dyn Error>> {
    let cmd = match action {
        "-u" | "--unpack" | "-d" | "--decompress" => unpack_path(path)?,
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
