use std::path::Path;

use crate::api::command::base;
use crate::api::command::create::create_dir_safe;

pub fn run() -> anyhow::Result<()> {
    const ROOT: &str = "/Users/saladin/study"; // TODO: make it from  .env + default 

    let root: &Path = Path::new(ROOT);

    let command = base::accepts_commands()?;

    let path_to = create_dir_safe(root, &command)?;

    println!("Тема: {command}");

    Ok(())
}
