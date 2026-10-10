use crate::file::file_metadata::FileMetadata;
use crate::file::{file_metadata, string_utils};
use crate::input_handler;
use glob::glob;
use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;

const VALID_ARCHIVE_FORMATS: &[&str] = &[
    ".zip", ".rar", ".ar", ".tar", ".tgz", ".tbz", ".tbz2", ".tzo", ".cab", ".cbz", ".zoo",
    ".tar.xz", ".tar.gz", ".tar.bz", ".tar.bz2", ".tar.lzo", ".tar.7z",
];

#[inline]
pub fn unpack() -> Result<String, Box<dyn Error>> {
    let file = input_handler::read_path_to_archive()?;
    unpack_path(&file)
}

#[inline]
pub fn unpack_path(path: &str) -> Result<String, Box<dyn Error>> {
    let target_dir = env::current_dir()?.join("extracted_files");
    fs::create_dir_all(&target_dir)?;

    Ok(format!("tar -xvf '{path}' -C '{}'", target_dir.display()))
}

pub fn unpack_all_in_path(path: &str) -> Result<String, Box<dyn Error>> {
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

pub fn zip(with_password: bool) -> Result<String, Box<dyn Error>> {
    let file_metadata = input_handler::read_path_to_file_or_directory()?;
    zip_path(&file_metadata, with_password)
}

pub fn zip_path(metadata: &FileMetadata, with_password: bool) -> Result<String, Box<dyn Error>> {
    let destination_archive = get_clean_archive_name(&metadata.to_short_name()?);
    let encryption = match with_password {
        true => "e",
        false => "",
    };
    let path_to_files = metadata.to_string_path();
    Ok(format!(
        "zip -r{encryption} {destination_archive}.zip {path_to_files}"
    ))
}

pub fn tar() -> Result<String, Box<dyn Error>> {
    let file_metadata = input_handler::read_path_to_file_or_directory()?;
    tar_path(&file_metadata)
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

pub fn add_to_exising_archive() -> Result<String, Box<dyn Error>> {
    print!("Let's find existing archive. ");
    let archive = input_handler::read_path_to_archive()?;
    let ext = string_utils::find_file_extension(&archive)?;
    print!("What do you want to add to the exising {ext} archive? ");
    let files = input_handler::read_path_to_file_or_directory()?.to_string_path();
    let cmd = match ext.as_str() {
        ".zip" => format!("zip -ur {archive} {files}"),
        _ => format!("tar -rv --append --file={archive} {files}"),
    };
    Ok(cmd)
}

pub fn extract_all() -> Result<String, Box<dyn Error>> {
    let path = input_handler::read_path_to_file_or_directory()?.to_string_path();
    let cmd = unpack_all_in_path(&path)?;
    Ok(cmd)
}

pub fn extract_all_in_current_dir() -> Result<String, Box<dyn Error>> {
    let cmd = unpack_all_in_path(".")?;
    Ok(cmd)
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
}
