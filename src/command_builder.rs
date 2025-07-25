use crate::command_runner::execute_cmd_get_lines;
use crate::file_metadata::FileMetadata;
use crate::{file_handler, input_handler, string_utils};
use lazy_static::lazy_static;
use std::collections::HashSet;
use std::error::Error;

lazy_static! {
    static ref VALID_ARCHIVE_FORMATS: HashSet<&'static str> = HashSet::from_iter(vec![
        ".zip", ".rar", ".ar", ".tar", ".tgz", ".tbz", ".tbz2", ".tzo", ".cab", ".cbz", ".zoo",
        ".tar.xz", ".tar.gz", ".tar.bz", ".tar.bz2", ".tar.lzo", ".tar.7z",
    ]);
    static ref FORMATS_JOINED: String = VALID_ARCHIVE_FORMATS
        .to_owned()
        .iter()
        .copied()
        .collect::<Vec<&str>>()
        .join("|");
}

#[inline]
pub fn unpack() -> Result<String, Box<dyn Error>> {
    let file = input_handler::read_path_to_archive()?;
    unpack_path(&file)
}

#[inline]
pub fn unpack_path(path: &str) -> Result<String, Box<dyn Error>> {
    let cmd = format!("tar -xvf '{path}'");
    Ok(cmd)
}

pub fn unpack_all_in_path(path: &str) -> Result<String, Box<dyn Error>> {
    let path = file_handler::get_file_metadata(path)?.to_string_path();
    let path = path.trim_end_matches("/*");
    let formats = FORMATS_JOINED.as_str();

    let files = execute_cmd_get_lines(&format!("ls '{path}' | grep -E '{formats}'"));
    let archive_paths = files
        .iter()
        .filter(|file| match string_utils::find_file_extension(file) {
            Ok(ext) => VALID_ARCHIVE_FORMATS.contains(&&*ext),
            _ => false,
        })
        .map(|file| format!("{path}/{file}"))
        .collect::<Vec<String>>();

    println!(
        "Found {} files to extract: {:?}.",
        archive_paths.len(),
        archive_paths
    );

    let mut commands = Vec::new();
    for file in archive_paths {
        let file = unpack_path(&file)?;
        commands.push(file);
    }
    Ok(commands.join(" && "))
}

#[inline]
pub fn zip(with_password: bool) -> Result<String, Box<dyn Error>> {
    let file_metadata = input_handler::read_path_to_file_or_directory()?;
    zip_path(&file_metadata, with_password)
}

#[inline]
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

#[inline]
pub fn tar() -> Result<String, Box<dyn Error>> {
    let file_metadata = input_handler::read_path_to_file_or_directory()?;
    tar_path(&file_metadata)
}

#[inline]
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
