use std::fs;
use study_nvim::study::topic::create_topic;

#[test]
fn creates_a_plan_and_preserves_existing_notes() {
    let root = tempfile::tempdir().unwrap();
    let topic = create_topic(root.path(), "ML").unwrap();
    assert_eq!(topic.file, root.path().join("ML/ML.study"));
    assert_eq!(
        fs::read_to_string(&topic.file).unwrap(),
        "# ML\n\n## Ссылки\n\n## Папки\n"
    );
    fs::write(&topic.file, "My learning progress").unwrap();
    create_topic(root.path(), "ML").unwrap();
    assert_eq!(
        fs::read_to_string(topic.file).unwrap(),
        "My learning progress"
    );
}

#[test]
fn reports_collisions_and_rejects_invalid_names_before_writing() {
    let root = tempfile::tempdir().unwrap();
    fs::create_dir_all(root.path().join("ML/ML.study")).unwrap();
    assert!(create_topic(root.path(), "ML").is_err());
    assert!(create_topic(root.path(), "../escape").is_err());
    fs::write(root.path().join("Rust"), "not a directory").unwrap();
    assert!(create_topic(root.path(), "Rust").is_err());
}
