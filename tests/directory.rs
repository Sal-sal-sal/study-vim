use study_nvim::study::directory::browse;

#[test]
fn directories_are_listed_first_in_a_stable_order() {
    let root = tempfile::tempdir().unwrap();
    std::fs::create_dir(root.path().join("z-folder")).unwrap();
    std::fs::write(root.path().join("B.study"), "").unwrap();
    std::fs::write(root.path().join("a.study"), "").unwrap();
    let directory = browse(root.path()).unwrap();
    let names: Vec<_> = directory
        .entries
        .iter()
        .map(|entry| entry.name.as_str())
        .collect();
    assert_eq!(names, ["z-folder", "a.study", "B.study"]);
    assert!(directory.entries[0].directory);
    assert_eq!(directory.parent.as_deref(), root.path().parent());
    assert!(browse(&root.path().join("a.study")).is_err());
}
