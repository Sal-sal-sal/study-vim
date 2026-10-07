use serde_json::{Value, json};
use study_nvim::protocol::serve;

fn request(value: Value) -> Value {
    let mut output = Vec::new();
    serve(value.to_string().as_bytes(), &mut output).unwrap();
    serde_json::from_slice(&output).unwrap()
}

#[test]
fn editor_can_create_topics_and_parse_unsaved_markdown() {
    let root = tempfile::tempdir().unwrap();
    let created = request(json!({"action": "create", "topic": "ML", "root": root.path()}));
    assert_eq!(created["ok"], true);
    let document = &created["data"]["file"];
    let response = request(
        json!({"action": "open", "document": document, "text": "[Course](https://example.org)", "line": 1, "column": 2}),
    );
    assert_eq!(response["data"]["kind"], "url");
    let no_link = request(
        json!({"action": "open", "document": document, "text": "My notes", "line": 1, "column": 2}),
    );
    assert!(no_link["data"].is_null());
}

#[test]
fn protocol_returns_structured_errors() {
    let response = request(json!({"action": "create", "topic": "../escape"}));
    assert_eq!(response["ok"], false);
    assert!(response["error"].as_str().unwrap().contains("forbidden"));
    let mut output = Vec::new();
    serve(b"not json".as_slice(), &mut output).unwrap();
    assert_eq!(
        serde_json::from_slice::<Value>(&output).unwrap()["ok"],
        false
    );
}

#[test]
fn annotations_parse_the_unsaved_editor_text() {
    let response = request(json!({
        "action": "annotations",
        "text": ">! Заметка\n\n- [x] Готово\n\n~~Повторил~~"
    }));
    assert_eq!(response["ok"], true);
    let marks = response["data"].as_array().unwrap();
    assert_eq!(marks.len(), 3);
    assert_eq!(marks[0]["kind"], "note");
    assert_eq!(marks[0]["end_col"], ">! Заметка".len());
    assert_eq!(marks[1]["kind"], "completed");
    assert_eq!(marks[1]["row"], 2);
    assert_eq!(marks[2]["row"], 4);
}
