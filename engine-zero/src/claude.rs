use crate::ingest::{Outcome, Place};
use crate::store::{Author, Item};
use serde_json::Value;

const TOOL_WRAPPERS: [&str; 2] = ["<task-notification>", "<local-command-stdout>"];
const COMMAND_WRAPPERS: [&str; 2] = ["<command-message>", "<command-name>"];
const INTERRUPTED: &str = "[Request interrupted by user";

pub fn read(record: &Value, place: &Place) -> Outcome {
    let kind = record["type"].as_str().unwrap_or("untyped");
    if kind != "user" && kind != "assistant" {
        return Outcome::Skipped(kind.into());
    }
    if kind == "user"
        && let Some(reason) = not_a_message(record)
    {
        return Outcome::Skipped(reason.into());
    }
    let Some(uuid) = record["uuid"].as_str() else {
        return Outcome::Skipped("no_uuid".into());
    };
    let Some(text) = text(&record["message"]["content"]) else {
        return Outcome::Skipped("no_text".into());
    };
    if kind == "user" && text.trim_start().starts_with(INTERRUPTED) {
        return Outcome::Skipped("interrupted".into());
    }
    let (author, prompt) = match kind {
        "assistant" => (Author::Agent, false),
        _ => user_author(record, place.file, &text),
    };
    let (text, also_sent_as) = match typed_command(&text) {
        Some((slash, dollar)) if prompt => (slash, Some(dollar)),
        _ => (text, None),
    };
    Outcome::Items(vec![Item {
        key: format!("claude:{uuid}"),
        tool: "claude".into(),
        session_id: record["sessionId"].as_str().unwrap_or_default().into(),
        author,
        time: record["timestamp"].as_str().map(Into::into),
        cwd: record["cwd"].as_str().map(Into::into),
        repo_url: None,
        text,
        file: place.file.into(),
        line: place.line,
        prompt,
        fallback_author: author,
        also_sent_as,
    }])
}

fn not_a_message(record: &Value) -> Option<&'static str> {
    let content = &record["message"]["content"];
    let tool_result = content
        .as_array()
        .is_some_and(|blocks| blocks.iter().any(|b| b["type"] == "tool_result"));
    if tool_result
        || record.get("toolUseResult").is_some()
        || record.get("sourceToolAssistantUUID").is_some()
    {
        Some("tool_result")
    } else if record["isMeta"] == true {
        Some("meta")
    } else if record["isCompactSummary"] == true {
        Some("compact_summary")
    } else {
        None
    }
}

fn text(content: &Value) -> Option<String> {
    if let Some(text) = content.as_str() {
        return Some(text.into());
    }
    let texts: Vec<&str> = content
        .as_array()?
        .iter()
        .filter(|b| b["type"] == "text")
        .filter_map(|b| b["text"].as_str())
        .collect();
    (!texts.is_empty()).then(|| texts.join("\n"))
}

fn user_author(record: &Value, file: &str, text: &str) -> (Author, bool) {
    let origin = record["origin"]["kind"].as_str();
    if file.split('/').any(|part| part == "subagents")
        || matches!(origin, Some("coordinator" | "peer"))
    {
        return (Author::OtherAgent, false);
    }
    let opening = text.trim_start();
    if origin == Some("task-notification") || TOOL_WRAPPERS.iter().any(|w| opening.starts_with(w)) {
        return (Author::Tool, false);
    }
    let human = origin == Some("human")
        || record["promptSource"] == "typed"
        || record["turnOrigin"] == "human";
    let fallback = if human {
        Author::Person
    } else {
        Author::Unknown
    };
    (fallback, true)
}

fn typed_command(text: &str) -> Option<(String, String)> {
    let opening = text.trim_start();
    if !COMMAND_WRAPPERS.iter().any(|w| opening.starts_with(w)) {
        return None;
    }
    let name = between(text, "command-name")?
        .trim()
        .trim_start_matches('/');
    let args = between(text, "command-args").unwrap_or_default();
    let typed = |sign: char| format!("{sign}{name} {args}").trim().to_string();
    Some((typed('/'), typed('$')))
}

fn between<'a>(text: &'a str, tag: &str) -> Option<&'a str> {
    let (_, rest) = text.split_once(&format!("<{tag}>"))?;
    let (inner, _) = rest.split_once(&format!("</{tag}>"))?;
    Some(inner)
}
