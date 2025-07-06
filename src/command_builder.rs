use crate::input_handler;
use std::error::Error;
use uuid::Uuid;

// todo: support wildcards like: ./some/path/*.png

pub fn unpack() -> Result<String, Box<dyn Error>> {
    let file = input_handler::read_path_to_archive()?;
    let cmd = format!("tar -xvf {file}");
    Ok(cmd)
}

pub fn zip() -> Result<String, Box<dyn Error>> {
    let cmd = get_zip_command()?;
    Ok(format!("zip -r {cmd}"))
}

pub fn zip_with_password() -> Result<String, Box<dyn Error>> {
    let cmd = get_zip_command()?;
    Ok(format!("zip -er {cmd}"))
}

pub fn tar() -> Result<String, Box<dyn Error>> {
    todo!()
}

#[inline]
fn get_unique_archive_name() -> String {
    let uuid = Uuid::new_v4();
    format!("archive_{uuid}.zip")
}

fn get_zip_command() -> Result<String, Box<dyn Error>> {
    let file_metadata = input_handler::read_path_to_file_or_directory()?;
    let destination_archive = get_unique_archive_name();
    let path = file_metadata.path;
    let file_to_zip = match file_metadata.is_directory {
        true => format!("{path}/*"),
        false => path,
    };
    Ok(format!("{destination_archive} {file_to_zip}"))
}
