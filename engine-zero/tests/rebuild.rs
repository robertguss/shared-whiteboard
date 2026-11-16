use engine_zero::events::{append, identity};
use engine_zero::ingest::{READERS, Report, read};
use engine_zero::store::{self, text_hash};
use rusqlite::Connection;
use serde_json::{Value, json};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

const SESSION: &str = "sources/laptop/claude/-home-someone-code-garden/session-1.jsonl";
const T3: &str = "sources/laptop/t3/messages.jsonl";
const NOVEMBER: &str = "events/laptop/2026-11.jsonl";
const DESKTOP: &str = "events/desktop/2026-11.jsonl";

struct Setup {
    root: PathBuf,
    db: Connection,
}

impl Setup {
    fn new(name: &str) -> Setup {
        let root =
            std::env::temp_dir().join(format!("engine-zero-rebuild-{}-{name}", std::process::id()));
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

    fn append(&mut self, event: &Value) -> engine_zero::Result<()> {
        append(&mut self.db, &self.root.join("record"), "laptop", event)
    }

    fn read(&mut self) -> Report {
        read(&mut self.db, &self.root.join("record"), READERS, &[], &[]).unwrap()
    }

    fn texts(&self, sql: &str) -> Vec<String> {
        let mut statement = self.db.prepare(sql).unwrap();
        let rows = statement.query_map([], |row| row.get(0)).unwrap();
        rows.collect::<rusqlite::Result<_>>().unwrap()
    }

    fn values(&self, sql: &str) -> Vec<Value> {
        let texts = self.texts(sql);
        texts
            .iter()
            .map(|t| serde_json::from_str(t).unwrap())
            .collect()
    }
}

impl Drop for Setup {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn connection(kind: &str, keys: &[&str]) -> Value {
    let quotes: Vec<Value> = keys
        .iter()
        .enumerate()
        .map(|(i, key)| json!({"key": key, "file": SESSION, "line": i + 1, "text": format!("words from {key}")}))
        .collect();
    json!({"kind": kind, "statement": format!("{kind} across {}", keys.join(" and ")), "quotes": quotes})
}

fn judgement(time: &str, connection: Value, mark: &str) -> Value {
    json!({"kind": "judgement", "time": time, "writer": {"by": "page", "user": "gardener"},
        "connection": connection, "mark": mark})
}

fn recall(time: &str, query: &str) -> Value {
    json!({"kind": "recall", "time": time,
        "writer": {"by": "command", "command": "recall", "caller": "gardener"}, "query": query})
}

fn line(value: &Value) -> String {
    format!("{value}\n")
}

fn prompt(uuid: &str, text: &str) -> String {
    line(&json!({
        "type": "user", "uuid": uuid, "sessionId": "session-1", "timestamp": "2026-11-02T03:04:05.000Z",
        "cwd": "/home/someone/code/garden", "entrypoint": "sdk-ts",
        "message": {"role": "user", "content": text},
    }))
}

fn answer(uuid: &str, text: &str) -> String {
    line(&json!({
        "type": "assistant", "uuid": uuid, "sessionId": "session-1", "timestamp": "2026-11-02T03:04:06.000Z",
        "message": {"role": "assistant", "content": [{"type": "text", "text": text}]},
    }))
}

fn t3_row(text: &str) -> String {
    line(&json!({
        "messageId": format!("t3-{}", text.len()), "threadId": "thread-a", "sessionId": "session-1",
        "createdBy": "user", "creationSource": "web", "createdAt": "2026-11-02T03:04:00.000Z",
        "textSha256": text_hash(text),
    }))
}

#[test]
fn appending_adds_one_line_to_this_months_file_that_records_its_writer() {
    let mut s = Setup::new("append");
    let first = judgement(
        "2026-11-03T08:00:00Z",
        connection("same mechanism", &["claude:rose", "claude:bean"]),
        "yes",
    );
    s.append(&first).unwrap();
    let before = fs::read(s.path(NOVEMBER)).unwrap();
    let second = recall("2026-11-16T09:30:00Z", "compost");
    s.append(&second).unwrap();
    let after = fs::read(s.path(NOVEMBER)).unwrap();
    assert!(after.starts_with(&before));
    let added = String::from_utf8(after[before.len()..].to_vec()).unwrap();
    assert_eq!(added, line(&second));
    assert_eq!(fs::read_dir(s.path("events/laptop")).unwrap().count(), 1);
    let writers: Vec<Value> = s
        .values("SELECT body FROM events ORDER BY line")
        .iter()
        .map(|body| body["writer"].clone())
        .collect();
    assert_eq!(writers, [first["writer"].clone(), second["writer"].clone()]);
}

#[test]
fn appending_refuses_an_event_without_a_writer_an_unknown_kind_or_a_bad_time() {
    let mut s = Setup::new("refuse");
    let mut no_writer = recall("2026-11-16T09:30:00Z", "mulch");
    no_writer.as_object_mut().unwrap().remove("writer");
    let mut unknown = recall("2026-11-16T09:30:00Z", "mulch");
    unknown["kind"] = json!("harvest");
    let mut bad_time = recall("2026-11-16T09:30:00Z", "mulch");
    bad_time["time"] = json!("../../shed");
    for event in [no_writer, unknown, bad_time] {
        assert!(s.append(&event).is_err(), "{event}");
    }
    assert!(!s.path("events").exists());
}

#[test]
fn a_rebuild_gives_the_counts_and_judgements_built_step_by_step() {
    let mut s = Setup::new("rebuild");
    s.write(
        SESSION,
        &(prompt("rose", "Plant the roses")
            + &answer("sun", "Roses like full sun")
            + &prompt("bean", "Stake the beans")),
    );
    s.read();
    s.write(
        T3,
        &(t3_row("Plant the roses") + &t3_row("Stake the beans")),
    );
    s.read();
    let mut judged = vec![];
    for day in 1..=10 {
        let plot = format!("claude:plot-{day}");
        let joined = connection("same mechanism", &[&plot, "claude:rose"]);
        s.append(&judgement(
            &format!("2026-11-{day:02}T10:00:00Z"),
            joined.clone(),
            "yes",
        ))
        .unwrap();
        judged.push(joined);
    }
    for day in 11..=13 {
        s.append(&recall(&format!("2026-11-{day}T10:00:00Z"), "trellis"))
            .unwrap();
    }
    let authors = "SELECT author || ' ' || count(*) FROM items GROUP BY author ORDER BY author";
    let kinds = "SELECT kind || ' ' || count(*) FROM events GROUP BY kind ORDER BY kind";
    let step_by_step = (s.texts(authors), s.texts(kinds));
    assert_eq!(step_by_step.0, ["agent 1", "person 2"]);
    assert_eq!(step_by_step.1, ["judgement 10", "recall 3"]);
    s.db.execute(
        "INSERT INTO events VALUES ('stray', 1, 'recall', NULL, NULL, '{}')",
        [],
    )
    .unwrap();

    let config = s.root.join("config.json");
    let config_json = json!({
        "machine": "laptop", "remote": s.root.join("bucket"), "sources": {},
        "t3_database": s.root.join("t3.sqlite"), "record_folder": s.root.join("record"),
        "database": s.root.join("engine.sqlite"),
    });
    fs::write(&config, config_json.to_string()).unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_engine-zero"))
        .args(["--config", config.to_str().unwrap(), "rebuild"])
        .output()
        .unwrap();
    assert!(out.status.success(), "{out:?}");
    s.db = store::open(&s.root.join("engine.sqlite")).unwrap();

    assert_eq!((s.texts(authors), s.texts(kinds)), step_by_step);
    let held =
        s.values("SELECT json_extract(body, '$.connection') FROM holding_judgements ORDER BY time");
    assert_eq!(held, judged);
}

#[test]
fn events_from_every_machine_are_read_once_and_unknown_kinds_are_skipped() {
    let mut s = Setup::new("machines");
    s.append(&recall("2026-11-16T09:30:00Z", "compost"))
        .unwrap();
    s.write(
        DESKTOP,
        &(line(&recall("2026-11-15T09:30:00Z", "seeds")) + &line(&json!({"kind": "harvest"}))),
    );
    let first = s.read();
    s.read();
    assert_eq!(first.trees["events"].stored, 1);
    assert_eq!(first.trees["events"].skipped["harvest"], 1);
    assert_eq!(
        s.texts("SELECT json_extract(body, '$.query') FROM events ORDER BY file"),
        ["seeds", "compost"]
    );
}

#[test]
fn the_later_judgement_of_one_connection_holds() {
    let mut s = Setup::new("later");
    let joined = connection("same mechanism", &["claude:rose", "claude:bean"]);
    let again = connection(
        "same mechanism",
        &["claude:bean", "claude:rose", "claude:bean"],
    );
    assert_eq!(
        identity(&joined).unwrap(),
        r#"["same mechanism","claude:bean","claude:rose"]"#
    );
    assert_eq!(identity(&again), identity(&joined));
    let other = connection("contradiction", &["claude:rose", "claude:bean"]);
    assert_ne!(identity(&other), identity(&joined));

    s.append(&judgement("2026-11-05T10:00:00Z", again, "no"))
        .unwrap();
    s.append(&judgement("2026-11-04T10:00:00Z", joined.clone(), "yes"))
        .unwrap();
    s.append(&judgement("2026-11-04T10:00:00Z", other, "yes"))
        .unwrap();
    s.write(
        DESKTOP,
        &line(&judgement("2026-11-05T10:00:00Z", joined.clone(), "maybe")),
    );
    s.read();
    let held = s.texts(
        "SELECT connection || ' ' || json_extract(body, '$.mark') FROM holding_judgements ORDER BY connection DESC",
    );
    assert_eq!(
        held,
        [
            format!("{} no", identity(&joined).unwrap()),
            r#"["contradiction","claude:bean","claude:rose"] yes"#.into()
        ]
    );
}

#[test]
fn an_event_appended_while_another_command_holds_the_database_waits_and_is_stored() {
    let mut s = Setup::new("busy");
    let other = Connection::open(s.root.join("engine.sqlite")).unwrap();
    other.execute_batch("BEGIN IMMEDIATE").unwrap();
    let writer = std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(300));
        other.execute_batch("COMMIT").unwrap();
    });
    s.append(&recall("2026-11-03T00:00:00Z", "beds")).unwrap();
    writer.join().unwrap();
    assert_eq!(s.texts("SELECT kind FROM events"), ["recall"]);
}
