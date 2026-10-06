use study_nvim::markdown::anchors::heading_line;

#[test]
fn finds_markdown_headings_and_duplicate_slugs() {
    let text = "# ML\n\n## Ссылки\n\n## Linear **algebra**\n\n## Linear algebra\n\nNamed {#custom}\n---\n\n```\n# fake\n```";
    assert_eq!(heading_line(text, "ссылки").unwrap(), 3);
    assert_eq!(heading_line(text, "linear-algebra").unwrap(), 5);
    assert_eq!(heading_line(text, "linear-algebra-1").unwrap(), 7);
    assert_eq!(heading_line(text, "custom").unwrap(), 9);
    assert_eq!(heading_line(text, "").unwrap(), 1);
    assert!(heading_line(text, "fake").is_err());
}
