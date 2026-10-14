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

pub fn unpack() -> Result<PlannedCommand, Box<dyn Error>> {
    let file = input_handler::read_path_to_archive()?;
    Ok((unpack_path(&file)?, extraction_dir()?))
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

pub fn extract_archive_stem(file_name: &str) -> &str {
    let mut sorted_formats = VALID_ARCHIVE_FORMATS.to_vec();
    sorted_formats.sort_by_key(|b| std::cmp::Reverse(b.len()));
    for ext in sorted_formats {
        if let Some(stripped) = file_name.strip_suffix(ext) {
            return stripped;
        }
    }
    Path::new(file_name)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(file_name)
}

pub fn get_unpacked_dir_for_archive(archive_path: &Path) -> PathBuf {
    let parent = archive_path.parent().unwrap_or_else(|| Path::new("."));
    let parent = if parent.as_os_str().is_empty() {
        Path::new(".")
    } else {
        parent
    };
    let file_name = archive_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("archive");
    let stem = extract_archive_stem(file_name);
    parent.join(format!("unpacked_{stem}"))
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

pub fn zip(with_password: bool) -> Result<PlannedCommand, Box<dyn Error>> {
    let file_metadata = input_handler::read_path_to_file_or_directory()?;
    Ok((
        zip_path(&file_metadata, with_password)?,
        zip_archive_file(&file_metadata)?,
    ))
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

pub fn zip_path_to_dir(
    metadata: &FileMetadata,
    target_dir: &Path,
    password: Option<&str>,
) -> Result<String, Box<dyn Error>> {
    let destination_archive = get_clean_archive_name(&metadata.to_short_name()?);
    let destination_file = target_dir.join(format!("{destination_archive}.zip"));
    let path_to_files = metadata.to_string_path();
    let dest_str = destination_file.display();
    let cmd = match password {
        Some(pwd) if !pwd.is_empty() => {
            let safe_pwd = pwd.replace('\'', "'\\''");
            format!("zip -P '{safe_pwd}' -r '{dest_str}' {path_to_files}")
        }
        Some(_) => format!("zip -re '{dest_str}' {path_to_files}"),
        None => format!("zip -r '{dest_str}' {path_to_files}"),
    };
    Ok(cmd)
}

pub fn tar() -> Result<PlannedCommand, Box<dyn Error>> {
    let file_metadata = input_handler::read_path_to_file_or_directory()?;
    Ok((tar_path(&file_metadata)?, tar_archive_file(&file_metadata)?))
}

pub fn tar_path(metadata: &FileMetadata) -> Result<String, Box<dyn Error>> {
    let destination_archive = get_clean_archive_name(&metadata.to_short_name()?);
    let path = metadata.to_string_path();
    let cmd = format!("tar -cf {destination_archive}.tar {path}");
    Ok(cmd)
}

pub fn tar_path_to_dir(
    metadata: &FileMetadata,
    target_dir: &Path,
) -> Result<String, Box<dyn Error>> {
    let destination_archive = get_clean_archive_name(&metadata.to_short_name()?);
    let destination_file = target_dir.join(format!("{destination_archive}.tar"));
    let path = metadata.to_string_path();
    let cmd = format!("tar -cf '{}' {path}", destination_file.display());
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

pub fn extract_all() -> Result<PlannedCommand, Box<dyn Error>> {
    let path = input_handler::read_path_to_file_or_directory()?.to_string_path();
    Ok((unpack_all_in_path(&path)?, extraction_dir()?))
}

pub fn extract_all_in_current_dir() -> Result<PlannedCommand, Box<dyn Error>> {
    Ok((unpack_all_in_path(".")?, extraction_dir()?))
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
    fn test_extract_archive_stem() {
        assert_eq!(extract_archive_stem("archive123.zip"), "archive123");
        assert_eq!(extract_archive_stem("archive123.tar.gz"), "archive123");
        assert_eq!(extract_archive_stem("my_file.tar"), "my_file");
        assert_eq!(extract_archive_stem("unknown_ext.xyz"), "unknown_ext");
    }

    #[test]
    fn test_get_unpacked_dir_for_archive() {
        let path = Path::new("/some/dir/archive123.zip");
        let unpacked_dir = get_unpacked_dir_for_archive(path);
        assert_eq!(unpacked_dir, PathBuf::from("/some/dir/unpacked_archive123"));

        let path_tar_gz = Path::new("/some/dir/data.tar.gz");
        assert_eq!(
            get_unpacked_dir_for_archive(path_tar_gz),
            PathBuf::from("/some/dir/unpacked_data")
        );
    }

    #[test]
    fn test_zip_path_to_dir() {
        let metadata = FileMetadata {
            path: "./resources/test/files".to_owned(),
            wildcard: None,
            is_directory: true,
        };
        let target_dir = Path::new("/output/dir");
        let cmd = zip_path_to_dir(&metadata, target_dir, None).unwrap();
        assert_eq!(
            "zip -r '/output/dir/files_archive.zip' ./resources/test/files/*",
            cmd
        );

        let cmd_pwd = zip_path_to_dir(&metadata, target_dir, Some("pass123")).unwrap();
        assert_eq!(
            "zip -P 'pass123' -r '/output/dir/files_archive.zip' ./resources/test/files/*",
            cmd_pwd
        );
    }

    #[test]
    fn test_tar_path_to_dir() {
        let metadata = FileMetadata {
            path: "./resources/test/files".to_owned(),
            wildcard: None,
            is_directory: true,
        };
        let target_dir = Path::new("/output/dir");
        let cmd = tar_path_to_dir(&metadata, target_dir).unwrap();
        assert_eq!(
            "tar -cf '/output/dir/files_archive.tar' ./resources/test/files/*",
            cmd
        );
    }
}
