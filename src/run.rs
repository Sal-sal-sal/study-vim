use crate::api::command::base;

pub fn run() -> anyhow::Result<()> {
    let command = base::accepts_commands()?;

    const ROOT: &str = "/Users/saladin/study";

    println!("Тема: {command}");

    Ok(())
}
