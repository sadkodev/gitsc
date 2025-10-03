use crate::error::{Error, Result};
use std::process::Command;

/// Checks if the current directory is a Git repository.
pub fn is_git_repository() -> bool {
    Command::new("git")
        .arg("rev-parse")
        .arg("--is-inside-work-tree")
        .output()
        .map(|output| {
            output.status.success() && String::from_utf8_lossy(&output.stdout).trim() == "true"
        })
        .unwrap_or(false)
}

/// Gets the staged diff from the Git repository.
///
/// # Returns
///
/// * `Ok(String)` if there are staged changes.
/// * `Err(Error::NoStagedChanges)` if there are no staged changes.
/// * `Err(Error::Git)` if the command fails for other reasons.
pub fn get_staged_diff() -> Result<String> {
    let output = Command::new("git").arg("diff").arg("--staged").output()?;

    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(Error::Git(format!("Failed to get staged diff: {}", error_message)));
    }

    let diff = String::from_utf8_lossy(&output.stdout).to_string();

    if diff.trim().is_empty() {
        Err(Error::NoStagedChanges)
    } else {
        Ok(diff)
    }
}
