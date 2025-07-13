use crate::error::OperationError;
use crate::string_utils;
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

#[inline]
pub fn read_int() -> Result<i32, ParseIntError> {
    let input = read_string();
    input.parse::<i32>()
}

#[inline]
pub fn read_path_to_archive() -> Result<String, Box<dyn Error>> {
    println!("Provide archive path (e.g. /some/directory/my_file.zip):");
    let path = read_string();
    let file_metadata = get_file_metadata(&path)?;
    match file_metadata.is_directory {
        false => Ok(file_metadata.path),
        true => Err(Box::new(OperationError::ArchiveIsDirectory)),
    }
}

#[inline]
pub fn read_path_to_file_or_directory() -> Result<FileMetadata, Box<dyn Error>> {
    println!("Provide file or director path (e.g. /some/directory/my_file.png):");
    let path = read_string();
    get_file_metadata(&path)
}

#[derive(Debug)]
pub struct FileMetadata {
    pub path: String,
    pub wildcard_value: String,
    pub is_directory: bool,
    pub has_wildcard: bool,
}

pub fn get_file_metadata(full_path: &str) -> Result<FileMetadata, Box<dyn Error>> {
    let has_wildcard = full_path.contains("*.");
    let mut wildcard_value = String::new();
    let path = match has_wildcard {
        true => {
            let dir = string_utils::find_dir_of_wildcard_files(full_path)?;
            wildcard_value = full_path[dir.len() + 1..].to_owned();
            dir
        }
        false => full_path.to_owned(),
    };

    let p = Path::new(&path);
    match p.try_exists() {
        Ok(exists) => match exists {
            true => {
                let metadata = FileMetadata {
                    path: path.to_owned(),
                    is_directory: p.is_dir(),
                    has_wildcard,
                    wildcard_value,
                };
                Ok(metadata)
            }
            false => Err(OperationError::FileDoesNotExist.into()),
        },
        Err(_) => Err(OperationError::CouldNotCheckFile(path).into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_FILES: &str = "./resources/test";

    #[test]
    fn should_get_metadata() {
        let metadata = get_file_metadata(TEST_FILES).unwrap();
        assert!(metadata.is_directory);
        assert!(!metadata.has_wildcard);
        assert_eq!(metadata.path, TEST_FILES);
        assert_eq!(metadata.wildcard_value, "");
    }

    #[test]
    fn should_get_metadata_with_wildcard() {
        let path = format!("{TEST_FILES}/*.txt");
        let metadata = get_file_metadata(&path).unwrap();
        assert!(metadata.is_directory);
        assert!(metadata.has_wildcard);
        assert_eq!(metadata.path, TEST_FILES);
        assert_eq!(metadata.wildcard_value, "*.txt");
    }
}
