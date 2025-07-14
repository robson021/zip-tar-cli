use crate::file_metadata::FileMetadata;
use crate::input_handler;
use std::error::Error;

pub fn unpack() -> Result<String, Box<dyn Error>> {
    let file = input_handler::read_path_to_archive()?;
    unpack_path(&file)
}

#[inline]
pub fn unpack_path(path: &str) -> Result<String, Box<dyn Error>> {
    let cmd = format!("tar -xvf {path}");
    Ok(cmd)
}

#[inline]
pub fn zip(with_password: bool) -> Result<String, Box<dyn Error>> {
    let file_metadata = input_handler::read_path_to_file_or_directory()?;
    zip_path(&file_metadata, with_password)
}

#[inline]
pub fn zip_path(metadata: &FileMetadata, with_password: bool) -> Result<String, Box<dyn Error>> {
    let destination_archive = get_unique_archive_name(&metadata.to_short_name()?);
    let encryption = match with_password {
        true => "e",
        false => "",
    };
    let path_to_files = metadata.to_string_path();
    Ok(format!(
        "zip -r{encryption} {destination_archive}.zip {path_to_files}"
    ))
}

#[inline]
pub fn tar() -> Result<String, Box<dyn Error>> {
    let file_metadata = input_handler::read_path_to_file_or_directory()?;
    tar_path(&file_metadata)
}

#[inline]
pub fn tar_path(metadata: &FileMetadata) -> Result<String, Box<dyn Error>> {
    let destination_archive = get_unique_archive_name(&metadata.to_short_name()?);
    let path = metadata.to_string_path();
    let cmd = format!("tar -cf {destination_archive}.tar {path}");
    Ok(cmd)
}

#[inline]
fn get_unique_archive_name(short_name: &str) -> String {
    format!("{short_name}_archive")
}
