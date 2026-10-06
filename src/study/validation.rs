use anyhow::{Result, ensure};

pub fn validate_topic(topic: &str) -> Result<()> {
    ensure!(
        !topic.is_empty() && topic.trim() == topic && !topic.ends_with('.'),
        "Topic must be nonempty and must not begin/end with whitespace or end with a dot"
    );
    ensure!(
        topic.len() <= 249,
        "Topic must fit in a portable filename (249 UTF-8 bytes)"
    );
    ensure!(
        !topic
            .chars()
            .any(|ch| ch.is_control() || "\\/:*?\"<>|".contains(ch)),
        "Topic contains a character forbidden in portable filenames"
    );
    let stem = topic
        .split('.')
        .next()
        .unwrap_or_default()
        .trim_end()
        .to_uppercase();
    let reserved = ["CON", "PRN", "AUX", "NUL", "CONIN$", "CONOUT$"].contains(&stem.as_str())
        || ["COM", "LPT"].iter().any(|prefix| {
            stem.strip_prefix(prefix).is_some_and(|suffix| {
                matches!(
                    suffix,
                    "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "¹" | "²" | "³"
                )
            })
        });
    ensure!(!reserved, "Topic uses a reserved Windows device name");
    Ok(())
}
