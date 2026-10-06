use anyhow::{Context, Result};
use std::env;

pub fn accepts_commands() -> Result<String> {
    env::args().nth(1).context("Использование: study <тема>")
}
