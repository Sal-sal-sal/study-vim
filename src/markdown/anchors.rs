use anyhow::{Result, bail};
use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
use std::collections::HashMap;

pub fn heading_line(text: &str, fragment: &str) -> Result<usize> {
    if fragment.is_empty() {
        return Ok(1);
    }
    let mut heading = None;
    let mut duplicates = HashMap::<String, usize>::new();
    for (event, range) in
        Parser::new_ext(text, Options::ENABLE_HEADING_ATTRIBUTES).into_offset_iter()
    {
        match event {
            Event::Start(Tag::Heading { id, .. }) => {
                heading = Some((range.start, String::new(), id.map(|id| id.into_string())));
            }
            Event::Text(part) | Event::Code(part) => {
                if let Some((_, title, _)) = &mut heading {
                    title.push_str(&part);
                }
            }
            Event::End(TagEnd::Heading(_)) => {
                if let Some((offset, title, explicit_id)) = heading.take() {
                    let slug: String = title
                        .to_lowercase()
                        .chars()
                        .filter_map(|ch| {
                            if ch.is_whitespace() {
                                Some('-')
                            } else if ch.is_alphanumeric() || matches!(ch, '-' | '_') {
                                Some(ch)
                            } else {
                                None
                            }
                        })
                        .collect();
                    let count = duplicates.entry(slug.clone()).or_default();
                    let identifier = if *count == 0 {
                        slug
                    } else {
                        format!("{slug}-{count}")
                    };
                    *count += 1;
                    if explicit_id.as_deref() == Some(fragment) || identifier == fragment {
                        return Ok(text[..offset].bytes().filter(|byte| *byte == b'\n').count() + 1);
                    }
                }
            }
            _ => {}
        }
    }
    bail!("Heading #{fragment} was not found")
}
