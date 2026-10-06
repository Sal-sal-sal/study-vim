use study_nvim::markdown::links::link_at;

#[test]
fn selects_the_link_under_the_cursor_using_byte_offsets() {
    let text = "# ML\n\n- [Алгебра](./algebra/) и [Курс](https://example.org)\n";
    assert_eq!(link_at(text, 3, 5).unwrap().as_deref(), Some("./algebra/"));
    let column = text.lines().nth(2).unwrap().find("Курс").unwrap();
    assert_eq!(
        link_at(text, 3, column).unwrap().as_deref(),
        Some("https://example.org")
    );
    assert_eq!(link_at(text, 1, 2).unwrap(), None);
    assert!(link_at(text, 3, 4).is_err());
    assert!(link_at(text, 0, 0).is_err());
    assert!(link_at(text, 100, 0).is_err());
}

#[test]
fn supports_references_escapes_and_autolinks_but_ignores_code() {
    let text = "[Course][ml]\n\n[ml]: <./Linear algebra/>\n\n`[fake](./wrong/)`\n\n```\n[fake](./wrong/)\n```\n\n<https://example.org>\n\n[folder](./a\\(b\\)/)";
    assert_eq!(
        link_at(text, 1, 2).unwrap().as_deref(),
        Some("./Linear algebra/")
    );
    assert_eq!(link_at(text, 5, 3).unwrap(), None);
    assert_eq!(link_at(text, 8, 3).unwrap(), None);
    assert_eq!(
        link_at(text, 11, 2).unwrap().as_deref(),
        Some("https://example.org")
    );
    assert_eq!(link_at(text, 13, 3).unwrap().as_deref(), Some("./a(b)/"));
}
