use crate::error::OperationError;
use crate::input_handler;
use std::error::Error;

const EXPECTED_NUMER_OF_ARGS: usize = 3;

pub fn parse_and_run(cmd_args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let number_of_args = cmd_args.len();
    if number_of_args != EXPECTED_NUMER_OF_ARGS {
        return Err(OperationError::FailedToRunCommand(format!(
            "Invalid number of arguments. Expected {EXPECTED_NUMER_OF_ARGS}, but was {number_of_args}.",
        ))
            .into());
    }
    let action = &cmd_args[1];
    let path = &cmd_args[2];

    let metadata = input_handler::get_file_metadata(path)?;

    todo!();

    match action.as_str() {
        "-z" | "-zip" => {
            println!("zip")
        }
        "-ze" | "-ez" | "-zip_encrypt" => {
            println!("zip & encrypt")
        }
        "-t" | "-tar" => {
            println!("tar")
        }
        _ => {
            return Err(
                OperationError::InvalidArgument(format!("Invalid argument {action}.")).into(),
            );
        }
    };
    Ok(())
}
