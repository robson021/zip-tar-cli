use crate::error::OperationError;
use std::error::Error;

pub fn find_dir_of_wildcard_files(path: &str) -> Result<String, Box<dyn Error>> {
    match path.find("*.") {
        Some(index) => Ok(path[..index - 1].to_owned()),
        None => Err(OperationError::CouldNotFindDirForFileWithWildcard(path.to_owned()).into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_dir_of_wildcard_files() {
        let dir = find_dir_of_wildcard_files("/foo/bar/path/*.txt").unwrap();
        assert_eq!(dir, "/foo/bar/path");
    }
}
