use anyhow::{Context, Result, bail, ensure};
use percent_encoding::percent_decode_str;
use serde::Serialize;
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Target {
    Url {
        url: String,
    },
    Directory {
        path: PathBuf,
    },
    File {
        path: PathBuf,
        fragment: Option<String>,
    },
    Anchor {
        fragment: String,
    },
}

pub fn resolve_target(document: &Path, destination: &str) -> Result<Target> {
    let lower = destination.to_ascii_lowercase();
    if lower.starts_with("https://") || lower.starts_with("http://") {
        ensure!(!destination.chars().any(char::is_control), "Invalid URL");
        return Ok(Target::Url {
            url: destination.to_owned(),
        });
    }
    if let Some(fragment) = destination.strip_prefix('#') {
        return Ok(Target::Anchor {
            fragment: decode(fragment)?,
        });
    }
    ensure!(!destination.is_empty(), "Link destination is empty");
    let (raw_path, fragment) = destination
        .split_once('#')
        .map_or((destination, None), |(path, fragment)| {
            (path, Some(fragment))
        });
    let decoded = decode(raw_path)?;
    let drive = decoded.as_bytes().get(1) == Some(&b':')
        && decoded
            .as_bytes()
            .get(2)
            .is_some_and(|ch| matches!(ch, b'/' | b'\\'));
    ensure!(
        !decoded.contains(':') || drive,
        "Unsupported URL scheme; use http(s) or a local path"
    );
    ensure!(!decoded.contains('\0'), "Link path contains a null byte");
    let base = document
        .parent()
        .context("Study document has no parent directory")?;
    let path = base.join(decoded);
    let metadata = std::fs::metadata(&path)
        .with_context(|| format!("Linked path does not exist: {}", path.display()))?;
    if metadata.is_dir() {
        Ok(Target::Directory { path })
    } else if metadata.is_file() {
        Ok(Target::File {
            path,
            fragment: fragment.map(decode).transpose()?,
        })
    } else {
        bail!("Linked path is not a file or directory: {}", path.display())
    }
}

fn decode(value: &str) -> Result<String> {
    Ok(percent_decode_str(value)
        .decode_utf8()
        .context("Link contains invalid UTF-8")?
        .into_owned())
}
