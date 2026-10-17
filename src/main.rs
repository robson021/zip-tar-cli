use crate::cmd::{arg_parser, command_builder, command_runner, version_check};
use crate::file::file_metadata::FileMetadata;
use crate::operations::{
    CREATE_ARCHIVE_FORMAT_ITEMS, CREATE_ARCHIVE_PASSWORD_ITEMS, CreateArchiveFormatItem,
    CreateArchivePasswordItem, EXTRACT_MENU_ITEMS, ExtractMenuItem, MENU_ITEMS, MainMenuItem,
};
use std::env;
use std::error::Error;
use std::process::exit;

mod cmd;
mod error;
mod file;
mod input_handler;
mod operations;

fn main() {
    if !version_check::check_archive_tools() {
        error::print_error("Both 'zip' and 'tar' are missing. Install at least one to continue.");
        exit(1)
    }

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

fn handle_menu_option(option: MainMenuItem) {
    let command = match option {
        MainMenuItem::CreateArchive => create_archive_flow(),
        MainMenuItem::ExtractArchives => extract_flow(),
        MainMenuItem::AddToExistingArchive => command_builder::add_to_exising_archive().map(Some),
        MainMenuItem::ExitProgram => exit(0),
    };

    match command {
        Ok(Some((cmd, target))) => match command_runner::run_command(&cmd) {
            Ok(_) => {
                error::print_result_location(&target);
                exit(0)
            }
            Err(e) => error::print_error(format!("Failed with error: {e}")),
        },
        Ok(None) => (),
        Err(error) => {
            error::print_error(error);
        }
    }
}

fn create_archive_flow() -> Result<Option<command_builder::PlannedCommand>, Box<dyn Error>> {
    let metadata = input_handler::read_path_to_file_or_directory()?;

    let result = match input_handler::select_submenu_option(
        "Choose archive format",
        CREATE_ARCHIVE_FORMAT_ITEMS,
    ) {
        Some(CreateArchiveFormatItem::Zip) => build_zip_command(&metadata),
        Some(CreateArchiveFormatItem::Tar) => build_tar_command(&metadata),
        Some(CreateArchiveFormatItem::Back) | None => return Ok(None),
    };

    Ok(Some(result?))
}

fn build_zip_command(
    metadata: &FileMetadata,
) -> Result<command_builder::PlannedCommand, Box<dyn Error>> {
    let with_password = matches!(
        input_handler::select_submenu_option(
            "Password for ZIP archives?",
            CREATE_ARCHIVE_PASSWORD_ITEMS
        ),
        Some(CreateArchivePasswordItem::UsePassword)
    );

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

fn extract_flow() -> Result<Option<command_builder::PlannedCommand>, Box<dyn Error>> {
    match input_handler::select_submenu_option("Choose extraction mode", EXTRACT_MENU_ITEMS) {
        Some(ExtractMenuItem::ExtractFromPath) => Ok(Some(command_builder::extract_from_path()?)),
        Some(ExtractMenuItem::ListAllCurrentDir) => {
            Ok(Some(command_builder::list_all_archives_in_current_dir()?))
        }
        Some(ExtractMenuItem::Back) | None => Ok(None),
    }
}
