use crate::error::OperationError;
use std::error::Error;

pub fn find_dir_of_wildcard_files(path: &str) -> Result<String, Box<dyn Error>> {
    match last_index_of_char(path, '/') {
        Some(slash_idx) => {
            let wildcard_idx = last_index_of_char(path, '*').unwrap();
            if wildcard_idx < slash_idx {
                return Err(OperationError::InvalidWildcardIndex.into());
            }
            Ok(path[..slash_idx].to_owned())
        }
        None => Err(OperationError::CouldNotFindDirForFileWithWildcard(path.to_owned()).into()),
    }
}

pub fn find_file_extension(path: &str) -> Result<String, &'static str> {
    match last_index_of_char(path, '.') {
        Some(idx) => Ok(path[idx..].to_owned()),
        None => Err("No extension found."),
    }
}

fn last_index_of_char(s: &str, to_find: char) -> Option<usize> {
    s.chars()
        .rev()
        .position(|c| c == to_find)
        .map(|rev_pos| s.chars().count() - rev_pos - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_dir_of_wildcard_files() {
        let dir = find_dir_of_wildcard_files("/foo/bar/path/file*").unwrap();
        assert_eq!(dir, "/foo/bar/path");

        let dir = find_dir_of_wildcard_files("/foo/bar/path/*.txt").unwrap();
        assert_eq!(dir, "/foo/bar/path");
    }

    #[test]
    fn should_fail_for_invalid_wildcard_index() {
        let dir = find_dir_of_wildcard_files("/foo/bar/*/file.txt");
        assert!(dir.is_err());
    }

    #[test]
    fn should_find_file_extension() {
        let ext = find_file_extension("/foo/bar/path/file.zip").unwrap();
        assert_eq!(ext, ".zip");
    }

    #[test]
    fn should_find_last_index_of_char() {
        let index = last_index_of_char("/foo/bar/path/example_file.txt", '/').unwrap();
        assert_eq!(13, index);

        let option = last_index_of_char("/foo/bar/file", '.');
        assert!(option.is_none());
    }
}
