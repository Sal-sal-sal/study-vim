use std::path::Path;
use study_nvim::study::config::root_path;

#[test]
fn uses_the_current_users_home() {
    let home = Path::new("other-user");
    assert_eq!(
        root_path(None, Some(home), Path::new("workspace")).unwrap(),
        Path::new("workspace/other-user/study")
    );
}

#[test]
fn supports_absolute_relative_and_home_paths() {
    let base = tempfile::tempdir().unwrap();
    let home = base.path().join("alice");
    let absolute = base.path().join("learning");
    assert_eq!(
        root_path(Some(&absolute), None, base.path()).unwrap(),
        absolute
    );
    assert_eq!(
        root_path(Some(Path::new("learning")), None, base.path()).unwrap(),
        base.path().join("learning")
    );
    assert_eq!(
        root_path(Some(Path::new("~/courses")), Some(&home), base.path()).unwrap(),
        home.join("courses")
    );
    assert!(root_path(None, None, base.path()).is_err());
    assert!(root_path(Some(Path::new("")), Some(&home), base.path()).is_err());
}
