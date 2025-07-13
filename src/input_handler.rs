use crate::error::OperationError;
use crate::file_handler;
use crate::file_metadata::FileMetadata;
use std::error::Error;
use std::num::ParseIntError;

fn read_string() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Can not read user input.");
    input.trim().to_owned()
}

#[inline]
pub fn read_int() -> Result<i32, ParseIntError> {
    let input = read_string();
    input.parse::<i32>()
}

#[inline]
pub fn read_path_to_archive() -> Result<String, Box<dyn Error>> {
    println!("Provide archive path (e.g. /some/directory/my_file.zip):");
    let path = read_string();
    let file_metadata = file_handler::get_file_metadata(&path)?;
    match file_metadata.is_directory {
        false => Ok(file_metadata.path),
        true => Err(Box::new(OperationError::ArchiveIsDirectory)),
    }
}

#[inline]
pub fn read_path_to_file_or_directory() -> Result<FileMetadata, Box<dyn Error>> {
    println!("Provide file or director path (e.g. /some/directory/my_file.png):");
    let path = read_string();
    file_handler::get_file_metadata(&path)
}
