use crate::error::OperationError;
use crate::file::string_utils;
use std::error::Error;
use std::path::Path;

#[derive(Debug)]
pub struct FileMetadata {
    pub path: String,
    pub wildcard: Option<String>,
    pub is_directory: bool,
}

impl FileMetadata {
    pub fn to_string_path(&self) -> String {
        match self.is_directory {
            true => get_from_dir(self),
            false => self.path.to_owned(),
        }
    }
    pub fn to_short_name(&self) -> Result<String, Box<dyn Error>> {
        let option = self.path.split('/').next_back();
        if option.is_none() {
            return Err(Box::new(OperationError::CouldNotSpitPath(
                self.path.to_owned(),
            )));
        }

        let last = option.unwrap();

        match self.is_directory {
            true => Ok(last.to_owned()),
            false => {
                let option = last.split(".").next();
                if option.is_none() {
                    return Err(Box::new(OperationError::CouldNotSpitPath(
                        "no file extension".to_owned(),
                    )));
                }

                Ok(option.unwrap().to_owned())
            }
        }
    }
}

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

#[inline]
fn get_from_dir(metadata: &FileMetadata) -> String {
    if !metadata.is_directory {
        panic!("Not a directory: {metadata:?}");
    }
    let path = &metadata.path;
    match &metadata.wildcard {
        None => format!("{path}/*"),
        Some(wildcard) => format!("{path}/{wildcard}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_dir_with_no_wildcard() {
        let path = get_from_dir(&FileMetadata {
            path: String::from("/some/test/path"),
            wildcard: None,
            is_directory: true,
        });
        assert_eq!(path, "/some/test/path/*");
    }

    #[test]
    fn get_dir_with_wildcard() {
        let path = get_from_dir(&FileMetadata {
            path: String::from("/some/test/path"),
            wildcard: Some(String::from("*.txt")),
            is_directory: true,
        });
        assert_eq!(path, "/some/test/path/*.txt");
    }

    #[test]
    fn get_short_name_of_dir() {
        let metadata = FileMetadata {
            path: String::from("/some/test/path/my_dir"),
            wildcard: None,
            is_directory: true,
        };
        assert_eq!(metadata.to_short_name().unwrap(), "my_dir");
    }

    #[test]
    fn get_short_name_of_file() {
        let metadata = FileMetadata {
            path: String::from("/some/test/path/my_file"),
            wildcard: None,
            is_directory: false,
        };
        assert_eq!(metadata.to_short_name().unwrap(), "my_file");
    }

    #[test]
    fn get_short_name_of_file_with_extension() {
        let metadata = FileMetadata {
            path: String::from("/some/test/path/my_file.txt"),
            wildcard: None,
            is_directory: false,
        };
        assert_eq!(metadata.to_short_name().unwrap(), "my_file");
    }

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
