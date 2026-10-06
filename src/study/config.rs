use anyhow::{Context, Result, ensure};
use std::path::{Path, PathBuf};

pub fn resolve_root(configured: Option<&Path>) -> Result<PathBuf> {
    let environment = std::env::var_os("STUDY_ROOT").map(PathBuf::from);
    let home = dirs::home_dir();
    let current = std::env::current_dir().context("Cannot find the current directory")?;
    root_path(
        configured.or(environment.as_deref()),
        home.as_deref(),
        &current,
    )
}

pub fn root_path(
    configured: Option<&Path>,
    home: Option<&Path>,
    current: &Path,
) -> Result<PathBuf> {
    let path = match configured {
        Some(path) => {
            ensure!(!path.as_os_str().is_empty(), "Study root must not be empty");
            let mut components = path.components();
            if components
                .next()
                .is_some_and(|part| part.as_os_str() == "~")
            {
                home.context("Cannot find the home directory; set STUDY_ROOT")?
                    .join(components.as_path())
            } else {
                path.to_path_buf()
            }
        }
        None => home
            .context("Cannot find the home directory; set STUDY_ROOT")?
            .join("study"),
    };
    Ok(if path.is_absolute() {
        path
    } else {
        current.join(path)
    })
}
