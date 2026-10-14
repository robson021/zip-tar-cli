use crate::error::OperationError;
use crate::file::file_metadata;
use crate::file::file_metadata::FileMetadata;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input, Select};
use std::error::Error;

fn read_string(prompt: &str) -> Result<String, Box<dyn Error>> {
    let input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .interact_text()?;
    Ok(input.trim().to_owned())
}

pub fn select_menu_option(items: &[&str]) -> Option<usize> {
    Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose an option")
        .items(items)
        .default(0)
        .interact_opt()
        .ok()
        .flatten()
}

pub fn read_path_to_archive() -> Result<String, Box<dyn Error>> {
    let path = read_string("Archive path (e.g. /some/directory/my_file.zip)")?;
    let file_metadata = file_metadata::get_file_metadata(&path)?;
    match file_metadata.is_directory {
        false => Ok(file_metadata.path),
        true => Err(Box::new(OperationError::ArchiveIsDirectory)),
    }
}

pub fn read_path_to_file_or_directory() -> Result<FileMetadata, Box<dyn Error>> {
    let path = read_string("File or directory path (e.g. /some/directory/my_file.png)")?;
    file_metadata::get_file_metadata(&path)
}
