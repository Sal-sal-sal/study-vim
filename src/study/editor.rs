use super::topic::Topic;
use anyhow::{Context, Result, ensure};
use std::{ffi::OsStr, process::Command};

pub fn open_editor(topic: &Topic, executable: &OsStr) -> Result<()> {
    let status = Command::new(executable)
        .current_dir(&topic.directory)
        .arg("--")
        .arg(&topic.file)
        .status()
        .with_context(|| {
            format!(
                "Cannot start {:?}; install Neovim or set STUDY_NVIM",
                executable
            )
        })?;
    ensure!(status.success(), "Neovim exited with {status}");
    Ok(())
}
