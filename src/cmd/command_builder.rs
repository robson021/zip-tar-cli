use crate::file::file_metadata::FileMetadata;
use crate::file::{file_metadata, string_utils};
use crate::input_handler;
use glob::glob;
use std::env;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

const VALID_ARCHIVE_FORMATS: &[&str] = &[
    ".zip", ".rar", ".ar", ".tar", ".tgz", ".tbz", ".tbz2", ".tzo", ".cab", ".cbz", ".zoo",
    ".tar.xz", ".tar.gz", ".tar.bz", ".tar.bz2", ".tar.lzo", ".tar.7z",
];

pub type PlannedCommand = (String, PathBuf);

pub fn extraction_dir() -> Result<PathBuf, Box<dyn Error>> {
    Ok(env::current_dir()?.join("extracted_files"))
}

pub fn zip_archive_file(metadata: &FileMetadata) -> Result<PathBuf, Box<dyn Error>> {
    let name = get_clean_archive_name(&metadata.to_short_name()?);
    Ok(env::current_dir()?.join(format!("{name}.zip")))
}

pub fn tar_archive_file(metadata: &FileMetadata) -> Result<PathBuf, Box<dyn Error>> {
    let name = get_clean_archive_name(&metadata.to_short_name()?);
    Ok(env::current_dir()?.join(format!("{name}.tar")))
}

#[inline]
pub fn unpack_path(path: &str) -> Result<String, Box<dyn Error>> {
    unpack_path_to_dir(path, &extraction_dir()?)
}

#[inline]
pub fn unpack_path_to_dir(path: &str, target_dir: &Path) -> Result<String, Box<dyn Error>> {
    fs::create_dir_all(target_dir)?;

    Ok(format!("tar -xvf '{path}' -C '{}'", target_dir.display()))
}

pub fn unpack_all_in_path(path: &str) -> Result<String, Box<dyn Error>> {
    let archive_paths = find_archive_paths_in_path(path)?;

    println!(
        "Found {} files to extract: {:?}.",
        archive_paths.len(),
        archive_paths
    );

    let mut commands = Vec::new();
    for file in archive_paths {
        let file = unpack_path(&file.to_string_lossy())?;
        commands.push(file);
    }
    Ok(commands.join(" && "))
}

fn find_archive_paths_in_path(path: &str) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let metadata = file_metadata::get_file_metadata(path)?;
    let wildcard = metadata.wildcard.as_deref().unwrap_or("*");
    let pattern = Path::new(&metadata.path).join(wildcard);
    let mut archive_paths = glob(&pattern.to_string_lossy())?.collect::<Result<Vec<_>, _>>()?;
    archive_paths.retain(|path| {
        path.is_file()
            && path
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| {
                    VALID_ARCHIVE_FORMATS
                        .iter()
                        .any(|extension| name.ends_with(extension))
                })
    });
    archive_paths.sort();
    Ok(archive_paths)
}

fn build_list_archives_command(archive_paths: &[PathBuf]) -> String {
    if archive_paths.is_empty() {
        return "printf 'No archives found in current directory.\\n'".to_owned();
    }

    let escaped_paths: Vec<String> = archive_paths
        .iter()
        .map(|path| path.to_string_lossy().replace('\'', "'\\''"))
        .collect();
    format!("printf '%s\\n' '{}'", escaped_paths.join("' '"))
}

pub fn zip_path(metadata: &FileMetadata, with_password: bool) -> Result<String, Box<dyn Error>> {
    let encryption = match with_password {
        true => Some(""),
        false => None,
    };
    zip_path_with_password(metadata, encryption)
}

pub fn zip_path_with_password(
    metadata: &FileMetadata,
    password: Option<&str>,
) -> Result<String, Box<dyn Error>> {
    let destination_archive = get_clean_archive_name(&metadata.to_short_name()?);
    let path_to_files = metadata.to_string_path();
    let cmd = match password {
        Some(pwd) if !pwd.is_empty() => {
            let safe_pwd = pwd.replace('\'', "'\\''");
            format!("zip -P '{safe_pwd}' -r {destination_archive}.zip {path_to_files}")
        }
        Some(_) => format!("zip -re {destination_archive}.zip {path_to_files}"),
        None => format!("zip -r {destination_archive}.zip {path_to_files}"),
    };
    Ok(cmd)
}

pub fn tar_path(metadata: &FileMetadata) -> Result<String, Box<dyn Error>> {
    let destination_archive = get_clean_archive_name(&metadata.to_short_name()?);
    let path = metadata.to_string_path();
    let cmd = format!("tar -cf {destination_archive}.tar {path}");
    Ok(cmd)
}

#[inline]
fn get_clean_archive_name(short_name: &str) -> String {
    let short_name = short_name.replace(".", "_");
    format!("{short_name}_archive")
}

pub fn add_to_exising_archive() -> Result<PlannedCommand, Box<dyn Error>> {
    print!("Let's find existing archive. ");
    let archive = input_handler::read_path_to_archive()?;
    let ext = string_utils::find_file_extension(&archive)?;
    print!("What do you want to add to the exising {ext} archive? ");
    let files = input_handler::read_path_to_file_or_directory()?.to_string_path();
    let cmd = match ext.as_str() {
        ".zip" => format!("zip -ur {archive} {files}"),
        _ => format!("tar -rv --append --file={archive} {files}"),
    };
    Ok((cmd, PathBuf::from(archive)))
}

pub fn extract_from_path() -> Result<PlannedCommand, Box<dyn Error>> {
    let metadata = input_handler::read_path_to_file_or_directory()?;
    let command = match metadata.is_directory {
        true => unpack_all_in_path(&metadata.to_string_path())?,
        false => unpack_path(&metadata.path)?,
    };
    Ok((command, extraction_dir()?))
}

pub fn list_all_archives_in_current_dir() -> Result<PlannedCommand, Box<dyn Error>> {
    let archive_paths = find_archive_paths_in_path(".")?;
    Ok((
        build_list_archives_command(&archive_paths),
        env::current_dir()?,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_all_respects_archive_wildcard() {
        let command = unpack_all_in_path("./resources/test/archives/*.zip").unwrap();

        assert!(command.contains("resources_archive.zip"));
        assert!(!command.contains("resources_archive.tar"));
    }

    #[test]
    fn zip_with_explicit_password() {
        let metadata = FileMetadata {
            path: "./resources/test/files".to_owned(),
            wildcard: None,
            is_directory: true,
        };
        let cmd = zip_path_with_password(&metadata, Some("my_secret")).unwrap();
        assert_eq!(
            "zip -P 'my_secret' -r files_archive.zip ./resources/test/files/*",
            cmd
        );
    }

    #[test]
    fn list_archives_command_has_archives_only() {
        let archives = find_archive_paths_in_path("./resources/test/archives").unwrap();
        let command = build_list_archives_command(&archives);

        assert!(command.starts_with("printf '%s\\n' "));
        assert!(command.contains("resources_archive.tar"));
        assert!(command.contains("resources_archive.zip"));
        assert!(!command.contains("test1.txt"));
    }

    #[test]
    fn list_archives_command_when_empty() {
        let command = build_list_archives_command(&[]);

        assert_eq!(
            "printf 'No archives found in current directory.\\n'",
            command
        );
    }
}
