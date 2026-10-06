use std::{fs, process::Command};

#[test]
fn cli_opens_the_plan_in_the_topic_directory_and_reports_editor_errors() {
    let root = tempfile::tempdir().unwrap();
    let editor = root
        .path()
        .join(format!("editor{}", std::env::consts::EXE_SUFFIX));
    assert!(
        Command::new("rustc")
            .arg("tests/fixtures/editor.rs")
            .arg("-o")
            .arg(&editor)
            .status()
            .unwrap()
            .success()
    );
    let record = root.path().join("record");
    let learning = root.path().join("learning");
    let mut command = Command::new(env!("CARGO_BIN_EXE_study"));
    command
        .arg("Linear algebra")
        .env("STUDY_ROOT", &learning)
        .env("STUDY_NVIM", &editor)
        .env("STUDY_EDITOR_RECORD", &record);
    assert!(command.status().unwrap().success());
    let plan = learning.join("Linear algebra/Linear algebra.study");
    let contents = fs::read_to_string(&record).unwrap();
    let parts: Vec<_> = contents.lines().collect();
    assert_eq!(
        fs::canonicalize(parts[0]).unwrap(),
        fs::canonicalize(plan.parent().unwrap()).unwrap()
    );
    assert_eq!(parts[1], "--");
    assert_eq!(std::path::Path::new(parts[2]), plan);
    fs::write(&plan, "My notes").unwrap();
    command.env("STUDY_EDITOR_EXIT", "7");
    let failure = command.output().unwrap();
    assert!(!failure.status.success());
    assert!(String::from_utf8_lossy(&failure.stderr).contains("Neovim exited"));
    assert_eq!(fs::read_to_string(plan).unwrap(), "My notes");
}

#[test]
fn cli_reports_missing_or_invalid_arguments() {
    let binary = env!("CARGO_BIN_EXE_study");
    assert!(!Command::new(binary).output().unwrap().status.success());
    assert!(
        !Command::new(binary)
            .arg("../bad")
            .output()
            .unwrap()
            .status
            .success()
    );
    let help = Command::new(binary).arg("--help").output().unwrap();
    assert!(help.status.success());
    assert!(String::from_utf8_lossy(&help.stdout).contains("STUDY_ROOT"));
}
