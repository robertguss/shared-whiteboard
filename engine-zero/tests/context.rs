use engine_zero::ingest::{READERS, read};
use engine_zero::store;
use serde_json::{Value, json};
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};

const PROJECTS: &str = "sources/laptop/t3/projects.jsonl";
const EVENTS: &str = "events/laptop/2026-11.jsonl";

struct Setup {
    root: PathBuf,
    config: Value,
}

impl Setup {
    fn new(name: &str) -> Setup {
        let root =
            std::env::temp_dir().join(format!("engine-zero-context-{}-{name}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("record")).unwrap();
        let config = json!({
            "machine": "laptop", "remote": root.join("bucket"), "sources": {},
            "t3_database": root.join("t3.sqlite"), "record_folder": root.join("record"),
            "database": root.join("engine.sqlite"),
        });
        Setup { root, config }
    }

    fn write(&self, relative: &str, lines: &[String]) {
        let path = self.root.join("record").join(relative);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, lines.concat()).unwrap();
    }

    fn read(&self, parents: &[String], temporary: &[String]) {
        let mut db = store::open(&self.root.join("engine.sqlite")).unwrap();
        read(
            &mut db,
            &self.root.join("record"),
            READERS,
            parents,
            temporary,
        )
        .unwrap();
    }

    fn run(&self, args: &[&str]) -> Output {
        let file = self.root.join("config.json");
        fs::write(&file, self.config.to_string()).unwrap();
        Command::new(env!("CARGO_BIN_EXE_engine-zero"))
            .args(["--config", file.to_str().unwrap()])
            .args(args)
            .output()
            .unwrap()
    }

    fn contexts(&self) -> Vec<(String, String)> {
        let db = store::open(&self.root.join("engine.sqlite")).unwrap();
        let mut statement = db
            .prepare(
                "SELECT session_id, coalesce(context, 'unknown') FROM items ORDER BY session_id",
            )
            .unwrap();
        statement
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
            .unwrap()
            .map(Result::unwrap)
            .collect()
    }
}

impl Drop for Setup {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn claude(session: &str, cwd: Option<&str>) -> String {
    let mut value = json!({
        "type": "user", "uuid": format!("{session}-prompt"), "sessionId": session,
        "timestamp": "2026-11-02T03:04:05.000Z", "message": {"role": "user", "content": "Water the beds"},
    });
    if let Some(cwd) = cwd {
        value["cwd"] = json!(cwd);
    }
    format!("{value}\n")
}

fn codex(session: &str, cwd: &str, repo_url: &str) -> Vec<String> {
    let meta = json!({"timestamp": "2026-11-02T03:04:05.000Z", "type": "session_meta", "ordinal": 0,
        "payload": {"id": session, "cwd": cwd, "originator": "codex-tui", "git": {"repository_url": repo_url}}});
    let item = json!({"type": "UserMessage", "id": "item-1", "content": [{"type": "text", "text": "Turn the compost"}]});
    let message = json!({"timestamp": "2026-11-02T03:04:06.000Z", "type": "event_msg", "ordinal": 1,
        "payload": {"type": "item_completed", "turn_id": "turn-1", "item": item}});
    vec![format!("{meta}\n"), format!("{message}\n")]
}

fn project(root: &str) -> String {
    format!("{}\n", json!({"projectId": root, "workspaceRoot": root}))
}

fn pairs(expected: &[(&str, &str)]) -> Vec<(String, String)> {
    expected
        .iter()
        .map(|(session, context)| (session.to_string(), context.to_string()))
        .collect()
}

#[test]
fn each_working_directory_gets_the_context_the_ordered_rule_gives() {
    let s = Setup::new("rule");
    let sessions = [
        (
            "s01",
            Some("/home/someone/.t3/worktrees/garden/feature-roses"),
        ),
        (
            "s02",
            Some("/home/someone/.t3/worktrees/garden/feature-beans/src"),
        ),
        (
            "s03",
            Some("/home/someone/code/garden/.claude/worktrees/tulips"),
        ),
        (
            "s04",
            Some("/home/someone/code/garden/.claude/worktrees/lilies/docs"),
        ),
        ("s05", Some("/home/someone/code/garden/beds/north")),
        ("s06", Some("/home/someone/code/garden/shed/tools")),
        ("s07", Some("/home/someone/code/gardenia")),
        ("s08", Some("/home/someone/Projects/orchard")),
        ("s09", Some("/home/someone/Projects/orchard/trees")),
        ("s10", Some("/home/someone/.t3/scratch/2026-11-01-sketch")),
        ("s11", Some("/home/someone/.t3/scratch/2026-11-02-doodle")),
        ("s12", Some("/tmp/seedlings/tray")),
        ("s13", None),
    ];
    for (session, cwd) in sessions {
        s.write(
            &format!("sources/laptop/claude/p/{session}.jsonl"),
            &[claude(session, cwd)],
        );
    }
    s.write(
        "sources/laptop/codex/2026/11/02/rollout-c1.jsonl",
        &codex(
            "c1",
            "/home/someone/code/garden",
            "https://example.com/someone/allotment.git",
        ),
    );
    s.write(
        "sources/laptop/codex/2026/11/02/rollout-c2.jsonl",
        &codex(
            "c2",
            "/home/someone/elsewhere/pond",
            "git@example.com:someone/pond-pump.git",
        ),
    );
    let parents = ["/home/someone/Projects".to_string()];
    let temporary = ["/tmp".to_string()];

    s.read(&parents, &temporary);
    assert!(s.contexts().contains(&pairs(&[("s05", "session:s05")])[0]));

    s.write(
        PROJECTS,
        &[
            project("/home/someone/code/garden"),
            project("/home/someone/code/garden/shed"),
            project("/home/someone/.t3/scratch"),
            project("/tmp/seedlings"),
        ],
    );
    s.read(&parents, &temporary);
    assert_eq!(
        s.contexts(),
        pairs(&[
            ("c1", "garden"),
            ("c2", "pond-pump"),
            ("s01", "garden"),
            ("s02", "garden"),
            ("s03", "garden"),
            ("s04", "garden"),
            ("s05", "garden"),
            ("s06", "shed"),
            ("s07", "session:s07"),
            ("s08", "orchard"),
            ("s09", "orchard"),
            ("s10", "session:s10"),
            ("s11", "session:s11"),
            ("s12", "session:s12"),
            ("s13", "unknown"),
        ])
    );
}

#[test]
fn a_correction_relabels_the_items_under_its_prefix_reaches_the_bucket_and_holds_after_a_rebuild() {
    let mut s = Setup::new("correction");
    let sessions = [
        ("s1", "/home/someone/code/garden/beds"),
        ("s2", "/home/someone/code/garden/beds/north"),
        ("s3", "/home/someone/code/garden/.claude/worktrees/tulips"),
        ("s4", "/home/someone/code/pond"),
    ];
    for (session, cwd) in sessions {
        s.write(
            &format!("sources/laptop/claude/p/{session}.jsonl"),
            &[claude(session, Some(cwd))],
        );
    }
    assert!(s.run(&["read"]).status.success());
    let blocked = s.root.join("blocked");
    fs::write(&blocked, "not a folder").unwrap();
    s.config["remote"] = json!(blocked);

    let failed = s.run(&[
        "context",
        "set",
        "/home/someone/code/garden",
        "flowerbed",
        "--caller",
        "gardener",
    ]);
    assert!(!failed.status.success());
    assert!(String::from_utf8_lossy(&failed.stderr).contains("bucket"));
    let local = s.root.join("record").join(EVENTS);
    let event: Value = serde_json::from_str(fs::read_to_string(&local).unwrap().trim()).unwrap();
    assert_eq!(event["kind"], "context_correction");
    assert_eq!(
        event["writer"],
        json!({"by": "command", "command": "context set", "caller": "gardener"})
    );
    assert_eq!(
        s.contexts(),
        pairs(&[
            ("s1", "flowerbed"),
            ("s2", "flowerbed"),
            ("s3", "flowerbed"),
            ("s4", "session:s4"),
        ])
    );

    s.config["remote"] = json!(s.root.join("bucket"));
    for (prefix, name) in [
        ("/home/someone/code/garden/beds/north", "north-bed"),
        ("/home/someone/code/garden", "lawn"),
    ] {
        let out = s.run(&["context", "set", prefix, name, "--caller", "gardener"]);
        assert!(out.status.success(), "{out:?}");
    }
    let expected = pairs(&[
        ("s1", "lawn"),
        ("s2", "north-bed"),
        ("s3", "lawn"),
        ("s4", "session:s4"),
    ]);
    assert_eq!(s.contexts(), expected);
    assert_eq!(
        fs::read_to_string(s.root.join("bucket").join(EVENTS)).unwrap(),
        fs::read_to_string(&local).unwrap()
    );

    assert!(s.run(&["rebuild"]).status.success());
    assert_eq!(s.contexts(), expected);
}
