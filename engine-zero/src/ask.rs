use crate::{Result, copy, events};
use rusqlite::Connection;
use serde_json::{Value, json};
use std::path::Path;

const SEARCH: &str = "
SELECT items.key, author, time, context, tool, session_id, file, line, items.text
FROM items_fts JOIN items ON items.id = items_fts.rowid
WHERE items_fts MATCH ?1 ORDER BY rank LIMIT 20";

pub fn ask(
    db: &mut Connection,
    record: &Path,
    machine: &str,
    caller: &str,
    question: &str,
) -> Result<Vec<String>> {
    if caller.trim().is_empty() {
        return Err(format!(
            "name who is asking: engine-zero ask \"{question}\" --caller \"<your name or agent and project>\""
        )
        .into());
    }
    let words: Vec<String> = question
        .split_whitespace()
        .map(|word| format!("\"{}\"", word.replace('"', "\"\"")))
        .collect();
    if words.is_empty() {
        return Ok(vec![]);
    }
    let items = search(db, &words.join(" "))?;
    let time = copy::time_now()?;
    for item in &items {
        let event = json!({
            "kind": "recall", "time": time,
            "writer": {"by": "command", "command": "ask", "caller": caller},
            "question": question, "key": item["key"], "file": item["file"], "line": item["line"],
        });
        events::append(db, record, machine, &event)?;
    }
    Ok(items.iter().map(Value::to_string).collect())
}

fn search(db: &Connection, query: &str) -> Result<Vec<Value>> {
    let mut statement = db.prepare(SEARCH)?;
    let items = statement.query_map([query], |row| {
        Ok(json!({
            "key": row.get::<_, String>(0)?, "author": row.get::<_, String>(1)?,
            "time": row.get::<_, Option<String>>(2)?, "context": row.get::<_, Option<String>>(3)?,
            "tool": row.get::<_, String>(4)?, "session": row.get::<_, String>(5)?,
            "file": row.get::<_, String>(6)?, "line": row.get::<_, i64>(7)?,
            "text": row.get::<_, String>(8)?,
        }))
    })?;
    Ok(items.collect::<rusqlite::Result<_>>()?)
}
