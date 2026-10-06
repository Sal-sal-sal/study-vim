use std::{env, fs, process};

fn main() {
    let mut record = env::current_dir().unwrap().to_string_lossy().into_owned();
    for argument in env::args().skip(1) {
        record.push('\n');
        record.push_str(&argument);
    }
    fs::write(env::var_os("STUDY_EDITOR_RECORD").unwrap(), record).unwrap();
    process::exit(env::var("STUDY_EDITOR_EXIT").unwrap_or_default().parse().unwrap_or(0));
}
