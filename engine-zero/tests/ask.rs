use engine_zero::ask::ask;
use engine_zero::ingest::{READERS, read};
use engine_zero::store;
use rusqlite::Connection;
use serde_json::{Value, json};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const GARDEN: &str = "/home/someone/code/garden";
const BEDS: &str = "sources/laptop/claude/garden/garden.jsonl";

struct Setup {
    root: PathBuf,
    db: Connection,
}

impl Setup {
    fn new(name: &str) -> Setup {
        let root =
            std::env::temp_dir().join(format!("engine-zero-ask-{}-{name}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        write(
            &root.join("record").join(BEDS),
            &[
                prompt("rose", "The roses need more sun than the beans do"),
                answer(
                    "agent-rose",
                    "Give the roses six hours of sun, key sk-FAKEFAKEFAKEFAKEFAKEFAKE",
                ),
                line(
                    json!({"type": "user", "uuid": "note", "sessionId": "garden", "cwd": GARDEN,
                    "timestamp": "2026-11-02T03:04:07.000Z", "origin": {"kind": "task-notification"},
                    "message": {"role": "user", "content": "<task-notification>The roses were watered at dawn</task-notification>"}}),
                ),
                prompt("bed", "The sun-loving herbs go in the south bed"),
            ],
        );
        let db = store::open(&root.join("engine.sqlite")).unwrap();
        let mut s = Setup { root, db };
        s.read();
        s
    }

    fn read(&mut self) {
        let parents = ["/home/someone/code".to_string()];
        read(
            &mut self.db,
            &self.root.join("record"),
            READERS,
            &parents,
            &[],
        )
        .unwrap();
    }

    fn ask(&mut self, question: &str) -> Vec<Value> {
        let record = self.root.join("record");
        ask(&mut self.db, &record, "laptop", "gardener", question)
            .unwrap()
            .iter()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect()
    }

    fn recalls(&self) -> Vec<Value> {
        let mut statement = self
            .db
            .prepare("SELECT body FROM events WHERE kind = 'recall' ORDER BY file, line")
            .unwrap();
        let rows = statement
            .query_map([], |row| row.get::<_, String>(0))
            .unwrap();
        rows.map(|body| serde_json::from_str(&body.unwrap()).unwrap())
            .collect()
    }
}

impl Drop for Setup {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn write(path: &Path, lines: &[String]) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, lines.concat()).unwrap();
}

fn line(value: Value) -> String {
    format!("{value}\n")
}

fn prompt(uuid: &str, text: &str) -> String {
    line(
        json!({"type": "user", "uuid": uuid, "sessionId": "garden", "promptSource": "typed", "cwd": GARDEN,
        "timestamp": "2026-11-02T03:04:05.000Z", "message": {"role": "user", "content": text}}),
    )
}

fn answer(uuid: &str, text: &str) -> String {
    line(
        json!({"type": "assistant", "uuid": uuid, "sessionId": "garden", "timestamp": "2026-11-02T03:04:06.000Z",
        "message": {"role": "assistant", "content": [{"type": "text", "text": text}]}}),
    )
}

fn keys(items: &[Value]) -> Vec<&str> {
    let mut keys: Vec<&str> = items.iter().map(|i| i["key"].as_str().unwrap()).collect();
    keys.sort();
    keys
}

#[test]
fn each_returned_item_names_its_author_and_place_and_writes_one_recall_that_survives_a_rebuild() {
    let mut s = Setup::new("recall");
    let items = s.ask("roses");
    assert_eq!(
        keys(&items),
        ["claude:agent-rose", "claude:note", "claude:rose"]
    );
    let by_key = |key: &str| items.iter().find(|i| i["key"] == key).unwrap().clone();
    assert_eq!(
        by_key("claude:rose"),
        json!({"key": "claude:rose", "author": "person", "time": "2026-11-02T03:04:05.000Z",
            "context": "garden", "tool": "claude", "session": "garden", "file": BEDS, "line": 1,
            "text": "The roses need more sun than the beans do"})
    );
    let agent = by_key("claude:agent-rose");
    assert_eq!(agent["author"], "agent");
    assert_eq!(
        agent["text"],
        "Give the roses six hours of sun, key [masked]"
    );
    let tool = by_key("claude:note");
    assert_eq!(tool["author"], "tool");
    assert_eq!(tool["line"], 3);

    let recalls = s.recalls();
    assert_eq!(recalls.len(), 3);
    for (recall, item) in recalls.iter().zip(&items) {
        assert_eq!(recall["kind"], "recall");
        assert_eq!(recall["time"], recalls[0]["time"]);
        assert!(recall["time"].as_str().unwrap().ends_with('Z'));
        assert_eq!(
            recall["writer"],
            json!({"by": "command", "command": "ask", "caller": "gardener"})
        );
        assert_eq!(recall["question"], "roses");
        assert_eq!(
            [&recall["key"], &recall["file"], &recall["line"]],
            [&item["key"], &item["file"], &item["line"]]
        );
    }

    assert!(s.ask("giraffe").is_empty());
    assert_eq!(s.recalls().len(), 3);

    fs::remove_file(s.root.join("engine.sqlite")).unwrap();
    s.db = store::open(&s.root.join("engine.sqlite")).unwrap();
    s.read();
    assert_eq!(s.recalls(), recalls);
}

#[test]
fn a_question_with_quotes_hyphens_or_operators_returns_results_or_none_and_never_fails() {
    let mut s = Setup::new("operators");
    assert_eq!(keys(&s.ask("sun-loving")), ["claude:bed"]);
    assert_eq!(keys(&s.ask("\"roses\" beans")), ["claude:rose"]);
    assert_eq!(keys(&s.ask("beans: roses\"")), ["claude:rose"]);
    for question in [
        "roses OR",
        "text:roses",
        "*",
        "^sun",
        "(roses",
        "NEAR(roses beans)",
        "\"",
        "-",
    ] {
        s.ask(question);
    }
    for question in ["NOT roses", "", "   "] {
        assert!(s.ask(question).is_empty(), "{question}");
    }

    let hedges: Vec<String> = (0..25)
        .map(|n| prompt(&format!("hedge-{n}"), &format!("Trim hedge number {n}")))
        .collect();
    write(
        &s.root
            .join("record/sources/laptop/claude/hedge/hedge.jsonl"),
        &hedges,
    );
    s.read();
    assert_eq!(s.ask("hedge").len(), 20);
}

#[test]
fn ask_without_a_caller_is_refused_with_how_to_name_one() {
    let root = std::env::temp_dir().join(format!("engine-zero-ask-cli-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    write(
        &root.join("record").join(BEDS),
        &[prompt("rose", "The roses need more sun than the beans do")],
    );
    let config = json!({
        "machine": "laptop", "remote": root.join("bucket"), "sources": {},
        "t3_database": root.join("t3.sqlite"), "record_folder": root.join("record"),
        "database": root.join("engine.sqlite"),
    });
    let file = root.join("config.json");
    fs::write(&file, config.to_string()).unwrap();
    let run = |args: &[&str]| -> Output {
        Command::new(env!("CARGO_BIN_EXE_engine-zero"))
            .args(["--config", file.to_str().unwrap()])
            .args(args)
            .output()
            .unwrap()
    };
    assert!(run(&["read"]).status.success());

    let refusal = "name who is asking: engine-zero ask \"roses beans\" --caller \"<your name or agent and project>\"";
    for args in [
        &["ask", "roses", "beans"][..],
        &["ask", "roses", "beans", "--caller", ""],
    ] {
        let out = run(args);
        assert!(!out.status.success());
        assert!(
            String::from_utf8_lossy(&out.stderr).contains(refusal),
            "{out:?}"
        );
        assert!(out.stdout.is_empty());
    }
    assert!(!root.join("record/events").exists());

    let out = run(&["ask", "roses", "beans", "--caller", "gardener"]);
    assert!(out.status.success(), "{out:?}");
    let item: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(item["key"], "claude:rose");
    assert!(root.join("record/events/laptop").exists());
    let _ = fs::remove_dir_all(&root);
}
