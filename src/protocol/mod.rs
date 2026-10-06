use crate::{
    markdown::{links::link_at, target::resolve_target},
    study::{config::resolve_root, topic::create_topic},
};
use anyhow::{Context, Result};
use serde::Deserialize;
use serde_json::{Value, json};
use std::{
    io::{Read, Write},
    path::PathBuf,
};

#[derive(Deserialize)]
#[serde(tag = "action", rename_all = "snake_case", deny_unknown_fields)]
enum Request {
    Heading {
        text: String,
        fragment: String,
    },
    Create {
        topic: String,
        root: Option<PathBuf>,
    },
    Open {
        document: PathBuf,
        text: String,
        line: usize,
        column: usize,
    },
}

fn dispatch(request: Request) -> Result<Value> {
    match request {
        Request::Heading { text, fragment } => {
            Ok(json!({ "line": crate::markdown::anchors::heading_line(&text, &fragment)? }))
        }
        Request::Create { topic, root } => {
            let root = resolve_root(root.as_deref())?;
            Ok(serde_json::to_value(create_topic(&root, &topic)?)?)
        }
        Request::Open {
            document,
            text,
            line,
            column,
        } => match link_at(&text, line, column)? {
            Some(destination) => Ok(serde_json::to_value(resolve_target(
                &document,
                &destination,
            )?)?),
            None => Ok(Value::Null),
        },
    }
}

pub fn serve(mut input: impl Read, mut output: impl Write) -> Result<()> {
    let mut text = String::new();
    input
        .read_to_string(&mut text)
        .context("Cannot read editor request")?;
    let result = serde_json::from_str(&text)
        .context("Invalid editor request")
        .and_then(dispatch);
    let response = match result {
        Ok(data) => json!({ "ok": true, "data": data }),
        Err(error) => json!({ "ok": false, "error": format!("{error:#}") }),
    };
    serde_json::to_writer(&mut output, &response).context("Cannot write editor response")?;
    writeln!(output)?;
    Ok(())
}
