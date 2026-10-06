use anyhow::{Context, Result, ensure};
use serde::Serialize;
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Serialize)]
pub struct Directory {
    pub path: PathBuf,
    pub parent: Option<PathBuf>,
    pub entries: Vec<Entry>,
}

#[derive(Serialize)]
pub struct Entry {
    pub name: String,
    pub path: PathBuf,
    pub directory: bool,
}

pub fn browse(path: &Path) -> Result<Directory> {
    ensure!(path.is_dir(), "Not a directory: {}", path.display());
    let mut entries = fs::read_dir(path)
        .with_context(|| format!("Cannot read directory {}", path.display()))?
        .map(|entry| {
            let entry = entry?;
            let path = entry.path();
            Ok(Entry {
                name: entry.file_name().to_string_lossy().into_owned(),
                directory: path.is_dir(),
                path,
            })
        })
        .collect::<Result<Vec<_>>>()?;
    entries.sort_by_key(|entry| {
        (
            !entry.directory,
            entry.name.to_lowercase(),
            entry.name.clone(),
        )
    });
    Ok(Directory {
        path: path.into(),
        parent: path.parent().map(Path::to_path_buf),
        entries,
    })
}
