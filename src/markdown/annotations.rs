use pulldown_cmark::{Event, Parser, Tag};
use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AnnotationKind {
    Note,
}

/// Positions use Neovim's zero-based rows and UTF-8 byte columns.
#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Annotation {
    pub kind: AnnotationKind,
    pub row: usize,
    pub start_col: usize,
    pub end_col: usize,
}

pub fn annotations(text: &str) -> Vec<Annotation> {
    let code: Vec<_> = Parser::new(text)
        .into_offset_iter()
        .filter_map(|(event, range)| {
            matches!(event, Event::Start(Tag::CodeBlock(_))).then_some(range)
        })
        .collect();
    let mut result = Vec::new();
    let mut offset = 0;
    for (row, source) in text.split('\n').enumerate() {
        let line = source.trim_end_matches('\r');
        let content = line.trim_start_matches(' ');
        let indent = line.len() - content.len();
        let marker = content
            .strip_prefix(">!")
            .is_some_and(|rest| rest.is_empty() || rest.starts_with(char::is_whitespace));
        if indent <= 3 && marker && !code.iter().any(|range| range.contains(&(offset + indent))) {
            result.push(Annotation {
                kind: AnnotationKind::Note,
                row,
                start_col: indent,
                end_col: line.len(),
            });
        }
        offset += source.len() + 1;
    }
    result
}
