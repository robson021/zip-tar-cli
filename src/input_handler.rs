use crate::error::OperationError;
use crate::file::file_metadata;
use crate::file::file_metadata::FileMetadata;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Completion, Input, Select};
use std::error::Error;
use std::fmt::Display;
use std::fs;
use std::path::MAIN_SEPARATOR;

struct PathCompletion;

impl Completion for PathCompletion {
    fn get(&self, input: &str) -> Option<String> {
        let (dir_part, prefix) = match input.rfind(MAIN_SEPARATOR) {
            Some(index) => input.split_at(index + 1),
            None => ("", input),
        };
        let dir_to_read = match dir_part {
            "" => ".".to_owned(),
            _ => expand_home(dir_part),
        };

        let mut matches: Vec<String> = fs::read_dir(dir_to_read)
            .ok()?
            .flatten()
            .filter_map(|entry| {
                let name = entry.file_name().to_string_lossy().into_owned();
                if !name.starts_with(prefix) || (name.starts_with('.') && prefix.is_empty()) {
                    return None;
                }
                let is_dir = entry.path().is_dir();
                Some(if is_dir {
                    format!("{name}{MAIN_SEPARATOR}")
                } else {
                    name
                })
            })
            .collect();
        matches.sort();

        let common = common_prefix(&matches)?;
        (common.len() > prefix.len() || matches.len() == 1).then(|| format!("{dir_part}{common}"))
    }
}

fn expand_home(path: &str) -> String {
    match (path.strip_prefix('~'), std::env::var("HOME")) {
        (Some(rest), Ok(home)) => format!("{home}{rest}"),
        _ => path.to_owned(),
    }
}

fn common_prefix(items: &[String]) -> Option<String> {
    let first = items.first()?;
    let mut end = first.len();
    for item in &items[1..] {
        end = first
            .char_indices()
            .zip(item.chars())
            .take_while(|((_, a), b)| a == b)
            .last()
            .map_or(0, |((i, c), _)| i + c.len_utf8())
            .min(end);
    }
    Some(first[..end].to_owned())
}

fn read_string(prompt: &str) -> Result<String, Box<dyn Error>> {
    let input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("{prompt} (Tab to autocomplete)"))
        .completion_with(&PathCompletion)
        .interact_text()?;
    Ok(input.trim().to_owned())
}

fn select_menu_option_with_prompt<T: Display + Copy>(prompt: &str, items: &[T]) -> Option<T> {
    let selected_index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(items)
        .default(0)
        .interact_opt()
        .ok()
        .flatten()?;

    items.get(selected_index).copied()
}

pub fn select_menu_option<T: Display + Copy>(items: &[T]) -> Option<T> {
    select_menu_option_with_prompt("Choose an option", items)
}

pub fn select_submenu_option<T: Display + Copy>(prompt: &str, items: &[T]) -> Option<T> {
    select_menu_option_with_prompt(prompt, items)
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
