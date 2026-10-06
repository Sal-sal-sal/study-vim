use std::fs;
use study_nvim::markdown::target::{Target, resolve_target};

#[test]
fn resolves_local_links_relative_to_the_document() {
    let root = tempfile::tempdir().unwrap();
    let document = root.path().join("ML.study");
    fs::create_dir(root.path().join("Linear algebra")).unwrap();
    fs::write(root.path().join("notes.md"), "# Intro").unwrap();
    assert_eq!(
        resolve_target(&document, "Linear%20algebra/").unwrap(),
        Target::Directory {
            path: root.path().join("Linear algebra/")
        }
    );
    assert_eq!(
        resolve_target(&document, "notes.md#intro").unwrap(),
        Target::File {
            path: root.path().join("notes.md"),
            fragment: Some("intro".into())
        }
    );
    assert_eq!(
        resolve_target(&document, "#intro").unwrap(),
        Target::Anchor {
            fragment: "intro".into()
        }
    );
    assert!(resolve_target(&document, "missing/").is_err());
}

#[test]
fn opens_web_links_without_executing_other_schemes() {
    let document = std::path::Path::new("ML.study");
    assert!(matches!(
        resolve_target(document, "https://example.org/a#b").unwrap(),
        Target::Url { .. }
    ));
    for link in [
        "javascript:alert(1)",
        "file:///tmp/a",
        "data:text/plain,a",
        "",
        "%FF",
        "a%00b",
    ] {
        assert!(resolve_target(document, link).is_err(), "{link}");
    }
}
