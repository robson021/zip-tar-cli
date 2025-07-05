use crate::error::OperationError;
use std::error::Error;
use std::num::ParseIntError;
use std::path::Path;

fn read_string() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Can not read user input.");
    input.trim().to_owned()
}

pub fn read_int() -> Result<i32, ParseIntError> {
    let input = read_string();
    input.parse::<i32>()
}

pub fn read_path() -> Result<String, Box<dyn Error>> {
    println!("Provide archive path (e.g. /some/directory/my_file.zip):");
    let path = read_string();
    match Path::new(&path).try_exists() {
        Ok(exists) => match exists {
            true => Ok(path),
            false => Err(OperationError::FileDoesNotExist.into()),
        },
        Err(_) => Err(OperationError::CouldNotCheckFile(path).into()),
    }
}
