use crate::ingest::{Outcome, Place};
use crate::store::{Author, Item};
use serde_json::Value;

pub fn read(record: &Value, place: &Place) -> Outcome {
    let meta = &place.head["payload"];
    let ordinal = record["ordinal"].as_i64();
    let start = meta["subagent_history_start_ordinal"].as_i64();
    if ordinal.zip(start).is_some_and(|(o, s)| o < s)
        || record["metadata"]["inherited_user_message"] == true
    {
        return Outcome::Skipped("inherited".into());
    }
    let payload = &record["payload"];
    let kind = kind(record);
    let own_agent = meta["agent_path"].as_str().unwrap_or("/root");
    let (author, prompt, content) = match kind {
        "AgentMessage" => (Author::Agent, false, &payload["item"]["content"]),
        "agent_message" if payload["recipient"] == own_agent => {
            (Author::OtherAgent, false, &payload["content"])
        }
        "UserMessage" if meta["originator"] == "codex-tui" => {
            (Author::Person, true, &payload["item"]["content"])
        }
        "UserMessage" => (Author::Unknown, true, &payload["item"]["content"]),
        _ => return Outcome::Skipped(kind.into()),
    };
    let (Some(session), Some(ordinal)) = (meta["id"].as_str(), ordinal) else {
        return Outcome::Skipped("no_key".into());
    };
    let Some(text) = text(content) else {
        return Outcome::Skipped("no_text".into());
    };
    Outcome::Items(vec![Item {
        key: format!("codex:{session}:{ordinal}"),
        tool: "codex".into(),
        session_id: session.into(),
        author,
        time: record["timestamp"].as_str().map(Into::into),
        cwd: meta["cwd"].as_str().map(Into::into),
        repo_url: meta["git"]["repository_url"].as_str().map(Into::into),
        text,
        file: place.file.into(),
        line: place.line,
        prompt,
        fallback_author: author,
        also_sent_as: None,
    }])
}

fn kind(record: &Value) -> &str {
    let payload = &record["payload"];
    let kind = match record["type"].as_str() {
        Some("event_msg") if payload["type"] == "item_completed" => &payload["item"]["type"],
        Some("event_msg" | "response_item") => &payload["type"],
        _ => &record["type"],
    };
    kind.as_str().unwrap_or("untyped")
}

fn text(content: &Value) -> Option<String> {
    let texts: Vec<&str> = content
        .as_array()?
        .iter()
        .filter_map(|part| part["text"].as_str())
        .collect();
    (!texts.is_empty()).then(|| texts.join("\n"))
}
