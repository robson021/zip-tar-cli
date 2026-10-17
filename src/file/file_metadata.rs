use crate::error::OperationError;
use crate::file::string_utils;
use anyhow::Result;
use std::path::{Path, PathBuf};

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
            false => self.path.clone(),
        }
    }
    pub fn to_short_name(&self) -> Result<String> {
        let path = Path::new(&self.path);
        let name = if self.is_directory {
            path.file_name()
        } else {
            path.file_stem()
        };
        name.and_then(|name| name.to_str())
            .map(str::to_owned)
            .ok_or_else(|| OperationError::CouldNotSplitPath(self.path.clone()).into())
    }
}

pub fn get_file_metadata(full_path: &str) -> Result<FileMetadata> {
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
    if !p.try_exists()? {
        return Err(OperationError::FileDoesNotExist.into());
    }

    let is_directory = p.is_dir();
    let metadata = FileMetadata {
        path,
        is_directory,
        wildcard: wildcard_value,
    };
    println!("Found file/directory: {metadata:?}.");
    Ok(metadata)
}

#[inline]
fn get_from_dir(metadata: &FileMetadata) -> String {
    let path = PathBuf::from(&metadata.path);
    let wildcard = metadata.wildcard.as_deref().unwrap_or("*");
    path.join(wildcard).to_string_lossy().into_owned()
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
