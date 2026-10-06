use anyhow::Result;
use std::fs::create_dir_all;
use std::path::{self, Path, PathBuf};

pub fn create_dir_safe(root: &Path, topic: &str) -> Result<PathBuf> {
    let path = root.join(topic);
    let path_tr = create_dir_all(path.clone())?;
    Ok(path)
}
