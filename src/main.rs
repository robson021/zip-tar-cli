use crate::cmd::{arg_parser, command_builder, command_runner};
use crate::file::file_metadata::FileMetadata;
use std::env;
use std::error::Error;
use std::process::exit;
use strum_macros::Display;

mod cmd;
mod error;
mod file;
mod input_handler;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);

    if args.len() > 1 {
        match arg_parser::parse_and_run(args) {
            Ok(_) => exit(0),
            Err(e) => error::print_error(e),
        }
    }
    loop {
        match input_handler::select_menu_option(MENU_ITEMS) {
            Some(option) => handle_menu_option(option),
            None => {
                error::print_warning("Operation cancelled by user.");
                exit(0)
            }
        }
    }
}

#[derive(Copy, Clone, Display)]
enum MainMenuItem {
    #[strum(serialize = "Create archive")]
    CreateArchive,
    #[strum(serialize = "Extract archive(s)")]
    ExtractArchives,
    #[strum(serialize = "Add to existing archive")]
    AddToExistingArchive,
    #[strum(serialize = "Exit program")]
    ExitProgram,
}

#[derive(Copy, Clone, Display)]
enum CreateArchivePasswordItem {
    #[strum(serialize = "No password")]
    NoPassword,
    #[strum(serialize = "Use password")]
    UsePassword,
    #[strum(serialize = "Back")]
    Back,
}

#[derive(Copy, Clone, Display)]
enum CreateArchiveFormatItem {
    #[strum(serialize = "ZIP")]
    Zip,
    #[strum(serialize = "TAR")]
    Tar,
    #[strum(serialize = "Back")]
    Back,
}

#[derive(Copy, Clone, Display)]
enum ExtractMenuItem {
    #[strum(serialize = "Extract one archive from a path")]
    ExtractOne,
    #[strum(serialize = "Extract all archives from a path")]
    ExtractAllFromPath,
    #[strum(serialize = "Extract all archives in current directory")]
    ExtractAllCurrentDir,
    #[strum(serialize = "Back")]
    Back,
}

const MENU_ITEMS: &[MainMenuItem] = &[
    MainMenuItem::CreateArchive,
    MainMenuItem::ExtractArchives,
    MainMenuItem::AddToExistingArchive,
    MainMenuItem::ExitProgram,
];

const CREATE_ARCHIVE_PASSWORD_ITEMS: &[CreateArchivePasswordItem] = &[
    CreateArchivePasswordItem::NoPassword,
    CreateArchivePasswordItem::UsePassword,
    CreateArchivePasswordItem::Back,
];

const CREATE_ARCHIVE_FORMAT_ITEMS: &[CreateArchiveFormatItem] = &[
    CreateArchiveFormatItem::Zip,
    CreateArchiveFormatItem::Tar,
    CreateArchiveFormatItem::Back,
];

const EXTRACT_MENU_ITEMS: &[ExtractMenuItem] = &[
    ExtractMenuItem::ExtractOne,
    ExtractMenuItem::ExtractAllFromPath,
    ExtractMenuItem::ExtractAllCurrentDir,
    ExtractMenuItem::Back,
];

fn handle_menu_option(option: MainMenuItem) {
    let command = match option {
        MainMenuItem::CreateArchive => create_archive_flow(),
        MainMenuItem::ExtractArchives => extract_flow(),
        MainMenuItem::AddToExistingArchive => Some(command_builder::add_to_exising_archive()),
        MainMenuItem::ExitProgram => exit(0),
    };

    let Some(command) = command else {
        return;
    };

    match command {
        Ok((cmd, target)) => match command_runner::run_command(&cmd) {
            Ok(_) => {
                error::print_result_location(&target);
                exit(0)
            }
            Err(e) => error::print_error(format!("Failed with error: {e}")),
        },
        Err(error) => {
            error::print_error(error);
        }
    }
}

fn create_archive_flow() -> Option<Result<command_builder::PlannedCommand, Box<dyn Error>>> {
    let metadata = match input_handler::read_path_to_file_or_directory() {
        Ok(metadata) => metadata,
        Err(error) => return Some(Err(error)),
    };

    let with_password = match input_handler::select_submenu_option(
        "Password for ZIP archives?",
        CREATE_ARCHIVE_PASSWORD_ITEMS,
    ) {
        Some(CreateArchivePasswordItem::NoPassword) => false,
        Some(CreateArchivePasswordItem::UsePassword) => true,
        Some(CreateArchivePasswordItem::Back) | None => return None,
    };

    let result = match input_handler::select_submenu_option(
        "Choose archive format",
        CREATE_ARCHIVE_FORMAT_ITEMS,
    ) {
        Some(CreateArchiveFormatItem::Zip) => build_zip_command(&metadata, with_password),
        Some(CreateArchiveFormatItem::Tar) => build_tar_command(&metadata),
        Some(CreateArchiveFormatItem::Back) | None => return None,
    };

    Some(result)
}

fn build_zip_command(
    metadata: &FileMetadata,
    with_password: bool,
) -> Result<command_builder::PlannedCommand, Box<dyn Error>> {
    Ok((
        command_builder::zip_path(metadata, with_password)?,
        command_builder::zip_archive_file(metadata)?,
    ))
}

fn build_tar_command(
    metadata: &FileMetadata,
) -> Result<command_builder::PlannedCommand, Box<dyn Error>> {
    Ok((
        command_builder::tar_path(metadata)?,
        command_builder::tar_archive_file(metadata)?,
    ))
}

fn extract_flow() -> Option<Result<command_builder::PlannedCommand, Box<dyn Error>>> {
    match input_handler::select_submenu_option("Choose extraction mode", EXTRACT_MENU_ITEMS) {
        Some(ExtractMenuItem::ExtractOne) => Some(command_builder::unpack()),
        Some(ExtractMenuItem::ExtractAllFromPath) => Some(command_builder::extract_all()),
        Some(ExtractMenuItem::ExtractAllCurrentDir) => {
            Some(command_builder::extract_all_in_current_dir())
        }
        Some(ExtractMenuItem::Back) | None => None,
    }
}
