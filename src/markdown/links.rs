use anyhow::{Context, Result, ensure};
use pulldown_cmark::{Event, Options, Parser, Tag};

/// Neovim uses one-based lines and zero-based UTF-8 byte columns.
pub fn link_at(text: &str, line: usize, column: usize) -> Result<Option<String>> {
    ensure!(line > 0, "Cursor line must be positive");
    let current = text
        .split('\n')
        .nth(line - 1)
        .context("Cursor line is outside the document")?;
    ensure!(
        column <= current.len() && current.is_char_boundary(column),
        "Invalid cursor byte column"
    );
    let offset = text
        .split('\n')
        .take(line - 1)
        .map(|part| part.len() + 1)
        .sum::<usize>()
        + column;
    let options =
        Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH | Options::ENABLE_TASKLISTS;
    for (event, range) in Parser::new_ext(text, options).into_offset_iter() {
        if let Event::Start(Tag::Link { dest_url, .. }) = event
            && range.contains(&offset)
        {
            return Ok(Some(dest_url.into_string()));
        }
    }
    Ok(None)
}
