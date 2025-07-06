use crate::error::OperationError;
use crate::input_handler;
use std::error::Error;
use std::process::Command;
use uuid::Uuid;

#[inline]
fn get_os_specific_cmd_args() -> (&'static str, &'static str) {
    if cfg!(target_os = "windows") {
        panic!("Windows is not supported yet.");
        // ("cmd", "/C")
    } else {
        ("sh", "-c")
    }
}

fn run_command(command: &str) -> Result<(), Box<dyn Error>> {
    println!("Running command: {command}");
    let (arg1, arg2) = get_os_specific_cmd_args();
    let mut cmd = Command::new(arg1).arg(arg2).arg(command).spawn()?;
    let code = cmd.wait()?.code();
    if code.is_some() {
        let code = code.unwrap();
        if code != 0 {
            return Err(OperationError::FailedToRunCommand(format!(
                "Command '{command}' failed with code: {code}"
            ))
            .into());
        }
    }
    Ok(())
}

pub fn unpack() -> Result<(), Box<dyn Error>> {
    let file = input_handler::read_path_to_archive()?;
    let cmd = format!("tar -xvf {file}");
    run_command(&cmd)?;
    Ok(())
}

pub fn zip() -> Result<(), Box<dyn Error>> {
    let file_metadata = input_handler::read_path_to_file_or_directory()?;
    let destination_archive = get_unique_archive_name();
    let path = file_metadata.path;
    let file_to_zip = match file_metadata.is_directory {
        true => format!("{path}/*"),
        false => path,
    };
    let cmd = format!("zip -r {destination_archive} {file_to_zip}");
    run_command(&cmd)?;
    Ok(())
}

#[inline]
fn get_unique_archive_name() -> String {
    let uuid = Uuid::new_v4();
    format!("archive_{uuid}.zip")
}

pub fn zip_with_password() -> Result<(), Box<dyn Error>> {
    todo!()
}

pub fn tar() -> Result<(), Box<dyn Error>> {
    todo!()
}
