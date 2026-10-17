use crate::error::OperationError;
use anyhow::{Result, anyhow};

pub fn find_dir_of_wildcard_files(path: &str) -> Result<String> {
    let slash_idx = path
        .rfind('/')
        .ok_or_else(|| OperationError::CouldNotFindDirForFileWithWildcard(path.to_owned()))?;
    let wildcard_idx = path
        .rfind('*')
        .ok_or_else(|| OperationError::CouldNotFindDirForFileWithWildcard(path.to_owned()))?;

    if wildcard_idx < slash_idx {
        return Err(OperationError::InvalidWildcardIndex.into());
    }
    Ok(path[..slash_idx].to_owned())
}

pub fn find_file_extension(path: &str) -> Result<String> {
    path.rfind('.')
        .map(|idx| path[idx..].to_owned())
        .ok_or_else(|| anyhow!("No extension found."))
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
}
