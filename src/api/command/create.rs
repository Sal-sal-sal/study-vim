use anyhow::Result;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};

pub fn create_dir_safe(root: &Path, topic: &str) -> Result<PathBuf> {
    let path = root.join(topic);
    create_dir_all(path.clone())?;
    Ok(path)
}
