use crate::error::OperationError;
use crate::file_metadata::FileMetadata;
use crate::string_utils;
use std::error::Error;
use std::path::Path;

pub fn get_file_metadata(full_path: &str) -> Result<FileMetadata, Box<dyn Error>> {
    let has_wildcard = full_path.contains("*");
    let mut wildcard_value = None;
    let path = match has_wildcard {
        true => {
            let dir = string_utils::find_dir_of_wildcard_files(full_path)?;
            wildcard_value = Some(full_path[dir.len() + 1..].to_owned());
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
                    wildcard: wildcard_value,
                };
                println!("Found file/directory: {metadata:?}.");
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

    const TEST_FILES: &str = "./resources/test/files";

    #[test]
    fn should_get_metadata() {
        let metadata = get_file_metadata(TEST_FILES).unwrap();
        assert!(metadata.is_directory);
        assert!(metadata.wildcard.is_none());
        assert_eq!(metadata.path, TEST_FILES);
    }

    #[test]
    fn should_get_metadata_with_wildcard_file_extension() {
        let path = format!("{TEST_FILES}/*.txt");
        let metadata = get_file_metadata(&path).unwrap();
        assert!(metadata.is_directory);
        assert_eq!(metadata.path, TEST_FILES);
        assert_eq!(metadata.wildcard.unwrap(), "*.txt");
    }

    #[test]
    fn should_get_metadata_with_wildcard_file_name() {
        let path = format!("{TEST_FILES}/some_file*");
        let metadata = get_file_metadata(&path).unwrap();
        assert!(metadata.is_directory);
        assert_eq!(metadata.path, TEST_FILES);
        assert_eq!(metadata.wildcard.unwrap(), "some_file*");
    }

    #[test]
    fn should_get_metadata_with_double_wildcard_file_name() {
        let path = format!("{TEST_FILES}/*some_file*");
        let metadata = get_file_metadata(&path).unwrap();
        assert!(metadata.is_directory);
        assert_eq!(metadata.path, TEST_FILES);
        assert_eq!(metadata.wildcard.unwrap(), "*some_file*");
    }
}
