use anyhow::Result;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};

pub fn create_dir_safe(root: &Path, topic: &str) -> Result<PathBuf> {
    let bad_string = ";,./[]<>*&^%$#()\\-+=";

    if topic.trim().is_empty() || topic.chars().any(|ch| bad_string.contains(ch)) {
        let err: anyhow::Error = anyhow::anyhow!(
            "Invalid topic name {topic:?}: expected a nonempty name \
             without these characters: {bad_string}",
        );
        return Err(err);
    }

    let path = root.join(topic);

    create_dir_all(path.clone())?;
    Ok(path)
}
