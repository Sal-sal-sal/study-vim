use std::path::Path;
use std::process::Command;

use crate::api::command::base;
use crate::api::command::create::create_dir_safe;

pub fn run() -> anyhow::Result<()> {
    const ROOT: &str = "/Users/saladin/study"; // TODO: make it from  .env + default 

    let root: &Path = Path::new(ROOT);

    let command = base::accepts_commands()?;

    let path_to = create_dir_safe(root, &command)?;

    // TODO: change to path {topic}.study plugin
    let status_nvim = Command::new("nvim")
        .current_dir(path_to)
        .arg(".")
        .status()?;

    if !status_nvim.success() {
        anyhow::bail!("Neovim clesed with Error: {status_nvim}")
    }

    println!("Тема: {command}");

    Ok(())
}
