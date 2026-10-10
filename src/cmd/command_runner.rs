use crate::error::OperationError;
use std::error::Error;
use std::process::Command;

pub fn run_command(command: &str) -> Result<(), Box<dyn Error>> {
    println!("Running command: {command}");
    let (arg1, arg2) = get_os_specific_cmd_args()?;
    let mut cmd = Command::new(arg1).arg(arg2).arg(command).spawn()?;
    let status = cmd.wait()?;
    if !status.success() {
        return Err(OperationError::FailedToRunCommand(format!(
            "Command '{command}' failed with status: {status}"
        ))
        .into());
    }
    Ok(())
}

#[inline]
fn get_os_specific_cmd_args() -> Result<(&'static str, &'static str), OperationError> {
    if cfg!(target_os = "windows") {
        Err(OperationError::UnsupportedPlatform)
    } else {
        Ok(("sh", "-c"))
    }
}
