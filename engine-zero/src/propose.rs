use crate::config::Config;
use crate::{Result, copy, events, store};
use rusqlite::{Connection, OptionalExtension};
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::BTreeSet;
use std::path::Path;

const KINDS: [&str; 6] = [
    "Same mechanism in unrelated domains",
    "One project could solve another's problem",
    "Contradiction between a stated position and a later action",
    "Supersedes",
    "Repeated attempt of",
    "Triggered by",
];

const ITEM: &str = "
SELECT author = 'person' AND time IS NOT NULL, context, text, file, line FROM items WHERE key = ?1";

const JUDGED: &str = "SELECT EXISTS (SELECT 1 FROM holding_judgements WHERE connection = ?1)";

const WAITING: &str =
    "SELECT EXISTS (SELECT 1 FROM events WHERE kind = 'proposal' AND connection = ?1)";

const CANDIDATES: &str = "
SELECT key, time, context, text FROM items
WHERE author = 'person' AND (?1 IS NULL OR time >= ?1) ORDER BY time, file, line";

#[derive(Deserialize)]
struct Proposal {
    kind: String,
    statement: String,
    quotes: Vec<Quote>,
    origin: Option<String>,
}

#[derive(Deserialize)]
struct Quote {
    key: String,
    text: String,
}

struct Cited {
    counts: bool,
    context: Option<String>,
    text: String,
    file: String,
    line: i64,
}

enum Verdict {
    Keep(Value),
    Merge,
    Drop(String),
}

pub fn run(config: &Config, file: &Path, caller: &str) -> Result<()> {
    let folder = config
        .proposals_folder
        .as_ref()
        .ok_or("config has no proposals_folder")?;
    let folder = folder
        .canonicalize()
        .map_err(|e| format!("cannot open the proposals folder {}: {e}", folder.display()))?;
    let resolved = file
        .canonicalize()
        .map_err(|e| format!("cannot read {}: {e}", file.display()))?;
    if !resolved.starts_with(&folder) {
        return Err(format!(
            "{} is outside the proposals folder {}",
            file.display(),
            folder.display()
        )
        .into());
    }
    let lines = std::fs::read_to_string(&resolved)
        .map_err(|e| format!("cannot read {}: {e}", file.display()))?;
    let mut db = store::open(&config.database)?;
    for outcome in check(
        &mut db,
        &config.record_folder,
        &config.machine,
        caller,
        &lines,
    )? {
        println!("{outcome}");
    }
    Ok(())
}

pub fn candidates(db: &Connection, from: Option<&str>) -> Result<Vec<String>> {
    let mut statement = db.prepare(CANDIDATES)?;
    let lines = statement.query_map([from], |row| {
        let item = json!({"key": row.get::<_, String>(0)?, "time": row.get::<_, Option<String>>(1)?,
            "context": row.get::<_, Option<String>>(2)?, "text": row.get::<_, String>(3)?});
        Ok(item.to_string())
    })?;
    Ok(lines.collect::<rusqlite::Result<_>>()?)
}

pub fn check(
    db: &mut Connection,
    record: &Path,
    machine: &str,
    caller: &str,
    lines: &str,
) -> Result<Vec<String>> {
    let time = copy::time_now()?;
    let mut outcomes = vec![];
    for (index, line) in lines.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let outcome = match serde_json::from_str::<Proposal>(line) {
            Err(error) => format!("malformed: {error}"),
            Ok(proposal) => match verdict(db, &proposal)? {
                Verdict::Drop(reason) => format!("dropped: {reason}"),
                Verdict::Merge => "merged".into(),
                Verdict::Keep(connection) => {
                    let mut event = json!({
                        "kind": "proposal", "time": time,
                        "writer": {"by": "command", "command": "propose", "caller": caller},
                        "connection": connection,
                    });
                    if let Some(origin) = proposal.origin {
                        event["origin"] = json!(origin);
                    }
                    events::append(db, record, machine, &event)?;
                    "kept".into()
                }
            },
        };
        outcomes.push(format!("{} {outcome}", index + 1));
    }
    Ok(outcomes)
}

fn verdict(db: &Connection, proposal: &Proposal) -> Result<Verdict> {
    if !KINDS.contains(&proposal.kind.as_str()) {
        let reason = format!("{:?} is not one of the six link kinds", proposal.kind);
        return Ok(Verdict::Drop(reason));
    }
    let mut cited = vec![];
    for (index, quote) in proposal.quotes.iter().enumerate() {
        let item = db.query_row(ITEM, [&quote.key], cited_item).optional()?;
        match item.filter(|item| found(&quote.text, &item.text)) {
            Some(item) => cited.push(item),
            None => {
                let reason = format!("quote {} is not found in item {}", index + 1, quote.key);
                return Ok(Verdict::Drop(reason));
            }
        }
    }
    if let Some(reason) = too_narrow(&proposal.quotes, &cited) {
        return Ok(Verdict::Drop(reason.into()));
    }
    let quotes: Vec<Value> = proposal
        .quotes
        .iter()
        .zip(&cited)
        .map(|(quote, item)| {
            json!({"key": quote.key, "file": item.file, "line": item.line, "text": quote.text})
        })
        .collect();
    let connection =
        json!({"kind": proposal.kind, "statement": proposal.statement, "quotes": quotes});
    let identity = events::identity(&connection);
    if db.query_row(JUDGED, [&identity], |row| row.get(0))? {
        let reason = "this connection has already been judged";
        return Ok(Verdict::Drop(reason.into()));
    }
    if db.query_row(WAITING, [&identity], |row| row.get(0))? {
        return Ok(Verdict::Merge);
    }
    Ok(Verdict::Keep(connection))
}

fn too_narrow(quotes: &[Quote], cited: &[Cited]) -> Option<&'static str> {
    let counted: Vec<(String, &Cited)> = quotes
        .iter()
        .zip(cited)
        .filter(|(_, item)| item.counts)
        .map(|(quote, item)| (collapse(&quote.text), item))
        .collect();
    if counted.len() < 2 {
        return Some("fewer than two quotes are the person's own words with a time");
    }
    let mut once: Vec<(String, &Cited)> = vec![];
    for (text, item) in counted {
        if !once
            .iter()
            .any(|(kept, _)| kept.contains(&text) || text.contains(kept))
        {
            once.push((text, item));
        }
    }
    let contexts: BTreeSet<&str> = once
        .iter()
        .filter_map(|(_, item)| item.context.as_deref())
        .collect();
    if contexts.len() < 2 {
        return Some("the person's quotes come from fewer than two contexts");
    }
    None
}

fn cited_item(row: &rusqlite::Row) -> rusqlite::Result<Cited> {
    Ok(Cited {
        counts: row.get(0)?,
        context: row.get(1)?,
        text: row.get(2)?,
        file: row.get(3)?,
        line: row.get(4)?,
    })
}

fn found(quote: &str, text: &str) -> bool {
    let quote = collapse(quote);
    !quote.is_empty() && collapse(text).contains(&quote)
}

fn collapse(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}
