use crate::error;
use std::process::Command;

pub fn check_archive_tools() -> bool {
    let zip_version = command_version("zip", &["-v"]);
    let tar_version = command_version("tar", &["--version"]);

    match zip_version.as_ref() {
        Some(version) => println!("zip detected: {version}"),
        None => error::print_warning("'zip' command is missing. ZIP operations may fail."),
    }

    match tar_version.as_ref() {
        Some(version) => println!("tar detected: {version}"),
        None => {
            error::print_warning("'tar' command is missing. TAR/extraction operations may fail.")
        }
    }

    zip_version.is_some() || tar_version.is_some()
}

fn command_version(command: &str, args: &[&str]) -> Option<String> {
    let output = Command::new(command).args(args).output().ok()?;
    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let version_text = if stdout.trim().is_empty() {
        stderr.as_ref()
    } else {
        stdout.as_ref()
    };

    version_text
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .map(ToOwned::to_owned)
}
