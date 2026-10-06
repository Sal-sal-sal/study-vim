use crate::study::{config::resolve_root, editor::open_editor, topic::create_topic};
use anyhow::Result;
use clap::Parser;
use std::{ffi::OsString, path::PathBuf};

#[derive(Parser)]
#[command(
    name = "study",
    version,
    about = "Create a study topic and open its Markdown plan in Neovim"
)]
struct Arguments {
    /// Topic name (quote names containing spaces).
    topic: String,
    /// Study directory; defaults to STUDY_ROOT or ~/study.
    #[arg(long)]
    root: Option<PathBuf>,
    /// Neovim executable; defaults to STUDY_NVIM or nvim.
    #[arg(long)]
    editor: Option<OsString>,
}

pub fn run() -> Result<()> {
    let arguments = Arguments::parse();
    let root = resolve_root(arguments.root.as_deref())?;
    let topic = create_topic(&root, &arguments.topic)?;
    let editor = arguments
        .editor
        .or_else(|| std::env::var_os("STUDY_NVIM"))
        .unwrap_or_else(|| "nvim".into());
    open_editor(&topic, &editor)
}
