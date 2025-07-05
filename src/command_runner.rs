use crate::error::OperationError;
use crate::input_handler;
use std::error::Error;
use std::process::Command;

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
    // todo: unpack in the same dir as archive
    let file = input_handler::read_path()?;
    let cmd = format!("tar -xvf {file}");
    run_command(&cmd)?;
    Ok(())
}

pub fn zip() -> Result<(), Box<dyn Error>> {
    todo!()
}

pub fn zip_with_password() -> Result<(), Box<dyn Error>> {
    todo!()
}

pub fn tar() -> Result<(), Box<dyn Error>> {
    todo!()
}
