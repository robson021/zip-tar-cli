use crate::error::OperationError;
use crate::file_handler;
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

#[derive(Debug)]
pub struct FileMetadata {
    pub path: String,
    pub wildcard_value: String,
    pub is_directory: bool,
    pub has_wildcard: bool,
}

impl FileMetadata {
    pub fn to_string_path(&self) -> String {
        match self.is_directory {
            true => get_from_dir(self),
            false => self.path.to_owned(),
        }
    }
}

#[inline(always)]
fn get_from_dir(metadata: &FileMetadata) -> String {
    if !metadata.is_directory {
        panic!("Not a directory: {metadata:?}");
    }
    let path = &metadata.path;
    match metadata.has_wildcard {
        true => {
            let wildcard = &metadata.wildcard_value;
            format!("{path}/{wildcard}")
        }
        false => format!("{path}/*"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_dir_with_no_wildcard() {
        let path = get_from_dir(&FileMetadata {
            path: String::from("/some/test/path"),
            wildcard_value: String::from(""),
            is_directory: true,
            has_wildcard: false,
        });
        assert_eq!(path, "/some/test/path/*");
    }

    #[test]
    fn get_dir_with_wildcard() {
        let path = get_from_dir(&FileMetadata {
            path: String::from("/some/test/path"),
            wildcard_value: String::from("*.txt"),
            is_directory: true,
            has_wildcard: true,
        });
        assert_eq!(path, "/some/test/path/*.txt");
    }
}
