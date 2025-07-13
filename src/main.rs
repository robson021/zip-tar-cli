use crate::error::OperationError;
use std::env;

mod arg_parser;
mod command_builder;
mod command_runner;
mod error;
mod file_handler;
mod file_metadata;
mod input_handler;
mod string_utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);

    if args.len() > 1 {
        match arg_parser::parse_and_run(args) {
            Ok(_) => std::process::exit(0),
            Err(e) => eprintln!("{e}"),
        }
    }
    loop {
        print_menu();
        match input_handler::read_int() {
            Ok(option) => handle_menu_option(option),
            Err(_) => eprintln!("Invalid option. A number was expected."),
        };
    }
}

fn print_menu() {
    println!("\nChose an option:");
    println!("1. Unpack archive.");
    println!("2. Zip file or directory.");
    println!("3. Zip and secure with password.");
    println!("4. Tar file or directory.");
    println!("0. Exit program.");
}

fn handle_menu_option(option: i32) {
    let command = match option {
        1 => command_builder::unpack(),
        2 => command_builder::zip(false),
        3 => command_builder::zip(true),
        4 => command_builder::tar(),
        0 => std::process::exit(0),
        _ => Err(OperationError::InvalidCommand.into()),
    };

    match command {
        Ok(cmd) => {
            let r = command_runner::run_command(&cmd);
            if r.is_err() {
                eprintln!("Failed to run command '{cmd}': {:?}", r.unwrap_err());
            }
        }
        Err(error) => {
            eprintln!("{error}");
        }
    }
}
