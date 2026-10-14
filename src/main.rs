use crate::cmd::{arg_parser, command_builder, command_runner};
use crate::error::OperationError;
use std::env;
use std::process::exit;

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

const MENU_ITEMS: &[&str] = &[
    "Extract archive",
    "Zip file or directory",
    "Zip and secure with password",
    "Tar file or directory",
    "Add to existing archive",
    "Extract all archives in a specific directory",
    "Extract all archives in the current directory",
    "Exit program",
];

fn handle_menu_option(option: usize) {
    let command = match option {
        0 => command_builder::unpack(),
        1 => command_builder::zip(false),
        2 => command_builder::zip(true),
        3 => command_builder::tar(),
        4 => command_builder::add_to_exising_archive(),
        5 => command_builder::extract_all(),
        6 => command_builder::extract_all_in_current_dir(),
        7 => exit(0),
        _ => Err(OperationError::InvalidCommand.into()),
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
