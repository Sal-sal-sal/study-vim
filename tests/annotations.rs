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

#[test]
fn completed_tasks_and_struck_text_ignore_pending_tasks_and_code() {
    let text = "- [ ] pending\n- [x] Готово\n1. [X] Complete\nplain ~~часть~~ tail\n`~~code~~`\n\n```\n- [x] example\n~~example~~\n```\n";
    assert_eq!(
        annotations(text),
        vec![
            Annotation {
                kind: AnnotationKind::Completed,
                row: 1,
                start_col: 0,
                end_col: "- [x] Готово".len(),
            },
            Annotation {
                kind: AnnotationKind::Completed,
                row: 2,
                start_col: 0,
                end_col: "1. [X] Complete".len(),
            },
            Annotation {
                kind: AnnotationKind::Completed,
                row: 3,
                start_col: "plain ".len(),
                end_col: "plain ~~часть~~".len(),
            },
        ]
    );
}

#[test]
fn multiline_strikes_use_each_rows_byte_columns_and_merge_overlaps() {
    let text = "~~первая\r\nвторая~~\r\n\r\n- [x] ~~готово~~\r\n";
    let marks = annotations(text);
    assert_eq!(marks.len(), 3);
    for (mark, row, text) in [
        (&marks[0], 0, "~~первая"),
        (&marks[1], 1, "вторая~~"),
        (&marks[2], 3, "- [x] ~~готово~~"),
    ] {
        assert_eq!(mark.row, row);
        assert_eq!(mark.start_col, 0);
        assert_eq!(mark.end_col, text.len());
    }
    assert!(annotations(r"\~\~escaped\~\~").is_empty());
}
