use pulldown_cmark::{Event, Options, Parser, Tag};
use serde::Serialize;
use std::ops::Range;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AnnotationKind {
    Note,
    Completed,
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
    let lines: Vec<_> = text
        .split('\n')
        .scan(0, |offset, source| {
            let start = *offset;
            *offset += source.len() + 1;
            Some((start, source.trim_end_matches('\r')))
        })
        .collect();
    let mut code = Vec::new();
    let mut result = Vec::new();
    let options = Options::ENABLE_TASKLISTS | Options::ENABLE_STRIKETHROUGH;
    for (event, range) in Parser::new_ext(text, options).into_offset_iter() {
        match event {
            Event::Start(Tag::CodeBlock(_)) => code.push(range),
            Event::TaskListMarker(true) => {
                let row = lines.partition_point(|(start, _)| *start <= range.start) - 1;
                result.push(Annotation {
                    kind: AnnotationKind::Completed,
                    row,
                    start_col: 0,
                    end_col: lines[row].1.len(),
                });
            }
            Event::Start(Tag::Strikethrough) => add_range(&mut result, &lines, range),
            _ => {}
        }
    }
    for (row, &(offset, line)) in lines.iter().enumerate() {
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
    }
    merge_ranges(result)
}

fn add_range(result: &mut Vec<Annotation>, lines: &[(usize, &str)], range: Range<usize>) {
    let first = lines.partition_point(|(offset, _)| *offset <= range.start) - 1;
    for (row, &(offset, line)) in lines.iter().enumerate().skip(first) {
        if offset >= range.end {
            break;
        }
        let start_col = range.start.saturating_sub(offset).min(line.len());
        let end_col = range.end.saturating_sub(offset).min(line.len());
        if start_col < end_col {
            result.push(Annotation {
                kind: AnnotationKind::Completed,
                row,
                start_col,
                end_col,
            });
        }
    }
}

fn merge_ranges(mut annotations: Vec<Annotation>) -> Vec<Annotation> {
    annotations.sort_by_key(|mark| (mark.kind, mark.row, mark.start_col));
    let mut result: Vec<Annotation> = Vec::new();
    for mark in annotations {
        if let Some(previous) = result.last_mut()
            && previous.kind == mark.kind
            && previous.row == mark.row
            && previous.end_col >= mark.start_col
        {
            previous.end_col = previous.end_col.max(mark.end_col);
        } else {
            result.push(mark);
        }
    }
    result
}
