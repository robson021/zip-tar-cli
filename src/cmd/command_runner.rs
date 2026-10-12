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

pub fn run_command_with_output(command: &str) -> Result<String, Box<dyn Error>> {
    println!("Running command: {command}");
    let (arg1, arg2) = get_os_specific_cmd_args()?;
    let output = Command::new(arg1).arg(arg2).arg(command).output()?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        let details = if !stderr.trim().is_empty() {
            stderr.trim().to_string()
        } else if !stdout.trim().is_empty() {
            stdout.trim().to_string()
        } else {
            format!("status {}", output.status)
        };
        return Err(OperationError::FailedToRunCommand(format!(
            "Command '{command}' failed: {details}"
        ))
        .into());
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.into_owned())
}

#[inline]
fn get_os_specific_cmd_args() -> Result<(&'static str, &'static str), OperationError> {
    if cfg!(target_os = "windows") {
        Err(OperationError::UnsupportedPlatform)
    } else {
        Ok(("sh", "-c"))
    }
}
