use crate::input_handler;
use crate::input_handler::FileMetadata;
use std::error::Error;
use uuid::Uuid;

pub fn unpack() -> Result<String, Box<dyn Error>> {
    let file = input_handler::read_path_to_archive()?;
    let cmd = format!("tar -xvf {file}");
    Ok(cmd)
}

pub fn zip(with_password: bool) -> Result<String, Box<dyn Error>> {
    let destination_archive = get_unique_archive_name();
    let file_to_zip = read_files_to_be_archived()?;
    let encryption = match with_password {
        true => "e",
        false => "",
    };
    Ok(format!(
        "zip -r{encryption} {destination_archive}.zip {file_to_zip}"
    ))
}

pub fn tar() -> Result<String, Box<dyn Error>> {
    let destination_archive = get_unique_archive_name();
    let file_to_zip = read_files_to_be_archived()?;
    let cmd = format!("tar -cf {destination_archive}.tar {file_to_zip}");
    Ok(cmd)
}

fn read_files_to_be_archived() -> Result<String, Box<dyn Error>> {
    let file_metadata = input_handler::read_path_to_file_or_directory()?;
    let file_to_archive = match file_metadata.is_directory {
        true => get_from_dir(&file_metadata),
        false => file_metadata.path,
    };
    Ok(file_to_archive)
}

#[inline]
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

#[inline]
fn get_unique_archive_name() -> String {
    let uuid = Uuid::new_v4();
    format!("archive_{uuid}")
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
