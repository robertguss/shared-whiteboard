use crate::Result;
use crate::mask::mask;
use rusqlite::{Connection, OptionalExtension, Row, params};
use sha2::{Digest, Sha256};
use std::path::Path;

const SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS files (path TEXT PRIMARY KEY, position INTEGER NOT NULL, lines INTEGER NOT NULL);
CREATE TABLE IF NOT EXISTS items (
  id INTEGER PRIMARY KEY, key TEXT NOT NULL UNIQUE, tool TEXT NOT NULL, session_id TEXT NOT NULL,
  author TEXT NOT NULL, time TEXT, cwd TEXT, repo_url TEXT, text TEXT NOT NULL, text_hash TEXT NOT NULL,
  file TEXT NOT NULL, line INTEGER NOT NULL, prompt INTEGER NOT NULL, fallback_author TEXT NOT NULL);
CREATE INDEX IF NOT EXISTS items_file ON items (file);
CREATE INDEX IF NOT EXISTS items_prompt ON items (session_id, text_hash) WHERE prompt;
CREATE VIRTUAL TABLE IF NOT EXISTS items_fts USING fts5 (text, content = 'items', content_rowid = 'id');
CREATE TRIGGER IF NOT EXISTS items_added AFTER INSERT ON items BEGIN
  INSERT INTO items_fts (rowid, text) VALUES (new.id, new.text);
END;
CREATE TRIGGER IF NOT EXISTS items_removed AFTER DELETE ON items BEGIN
  INSERT INTO items_fts (items_fts, rowid, text) VALUES ('delete', old.id, old.text);
END;
CREATE TRIGGER IF NOT EXISTS items_changed AFTER UPDATE OF text ON items BEGIN
  INSERT INTO items_fts (items_fts, rowid, text) VALUES ('delete', old.id, old.text);
  INSERT INTO items_fts (rowid, text) VALUES (new.id, new.text);
END;
CREATE TABLE IF NOT EXISTS t3_messages (
  file TEXT NOT NULL, line INTEGER NOT NULL, message_id TEXT, thread_id TEXT, session_id TEXT,
  created_by TEXT, creation_source TEXT, created_at TEXT, text_hash TEXT, PRIMARY KEY (file, line));
CREATE INDEX IF NOT EXISTS t3_messages_match ON t3_messages (session_id, text_hash);
CREATE TABLE IF NOT EXISTS t3_projects (
  file TEXT NOT NULL, line INTEGER NOT NULL, project_id TEXT, workspace_root TEXT, PRIMARY KEY (file, line));
";

const INSERT_ITEM: &str = "
INSERT INTO items (key, tool, session_id, author, time, cwd, repo_url, text, text_hash, file, line, prompt, fallback_author)
VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)
ON CONFLICT (key) DO UPDATE SET tool = excluded.tool, session_id = excluded.session_id,
  author = excluded.author, time = excluded.time, cwd = excluded.cwd, repo_url = excluded.repo_url,
  text = excluded.text, text_hash = excluded.text_hash, file = excluded.file, line = excluded.line,
  prompt = excluded.prompt, fallback_author = excluded.fallback_author
WHERE (excluded.file, excluded.line) < (items.file, items.line)";

const INSERT_T3_MESSAGE: &str = "
INSERT INTO t3_messages VALUES (?1, ?2, json_extract(?3, '$.messageId'),
  json_extract(?3, '$.threadId'), json_extract(?3, '$.sessionId'), json_extract(?3, '$.createdBy'),
  json_extract(?3, '$.creationSource'), json_extract(?3, '$.createdAt'), json_extract(?3, '$.textSha256'))";

const INSERT_T3_PROJECT: &str = "
INSERT INTO t3_projects VALUES (?1, ?2, json_extract(?3, '$.projectId'),
  json_extract(?3, '$.workspaceRoot'))";

const SETTLE_AUTHORS: &str = "
UPDATE items SET author = coalesce((
  SELECT CASE WHEN count(*) = sum(t.created_by IS 'user') THEN 'person'
              WHEN count(*) = sum(t.created_by IS 'agent') THEN 'other_agent'
              ELSE 'unknown' END
  FROM t3_messages t WHERE t.session_id = items.session_id AND t.text_hash = items.text_hash
  HAVING count(*) > 0), fallback_author)
WHERE prompt";

const SEARCH: &str = "
SELECT items.key, tool, session_id, author, time, cwd, repo_url, items.text, file, line, prompt, fallback_author
FROM items_fts JOIN items ON items.id = items_fts.rowid
WHERE items_fts MATCH ?1 ORDER BY rank LIMIT ?2";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Author {
    Person,
    Agent,
    OtherAgent,
    Tool,
    Unknown,
}

impl Author {
    const ALL: [Author; 5] = [
        Author::Person,
        Author::Agent,
        Author::OtherAgent,
        Author::Tool,
        Author::Unknown,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Author::Person => "person",
            Author::Agent => "agent",
            Author::OtherAgent => "other_agent",
            Author::Tool => "tool",
            Author::Unknown => "unknown",
        }
    }

    fn parse(name: &str) -> Author {
        Author::ALL
            .into_iter()
            .find(|author| author.as_str() == name)
            .unwrap_or(Author::Unknown)
    }
}

pub struct Item {
    pub key: String,
    pub tool: String,
    pub session_id: String,
    pub author: Author,
    pub time: Option<String>,
    pub cwd: Option<String>,
    pub repo_url: Option<String>,
    pub text: String,
    pub file: String,
    pub line: i64,
    pub prompt: bool,
    pub fallback_author: Author,
}

pub fn open(path: &Path) -> Result<Connection> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let db = Connection::open(path)?;
    db.execute_batch(SCHEMA)?;
    Ok(db)
}

pub fn position(db: &Connection, file: &str) -> Result<(u64, i64)> {
    let saved = db
        .query_row(
            "SELECT position, lines FROM files WHERE path = ?1",
            [file],
            |row| Ok((row.get::<_, i64>(0)? as u64, row.get(1)?)),
        )
        .optional()?;
    Ok(saved.unwrap_or((0, 0)))
}

pub fn save_position(db: &Connection, file: &str, position: u64, lines: i64) -> Result<()> {
    db.execute(
        "INSERT OR REPLACE INTO files VALUES (?1, ?2, ?3)",
        params![file, position as i64, lines],
    )?;
    Ok(())
}

pub fn forget(db: &Connection, file: &str) -> Result<()> {
    for table in ["items", "t3_messages", "t3_projects"] {
        db.execute(&format!("DELETE FROM {table} WHERE file = ?1"), [file])?;
    }
    save_position(db, file, 0, 0)
}

pub fn text_hash(text: &str) -> String {
    Sha256::digest(text.trim())
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

pub fn insert_item(db: &Connection, item: &Item) -> Result<()> {
    db.prepare_cached(INSERT_ITEM)?.execute(params![
        item.key,
        item.tool,
        item.session_id,
        item.author.as_str(),
        item.time,
        item.cwd,
        item.repo_url,
        mask(&item.text),
        text_hash(&item.text),
        item.file,
        item.line,
        item.prompt,
        item.fallback_author.as_str(),
    ])?;
    Ok(())
}

pub fn insert_t3_message(db: &Connection, file: &str, line: i64, json: &str) -> Result<()> {
    db.prepare_cached(INSERT_T3_MESSAGE)?
        .execute(params![file, line, json])?;
    Ok(())
}

pub fn insert_t3_project(db: &Connection, file: &str, line: i64, json: &str) -> Result<()> {
    db.prepare_cached(INSERT_T3_PROJECT)?
        .execute(params![file, line, json])?;
    Ok(())
}

pub fn settle_authors(db: &Connection) -> Result<()> {
    db.execute(SETTLE_AUTHORS, [])?;
    Ok(())
}

pub fn count_items(db: &Connection) -> Result<i64> {
    Ok(db.query_row("SELECT count(*) FROM items", [], |row| row.get(0))?)
}

pub fn search(db: &Connection, query: &str, limit: usize) -> Result<Vec<Item>> {
    let mut statement = db.prepare(SEARCH)?;
    let items = statement.query_map(params![query, limit as i64], item)?;
    Ok(items.collect::<rusqlite::Result<_>>()?)
}

fn item(row: &Row) -> rusqlite::Result<Item> {
    Ok(Item {
        key: row.get(0)?,
        tool: row.get(1)?,
        session_id: row.get(2)?,
        author: Author::parse(&row.get::<_, String>(3)?),
        time: row.get(4)?,
        cwd: row.get(5)?,
        repo_url: row.get(6)?,
        text: row.get(7)?,
        file: row.get(8)?,
        line: row.get(9)?,
        prompt: row.get(10)?,
        fallback_author: Author::parse(&row.get::<_, String>(11)?),
    })
}
