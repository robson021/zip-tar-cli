use crate::error::OperationError;
use anyhow::{Result, bail};
use std::process::Command;

const ZIP_NOTHING_TO_DO: i32 = 12;

pub fn run_command(command: &str) -> Result<()> {
    dbg!("Running command: {}", command);
    let (arg1, arg2) = get_os_specific_cmd_args()?;
    let mut cmd = Command::new(arg1).arg(arg2).arg(command).spawn()?;
    let status = cmd.wait()?;
    if !status.success() {
        if status.code() == Some(ZIP_NOTHING_TO_DO) {
            bail!(OperationError::NothingToZip);
        }
        bail!(OperationError::FailedToRunCommand(format!(
            "Command '{command}' failed with status: {status}"
        )));
    }
    Ok(())
}

fn get_os_specific_cmd_args() -> Result<(&'static str, &'static str)> {
    if cfg!(target_os = "windows") {
        bail!(OperationError::UnsupportedPlatform)
    } else {
        Ok(("sh", "-c"))
    }
}
