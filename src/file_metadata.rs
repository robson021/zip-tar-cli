use crate::error::OperationError;
use std::error::Error;

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

#[inline(always)]
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
}
