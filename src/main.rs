use crate::error::OperationError;

mod command_builder;
mod command_runner;
mod error;
mod input_handler;

fn main() {
    loop {
        print_menu();
        match input_handler::read_int() {
            Ok(option) => handle_menu_option(option),
            Err(_) => println!("Invalid option. A number was expected."),
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
        2 => command_builder::zip(),
        3 => command_builder::zip_with_password(),
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
