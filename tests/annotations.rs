use study_nvim::markdown::annotations::{Annotation, AnnotationKind, annotations};

#[test]
fn notes_use_zero_based_rows_and_utf8_byte_columns() {
    let text = "# ML\r\n\r\n  >! Это заметка\r\n>!\r\n>!not-a-note\r\n";
    assert_eq!(
        annotations(text),
        vec![
            Annotation {
                kind: AnnotationKind::Note,
                row: 2,
                start_col: 2,
                end_col: "  >! Это заметка".len(),
            },
            Annotation {
                kind: AnnotationKind::Note,
                row: 3,
                start_col: 0,
                end_col: 2,
            },
        ]
    );
}

#[test]
fn notes_ignore_fenced_indented_and_inline_code() {
    let text = "```text\n>! fenced\n```\n\n~~~\n>! tilde fence\n~~~\n\n    >! indented\n\n`>! inline`\n\n>! Real note";
    let marks = annotations(text);
    assert_eq!(marks.len(), 1);
    assert_eq!(marks[0].row, 12);
}
