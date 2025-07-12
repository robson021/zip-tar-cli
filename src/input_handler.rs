use crate::error::OperationError;
use std::error::Error;
use std::num::ParseIntError;
use std::path::Path;

#[inline]
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
    let file_metadata = read_path_and_check_it_exists()?;
    match file_metadata.is_directory {
        false => Ok(file_metadata.path),
        true => Err(Box::new(OperationError::ArchiveIsDirectory)),
    }
}

#[inline]
pub fn read_path_to_file_or_directory() -> Result<FileMetadata, Box<dyn Error>> {
    println!("Provide file or director path (e.g. /some/directory/my_file.png):");
    read_path_and_check_it_exists()
}

pub struct FileMetadata {
    pub path: String,
    pub is_directory: bool,
}

#[inline]
fn read_path_and_check_it_exists() -> Result<FileMetadata, Box<dyn Error>> {
    let path = read_string();
    let p = Path::new(&path);
    match p.try_exists() {
        Ok(exists) => match exists {
            true => {
                let metadata = FileMetadata {
                    path: path.to_owned(),
                    is_directory: p.is_dir(),
                };
                Ok(metadata)
            }
            false => Err(OperationError::FileDoesNotExist.into()),
        },
        Err(_) => Err(OperationError::CouldNotCheckFile(path).into()),
    }
}
