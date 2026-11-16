use crate::Result;
use rusqlite::{Connection, OpenFlags, Row};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

const MESSAGES: &str = "
SELECT m.message_id, m.thread_id, json_extract(p.payload_json, '$.nativeThreadRef.nativeId'),
       json_extract(m.payload_json, '$.createdBy'), json_extract(m.payload_json, '$.creationSource'),
       m.created_at, json_extract(m.payload_json, '$.text')
FROM orchestration_v2_projection_messages m
LEFT JOIN orchestration_v2_projection_runs r ON r.run_id = m.run_id
LEFT JOIN orchestration_v2_projection_provider_threads p ON p.provider_thread_id = r.provider_thread_id
WHERE m.role = 'user'
ORDER BY m.created_at, m.message_id";

const PROJECTS: &str =
    "SELECT project_id, workspace_root FROM projection_projects ORDER BY project_id";

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Message {
    message_id: String,
    thread_id: String,
    session_id: Option<String>,
    created_by: Option<String>,
    creation_source: Option<String>,
    created_at: String,
    text_sha256: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Project {
    project_id: String,
    workspace_root: String,
}

pub fn extract(database: &Path, out_dir: &Path) -> Result<()> {
    if !database.exists() {
        return Ok(());
    }
    let read = || -> rusqlite::Result<_> {
        let db = Connection::open_with_flags(database, OpenFlags::SQLITE_OPEN_READ_ONLY)?;
        let messages = query(&db, MESSAGES, |row| {
            let text: Option<String> = row.get(6)?;
            Ok(Message {
                message_id: row.get(0)?,
                thread_id: row.get(1)?,
                session_id: row.get(2)?,
                created_by: row.get(3)?,
                creation_source: row.get(4)?,
                created_at: row.get(5)?,
                text_sha256: text.map(|t| hex(&Sha256::digest(t.trim()))),
            })
        })?;
        let projects = query(&db, PROJECTS, |row| {
            Ok(Project {
                project_id: row.get(0)?,
                workspace_root: row.get(1)?,
            })
        })?;
        Ok((messages, projects))
    };
    let (messages, projects) =
        read().map_err(|e| format!("T3 database {}: {e}", database.display()))?;
    fs::create_dir_all(out_dir)?;
    append_new(&out_dir.join("messages.jsonl"), &messages)?;
    append_new(&out_dir.join("projects.jsonl"), &projects)
}

fn query<T>(
    db: &Connection,
    sql: &str,
    map: impl FnMut(&Row) -> rusqlite::Result<T>,
) -> rusqlite::Result<Vec<T>> {
    db.prepare(sql)?.query_map([], map)?.collect()
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn append_new<T: Serialize>(path: &Path, rows: &[T]) -> Result<()> {
    let existing = fs::read_to_string(path).unwrap_or_default();
    let seen: HashSet<&str> = existing.lines().collect();
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    for row in rows {
        let line = serde_json::to_string(row)?;
        if !seen.contains(line.as_str()) {
            writeln!(file, "{line}")?;
        }
    }
    Ok(())
}
