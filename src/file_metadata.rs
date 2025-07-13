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
}
