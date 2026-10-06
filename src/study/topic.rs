use anyhow::{Context, Result, ensure};
use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
use serde::Serialize;
use std::{
    fs::OpenOptions,
    io::Write,
    path::{Path, PathBuf},
};

#[derive(Debug, Serialize)]
pub struct Topic {
    pub directory: PathBuf,
    pub file: PathBuf,
}

pub fn folder_link(topic: &str) -> String {
    let label = topic
        .replace('\\', "\\\\")
        .replace('[', "\\[")
        .replace(']', "\\]");
    let path = utf8_percent_encode(topic, NON_ALPHANUMERIC);
    format!("- [{label}](./{path}/)")
}

pub fn create_topic(root: &Path, topic: &str) -> Result<Topic> {
    let directory = super::directory::create_dir_safe(root, topic)?;
    let file = directory.join(format!("{topic}.study"));
    match OpenOptions::new().write(true).create_new(true).open(&file) {
        Ok(mut output) => {
            let template = format!("# {topic}\n\n## Ссылки\n\n## Папки\n");
            output
                .write_all(template.as_bytes())
                .with_context(|| format!("Cannot write study file {}", file.display()))?;
        }
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
            ensure!(
                file.is_file(),
                "Study path is not a regular file: {}",
                file.display()
            );
        }
        Err(error) => {
            return Err(error)
                .with_context(|| format!("Cannot create study file {}", file.display()));
        }
    }
    Ok(Topic { directory, file })
}
