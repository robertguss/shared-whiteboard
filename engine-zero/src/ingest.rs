use crate::Result;
use crate::config::Config;
use crate::store::{self, Item};
use crate::{claude, codex, events};
use rusqlite::Connection;
use serde_json::Value;
use std::collections::BTreeMap;
use std::fmt;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

pub struct Place<'a> {
    pub file: &'a str,
    pub line: i64,
    pub head: &'a Value,
}

pub enum Outcome {
    Items(Vec<Item>),
    Skipped(String),
}

pub type Reader = fn(&Value, &Place) -> Outcome;

pub const READERS: &[(&str, Reader)] = &[("claude", claude::read), ("codex", codex::read)];

#[derive(Default)]
pub struct Counts {
    pub stored: usize,
    pub skipped: BTreeMap<String, usize>,
}

#[derive(Default)]
pub struct Report {
    pub added: i64,
    pub broken: usize,
    pub trees: BTreeMap<String, Counts>,
}

impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (added, broken) = (self.added, self.broken);
        writeln!(f, "added {added} items, skipped {broken} broken lines")?;
        for (tree, counts) in &self.trees {
            write!(f, "{tree}: stored {}", counts.stored)?;
            for (kind, count) in &counts.skipped {
                write!(f, ", skipped {kind} {count}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

enum Handler {
    Session(Reader),
    T3Messages,
    T3Projects,
    Events,
}

pub fn run(config: &Config) -> Result<()> {
    let mut db = store::open(&config.database)?;
    print!("{}", read(&mut db, &config.record_folder, READERS)?);
    Ok(())
}

pub fn rebuild(config: &Config) -> Result<()> {
    match fs::remove_file(&config.database) {
        Err(error) if error.kind() != std::io::ErrorKind::NotFound => Err(error.into()),
        _ => run(config),
    }
}

pub fn read(db: &mut Connection, record: &Path, readers: &[(&str, Reader)]) -> Result<Report> {
    let mut report = Report::default();
    let before = store::count_items(db)?;
    let mut paths = vec![];
    walk(record, &mut paths)?;
    paths.sort();
    for path in paths {
        read_path(db, record, &path, readers, &mut report)?;
    }
    store::settle_authors(db)?;
    report.added = store::count_items(db)? - before;
    Ok(report)
}

pub fn read_path(
    db: &mut Connection,
    record: &Path,
    path: &Path,
    readers: &[(&str, Reader)],
    report: &mut Report,
) -> Result<()> {
    let file = path.strip_prefix(record)?.to_string_lossy().into_owned();
    if let Some((tree, handler)) = handler(&file, readers) {
        let tx = db.transaction()?;
        read_file(&tx, path, &file, tree, handler, report)?;
        tx.commit()?;
    }
    Ok(())
}

fn walk(dir: &Path, paths: &mut Vec<PathBuf>) -> Result<()> {
    let entries = fs::read_dir(dir).map_err(|e| format!("cannot read {}: {e}", dir.display()))?;
    for entry in entries {
        let path = entry?.path();
        if path.is_dir() {
            walk(&path, paths)?;
        } else if path.extension().is_some_and(|e| e == "jsonl") {
            paths.push(path);
        }
    }
    Ok(())
}

fn handler<'a>(file: &str, readers: &[(&'a str, Reader)]) -> Option<(&'a str, Handler)> {
    match file.split('/').collect::<Vec<_>>().as_slice() {
        ["sources", _, "t3", "messages.jsonl"] => Some(("t3", Handler::T3Messages)),
        ["sources", _, "t3", "projects.jsonl"] => Some(("t3", Handler::T3Projects)),
        ["events", _, _] => Some(("events", Handler::Events)),
        ["sources", _, tree, _, ..] => readers
            .iter()
            .find(|(name, _)| name == tree)
            .map(|&(name, reader)| (name, Handler::Session(reader))),
        _ => None,
    }
}

fn read_file(
    db: &Connection,
    path: &Path,
    file: &str,
    tree: &str,
    handler: Handler,
    report: &mut Report,
) -> Result<()> {
    let (mut position, mut line) = store::position(db, file)?;
    let mut source = File::open(path)?;
    let length = source.metadata()?.len();
    if length < position {
        store::forget(db, file)?;
        (position, line) = (0, 0);
    }
    let mut bytes = vec![];
    BufReader::new(&source).read_until(b'\n', &mut bytes)?;
    let head = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
    bytes.clear();
    source.seek(SeekFrom::Start(position))?;
    source.take(length - position).read_to_end(&mut bytes)?;
    let complete = bytes.iter().rposition(|&b| b == b'\n').map_or(0, |i| i + 1);
    for raw in bytes[..complete].split_inclusive(|&b| b == b'\n') {
        line += 1;
        let Ok(value) = serde_json::from_slice::<Value>(raw) else {
            report.broken += 1;
            continue;
        };
        let counts = report.trees.entry(tree.to_string()).or_default();
        let place = Place {
            file,
            line,
            head: &head,
        };
        match handler {
            Handler::T3Messages => store::insert_t3_message(db, file, line, &value.to_string())?,
            Handler::T3Projects => store::insert_t3_project(db, file, line, &value.to_string())?,
            Handler::Events if !events::KINDS.contains(&value["kind"].as_str().unwrap_or("")) => {
                let kind = value["kind"].as_str().unwrap_or("untyped").to_string();
                *counts.skipped.entry(kind).or_default() += 1;
                continue;
            }
            Handler::Events => {
                let connection = events::identity(&value["connection"]);
                store::insert_event(db, file, line, &value.to_string(), connection.as_deref())?
            }
            Handler::Session(reader) => match reader(&value, &place) {
                Outcome::Items(items) => {
                    for item in &items {
                        store::insert_item(db, item)?;
                    }
                }
                Outcome::Skipped(kind) => {
                    *counts.skipped.entry(kind).or_default() += 1;
                    continue;
                }
            },
        }
        counts.stored += 1;
    }
    store::save_position(db, file, position + complete as u64, line)
}
