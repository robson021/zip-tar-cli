use crate::error::OperationError;

mod command_runner;
mod error;

fn main() {
    loop {
        print_menu();
        match read_input().parse::<i32>() {
            Ok(o) => handle_menu_option(o),
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

fn read_input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Can not read user input.");
    input.trim().to_owned()
}

fn handle_menu_option(option: i32) {
    let result = match option {
        1 => command_runner::unpack(),
        2 => command_runner::zip(),
        3 => command_runner::zip_with_password(),
        4 => command_runner::tar(),
        0 => std::process::exit(0),
        _ => Err(OperationError::InvalidCommand.into()),
    };
    if result.is_err() {
        eprintln!("{}", result.unwrap_err());
    }
}
