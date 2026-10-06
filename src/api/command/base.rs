use anyhow::{Context, Result};
use std::env;

pub fn accepts_commands() -> Result<String> {
    let topic = env::args()
        .nth(1)
        .context("Использование: study-nvim <тема>"); // превращает
    // some into Ok, None into Error
    match topic {
        Ok(name) => Ok(name),
        Err(err) => Err(err),
    }
}
