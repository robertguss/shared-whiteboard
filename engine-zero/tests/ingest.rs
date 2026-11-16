use engine_zero::ingest::{Outcome, Place, Report, read};
use engine_zero::store::{self, Author, Item};
use rusqlite::Connection;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

const CLAUDE: &str = "sources/laptop/claude/project/one.jsonl";
const T3: &str = "sources/laptop/t3/messages.jsonl";

struct Setup {
    root: PathBuf,
    db: Connection,
}

impl Setup {
    fn new(name: &str) -> Setup {
        let root =
            std::env::temp_dir().join(format!("engine-zero-ingest-{}-{name}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("record")).unwrap();
        let db = store::open(&root.join("engine.sqlite")).unwrap();
        Setup { root, db }
    }

    fn path(&self, relative: &str) -> PathBuf {
        self.root.join("record").join(relative)
    }

    fn write(&self, relative: &str, content: &str) {
        let path = self.path(relative);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, content).unwrap();
    }

    fn append(&self, relative: &str, content: &str) {
        let mut file = OpenOptions::new()
            .append(true)
            .open(self.path(relative))
            .unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }

    fn read(&mut self) -> Report {
        read(&mut self.db, &self.root.join("record"), &[("claude", tiny)]).unwrap()
    }

    fn one(&self, sql: &str) -> String {
        self.db.query_row(sql, [], |row| row.get(0)).unwrap()
    }

    fn count(&self, table: &str) -> i64 {
        self.db
            .query_row(&format!("SELECT COUNT(*) FROM {table}"), [], |row| {
                row.get(0)
            })
            .unwrap()
    }
}

impl Drop for Setup {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn tiny(value: &Value, place: &Place) -> Outcome {
    if value["type"] != "message" {
        return Outcome::Skipped(value["type"].as_str().unwrap_or("untyped").into());
    }
    let fallback = if value["fallback"] == "person" {
        Author::Person
    } else {
        Author::Unknown
    };
    Outcome::Items(vec![Item {
        key: value["key"].as_str().unwrap().into(),
        tool: "claude".into(),
        session_id: value["session"].as_str().unwrap_or("session-1").into(),
        author: Author::Agent,
        time: None,
        cwd: None,
        repo_url: None,
        text: value["text"].as_str().unwrap().into(),
        file: place.file.into(),
        line: place.line,
        prompt: value["prompt"] == true,
        also_sent_as: None,
        fallback_author: fallback,
    }])
}

fn message(key: &str, text: &str) -> String {
    format!("{}\n", json!({"type": "message", "key": key, "text": text}))
}

fn prompt(key: &str, text: &str, fallback: &str) -> String {
    format!(
        "{}\n",
        json!({"type": "message", "key": key, "text": text, "prompt": true, "fallback": fallback})
    )
}

fn t3_row(session: &str, created_by: &str, text: &str) -> String {
    let hash: String = Sha256::digest(text.trim())
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    format!(
        "{}\n",
        json!({"messageId": format!("{session}-{created_by}-{text}"), "threadId": "thread-a",
            "sessionId": session, "createdBy": created_by, "creationSource": "web",
            "createdAt": "2026-01-01T10:00:00.000Z", "textSha256": hash})
    )
}

#[test]
fn a_file_that_grew_by_three_messages_adds_each_once() {
    let mut s = Setup::new("grows");
    s.write(CLAUDE, &(message("a", "one") + &message("b", "two")));
    assert_eq!(s.read().added, 2);
    s.append(
        CLAUDE,
        &(message("c", "three") + &message("d", "four") + &message("e", "five")),
    );
    assert_eq!(s.read().added, 3);
    assert_eq!(s.read().added, 0);
    assert_eq!(s.count("items"), 5);
    assert_eq!(
        s.one("SELECT CAST(line AS TEXT) FROM items WHERE key = 'e'"),
        "5"
    );
}

#[test]
fn a_last_line_without_newline_waits_until_it_is_complete() {
    let mut s = Setup::new("partial");
    let second = message("b", "two");
    s.write(CLAUDE, &(message("a", "one") + second.trim_end()));
    assert_eq!(s.read().added, 1);
    s.append(CLAUDE, "\n");
    assert_eq!(s.read().added, 1);
    assert_eq!(s.read().added, 0);
    assert_eq!(
        s.one("SELECT CAST(line AS TEXT) FROM items WHERE key = 'b'"),
        "2"
    );
}

#[test]
fn read_skips_and_counts_a_broken_line_and_reads_the_lines_after_it() {
    let s = Setup::new("broken");
    s.write(
        T3,
        &(t3_row("session-1", "user", "one") + "{broken\n" + &t3_row("session-1", "user", "two")),
    );
    let config = s.root.join("config.json");
    let config_json = json!({
        "machine": "laptop", "remote": s.root.join("bucket"), "sources": {},
        "t3_database": s.root.join("t3.sqlite"), "record_folder": s.root.join("record"),
        "database": s.root.join("engine.sqlite"),
    });
    fs::write(&config, config_json.to_string()).unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_engine-zero"))
        .args(["--config", config.to_str().unwrap(), "read"])
        .output()
        .unwrap();
    assert!(out.status.success(), "{out:?}");
    assert_eq!(
        String::from_utf8(out.stdout).unwrap(),
        "added 0 items, skipped 1 broken lines\nt3: stored 2\n"
    );
    assert_eq!(s.count("t3_messages"), 2);
}

#[test]
fn the_read_report_counts_records_stored_and_skipped_by_type() {
    let mut s = Setup::new("report");
    s.write(
        CLAUDE,
        &(message("a", "one") + "{\"type\":\"progress\"}\n{\"type\":\"progress\"}\n"),
    );
    let report = s.read();
    let claude = &report.trees["claude"];
    assert_eq!(claude.stored, 1);
    assert_eq!(claude.skipped["progress"], 2);
}

#[test]
fn a_shorter_file_loses_its_rows_and_is_read_from_the_start() {
    let mut s = Setup::new("shorter");
    s.write(
        CLAUDE,
        &(message("a", "one") + &message("b", "two") + &message("c", "three")),
    );
    s.write(
        T3,
        &(t3_row("session-1", "user", "one") + &t3_row("session-1", "user", "two")),
    );
    s.read();
    s.write(CLAUDE, &message("z", "new"));
    s.write(T3, &t3_row("session-1", "agent", "x"));
    s.read();
    assert_eq!(s.count("items"), 1);
    assert_eq!(s.one("SELECT key FROM items"), "z");
    assert_eq!(s.one("SELECT created_by FROM t3_messages"), "agent");
}

#[test]
fn one_key_in_two_files_keeps_the_path_that_sorts_first_in_either_order() {
    let first = "sources/laptop/claude/a/one.jsonl";
    let second = "sources/laptop/claude/b/one.jsonl";
    for (name, order) in [("ab", [first, second]), ("ba", [second, first])] {
        let mut s = Setup::new(name);
        for file in order {
            s.write(file, &message("same", file));
            s.read();
        }
        assert_eq!(s.one("SELECT file FROM items WHERE key = 'same'"), first);
        assert_eq!(s.one("SELECT text FROM items WHERE key = 'same'"), first);
    }
}

#[test]
fn an_api_key_is_stored_masked_and_the_source_file_is_unchanged() {
    let mut s = Setup::new("masked");
    let content = message("k", "use sk-ant-api03-AAAAAAAAAAAAAAAAAAAAAAAA please");
    s.write(CLAUDE, &content);
    s.read();
    assert_eq!(s.one("SELECT text FROM items"), "use [masked] please");
    assert_eq!(fs::read_to_string(s.path(CLAUDE)).unwrap(), content);
}

#[test]
fn a_word_in_a_stored_item_is_found_by_full_text_search() {
    let mut s = Setup::new("search");
    s.write(
        CLAUDE,
        &(message("g", "the giraffe drinks") + &message("h", "an elephant sleeps")),
    );
    s.read();
    let found = store::search(&s.db, "giraffe", 10).unwrap();
    let keys: Vec<&str> = found.iter().map(|item| item.key.as_str()).collect();
    assert_eq!(keys, ["g"]);
}

#[test]
fn prompt_authors_follow_the_t3_rows_present_at_each_read() {
    let mut s = Setup::new("authors");
    s.write(
        CLAUDE,
        &(prompt("gate", "  Fix the gate\n", "unknown")
            + &prompt("roses", "Plant the roses", "person")
            + &prompt("beans", "Water the beans", "unknown")),
    );
    s.write(
        T3,
        &(t3_row("session-1", "agent", "Water the beans")
            + &t3_row("session-2", "user", "Plant the roses")),
    );
    let author =
        |s: &Setup, key: &str| s.one(&format!("SELECT author FROM items WHERE key = '{key}'"));
    s.read();
    assert_eq!(author(&s, "gate"), "unknown");
    assert_eq!(author(&s, "roses"), "person");
    assert_eq!(author(&s, "beans"), "other_agent");

    s.append(T3, &t3_row("session-1", "user", "Fix the gate"));
    s.read();
    assert_eq!(author(&s, "gate"), "person");

    s.append(T3, &t3_row("session-1", "agent", "Fix the gate"));
    s.read();
    assert_eq!(author(&s, "gate"), "unknown");
}
