use study_nvim::study::validation::validate_topic;

#[test]
fn accepts_useful_portable_names() {
    for topic in [
        "ML",
        "Linear algebra",
        "rust-basics",
        "Матанализ",
        "A.B",
        "CONcepts",
    ] {
        assert!(validate_topic(topic).is_ok(), "{topic}");
    }
}

#[test]
fn rejects_traversal_and_windows_reserved_names_on_every_os() {
    for topic in [
        "", ".", "..", "../ML", "a/b", "a\\b", "C:ML", "ML ", " ML", "ML.", "a\n", "x?", "NUL",
        "con.txt", "COM1", "lpt9.md", "COM¹",
    ] {
        assert!(validate_topic(topic).is_err(), "{topic:?}");
    }
    assert!(validate_topic(&"x".repeat(250)).is_err());
}
