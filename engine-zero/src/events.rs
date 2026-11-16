use crate::Result;
use crate::ingest::{self, Report};
use regex::Regex;
use rusqlite::Connection;
use serde_json::{Value, json};
use std::collections::BTreeSet;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

pub const KINDS: [&str; 4] = ["proposal", "judgement", "recall", "context_correction"];

pub fn identity(connection: &Value) -> Option<String> {
    let quotes = connection["quotes"].as_array().into_iter().flatten();
    let keys: BTreeSet<&str> = quotes.filter_map(|quote| quote["key"].as_str()).collect();
    let mut parts = vec![connection["kind"].as_str()?];
    parts.extend(keys);
    Some(json!(parts).to_string())
}

pub fn append(db: &mut Connection, record: &Path, machine: &str, event: &Value) -> Result<()> {
    if !KINDS.contains(&event["kind"].as_str().unwrap_or_default()) {
        return Err(format!("event has an unknown kind: {}", event["kind"]).into());
    }
    if !event["writer"].is_object() {
        return Err("event has no writer".into());
    }
    let time = event["time"].as_str().unwrap_or_default();
    if !Regex::new(r"^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z$")?.is_match(time) {
        return Err(format!("event time {time:?} is not YYYY-MM-DDTHH:MM:SSZ").into());
    }
    let folder = record.join("events").join(machine);
    fs::create_dir_all(&folder)?;
    let path = folder.join(format!("{}.jsonl", &time[..7]));
    let mut file = OpenOptions::new().create(true).append(true).open(&path)?;
    file.write_all(format!("{event}\n").as_bytes())?;
    ingest::read_path(db, record, &path, &[], &mut Report::default())
}
