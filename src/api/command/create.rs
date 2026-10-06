use anyhow::{Context, Result};
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};

pub fn create_dir_safe(root: &Path, topic: &str) -> Result<PathBuf> {
    crate::study::validation::validate_topic(topic)?;
    let path = root.join(topic);
    create_dir_all(&path)
        .with_context(|| format!("Cannot create topic directory {}", path.display()))?;
    Ok(path)
}
