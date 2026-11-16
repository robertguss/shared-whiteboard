use engine_zero::events::append;
use engine_zero::ingest::{READERS, read};
use engine_zero::propose::check;
use engine_zero::store;
use rusqlite::Connection;
use serde_json::{Value, json};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const GARDEN: &str = "/home/someone/code/garden";
const POND: &str = "/home/someone/code/pond";
const SKETCH: &str = "/home/someone/.t3/scratch/2026-11-01-sketch";
const DOODLE: &str = "/home/someone/.t3/scratch/2026-11-02-doodle";
const SAME: &str = "Same mechanism in unrelated domains";
const SOLVES: &str = "One project could solve another's problem";

struct Setup {
    root: PathBuf,
    db: Connection,
}

impl Setup {
    fn new(name: &str) -> Setup {
        let root =
            std::env::temp_dir().join(format!("engine-zero-propose-{}-{name}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        let record = root.join("record");
        write(
            &record.join("sources/laptop/claude/garden/garden.jsonl"),
            &[
                prompt(
                    "garden",
                    "rose",
                    Some(GARDEN),
                    "The roses need   more sun\nthan the beans do",
                ),
                prompt(
                    "garden",
                    "bean",
                    Some(GARDEN),
                    "Beans climb whatever stands near them",
                ),
                answer("garden", "agent-rose", "Give the roses six hours of sun"),
            ],
        );
        write(
            &record.join("sources/laptop/claude/pond/pond.jsonl"),
            &[
                prompt(
                    "pond",
                    "pump",
                    Some(POND),
                    "The pond pump hums like a beehive at noon",
                ),
                prompt(
                    "pond",
                    "pasted",
                    Some(POND),
                    "Earlier I wrote The roses need more sun than the beans do",
                ),
                answer("pond", "agent-pump", "Clean the pump filter every spring"),
                line(
                    json!({"type": "user", "uuid": "maybe", "sessionId": "pond", "timestamp": "2026-11-02T03:04:05.000Z",
                "cwd": POND, "message": {"role": "user", "content": "Maybe the pump wants a new filter"}}),
                ),
                line(
                    json!({"type": "user", "uuid": "undated", "sessionId": "pond", "cwd": POND, "promptSource": "typed",
                "message": {"role": "user", "content": "The frogs came back to the pond"}}),
                ),
            ],
        );
        write(
            &record.join("sources/laptop/claude/scratch/sketch.jsonl"),
            &[
                prompt(
                    "sketch",
                    "trellis",
                    Some(SKETCH),
                    "Sketch a trellis that folds flat",
                ),
                prompt(
                    "sketch",
                    "winter",
                    Some(SKETCH),
                    "Fold the trellis away for winter",
                ),
            ],
        );
        write(
            &record.join("sources/laptop/claude/scratch/doodle.jsonl"),
            &[prompt(
                "doodle",
                "maze",
                Some(DOODLE),
                "Doodle a hedge maze for the hens",
            )],
        );
        write(
            &record.join("sources/laptop/claude/drift/drift.jsonl"),
            &[prompt("drift", "hens", None, "The hens like the maze best")],
        );
        let mut db = store::open(&root.join("engine.sqlite")).unwrap();
        read(
            &mut db,
            &record,
            READERS,
            &["/home/someone/code".to_string()],
            &[],
        )
        .unwrap();
        Setup { root, db }
    }

    fn check(&mut self, lines: &[String]) -> Vec<String> {
        check(
            &mut self.db,
            &self.root.join("record"),
            "laptop",
            "gardener",
            &lines.concat(),
        )
        .unwrap()
    }

    fn events(&self, kind: &str) -> Vec<Value> {
        let mut statement = self
            .db
            .prepare("SELECT body FROM events WHERE kind = ?1 ORDER BY line")
            .unwrap();
        let rows = statement
            .query_map([kind], |row| row.get::<_, String>(0))
            .unwrap();
        rows.map(|body| serde_json::from_str(&body.unwrap()).unwrap())
            .collect()
    }

    fn rebuild(&mut self) {
        fs::remove_file(self.root.join("engine.sqlite")).unwrap();
        self.db = store::open(&self.root.join("engine.sqlite")).unwrap();
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

fn prompt(session: &str, uuid: &str, cwd: Option<&str>, text: &str) -> String {
    let mut value = json!({"type": "user", "uuid": uuid, "sessionId": session, "promptSource": "typed",
        "timestamp": "2026-11-02T03:04:05.000Z", "message": {"role": "user", "content": text}});
    if let Some(cwd) = cwd {
        value["cwd"] = json!(cwd);
    }
    line(value)
}

fn answer(session: &str, uuid: &str, text: &str) -> String {
    line(
        json!({"type": "assistant", "uuid": uuid, "sessionId": session, "timestamp": "2026-11-02T03:04:06.000Z",
        "message": {"role": "assistant", "content": [{"type": "text", "text": text}]}}),
    )
}

fn proposal(kind: &str, quotes: &[(&str, &str)]) -> String {
    let quotes: Vec<Value> = quotes
        .iter()
        .map(|(key, text)| json!({"key": format!("claude:{key}"), "text": text}))
        .collect();
    line(
        json!({"kind": kind, "statement": "the garden and the pond share a rhythm", "quotes": quotes}),
    )
}

const ROSE: (&str, &str) = ("rose", "The roses need more sun than the beans do");
const PUMP: (&str, &str) = ("pump", "pond pump hums like a beehive");
const TRELLIS: (&str, &str) = ("trellis", "a trellis that folds flat");
const MAZE: (&str, &str) = ("maze", "Doodle a hedge maze");

#[test]
fn each_proposal_is_kept_merged_dropped_or_reported_malformed_by_the_fixed_rules() {
    let mut s = Setup::new("rules");
    let kept = json!({"kind": SAME, "statement": "the garden and the pond share a rhythm", "origin": "evening pass",
        "quotes": [{"key": "claude:rose", "text": ROSE.1}, {"key": "claude:pump", "text": PUMP.1}]});
    let judgement_shaped = json!({"kind": "judgement", "time": "2026-11-03T08:00:00Z", "writer": {"by": "page", "user": "gardener"},
        "statement": "the garden and the pond share a rhythm", "mark": "not real",
        "quotes": [{"key": "claude:rose", "text": ROSE.1}, {"key": "claude:pump", "text": PUMP.1}],
        "connection": {"kind": SAME, "quotes": [{"key": "claude:rose"}, {"key": "claude:pump"}]}});
    let lines = [
        line(kept),
        proposal(
            SAME,
            &[
                ("agent-rose", "six hours of sun"),
                ("agent-pump", "Clean the pump filter"),
            ],
        ),
        proposal(SAME, &[("rose", "The roses need more shade"), PUMP]),
        proposal(SAME, &[("nobody", "The roses need"), PUMP]),
        proposal(SAME, &[ROSE, ("bean", "Beans climb")]),
        proposal(SOLVES, &[PUMP, TRELLIS]),
        "\n".to_string(),
        proposal(SOLVES, &[TRELLIS, MAZE]),
        proposal(SOLVES, &[TRELLIS, ("winter", "Fold the trellis away")]),
        proposal(SAME, &[ROSE, ("maybe", "Maybe the pump")]),
        proposal(SAME, &[ROSE, ("undated", "The frogs came back")]),
        proposal(SAME, &[ROSE, ("hens", "The hens like the maze")]),
        proposal(
            SAME,
            &[
                ("rose", "The roses need more sun"),
                ("pasted", "Earlier I wrote The roses need more sun"),
            ],
        ),
        proposal(
            SAME,
            &[
                ("rose", "roses  need\tmore sun than"),
                ("pasted", "I wrote The roses need more sun than the beans"),
            ],
        ),
        proposal("Looks alike", &[ROSE, PUMP]),
        line(judgement_shaped),
        "{\"kind\": \"Supersedes\", \"quotes\": oops}\n".to_string(),
        line(json!({"kind": "Supersedes", "quotes": []})),
        proposal(SAME, &[PUMP, ROSE]),
        proposal(SAME, &[ROSE, ("pump", " \n ")]),
    ];
    let outcomes = s.check(&lines);
    let fewer_counted = "dropped: fewer than two quotes are the person's own words with a time";
    let one_context = "dropped: the person's quotes come from fewer than two contexts";
    let expected = [
        "1 kept".to_string(),
        format!("2 {fewer_counted}"),
        "3 dropped: quote 1 is not found in item claude:rose".to_string(),
        "4 dropped: quote 1 is not found in item claude:nobody".to_string(),
        format!("5 {one_context}"),
        "6 kept".to_string(),
        "8 kept".to_string(),
        format!("9 {one_context}"),
        format!("10 {fewer_counted}"),
        format!("11 {fewer_counted}"),
        format!("12 {one_context}"),
        format!("13 {one_context}"),
        format!("14 {one_context}"),
        "15 dropped: \"Looks alike\" is not one of the six link kinds".to_string(),
        "16 dropped: \"judgement\" is not one of the six link kinds".to_string(),
    ];
    assert_eq!(outcomes[..15], expected);
    assert!(
        outcomes[15].starts_with("17 malformed: "),
        "{}",
        outcomes[15]
    );
    assert!(
        outcomes[16].starts_with("18 malformed: ") && outcomes[16].contains("statement"),
        "{}",
        outcomes[16]
    );
    assert_eq!(
        outcomes[17..],
        [
            "19 merged",
            "20 dropped: quote 2 is not found in item claude:pump"
        ]
    );

    assert!(s.events("judgement").is_empty());
    let proposals = s.events("proposal");
    assert_eq!(proposals.len(), 3);
    let first = &proposals[0];
    assert_eq!(first["kind"], "proposal");
    assert_eq!(
        first["writer"],
        json!({"by": "command", "command": "propose", "caller": "gardener"})
    );
    assert_eq!(
        first["connection"],
        json!({"kind": SAME, "statement": "the garden and the pond share a rhythm", "quotes": [
            {"key": "claude:rose", "file": "sources/laptop/claude/garden/garden.jsonl", "line": 1, "text": ROSE.1},
            {"key": "claude:pump", "file": "sources/laptop/claude/pond/pond.jsonl", "line": 1, "text": PUMP.1},
        ]})
    );
    assert_eq!(first["origin"], "evening pass");
    assert!(first["time"].as_str().unwrap().ends_with('Z'));
    assert!(proposals[1].get("origin").is_none());
}

#[test]
fn a_waiting_connection_is_merged_and_a_judged_one_is_dropped_with_any_mark_also_after_a_rebuild() {
    let mut s = Setup::new("judged");
    let lines = [
        proposal(SAME, &[ROSE, PUMP]),
        proposal(SOLVES, &[PUMP, TRELLIS]),
        proposal(SOLVES, &[TRELLIS, MAZE]),
    ];
    assert_eq!(s.check(&lines), ["1 kept", "2 kept", "3 kept"]);
    assert_eq!(s.check(&lines), ["1 merged", "2 merged", "3 merged"]);
    let proposals = s.events("proposal");
    assert_eq!(proposals.len(), 3);

    let record = s.root.join("record");
    for (proposal, mark) in
        proposals
            .iter()
            .zip(["real and new", "real and already known", "not real"])
    {
        let judgement = json!({"kind": "judgement", "time": "2026-11-03T08:00:00Z", "writer": {"by": "page", "user": "gardener"},
            "connection": proposal["connection"], "mark": mark});
        append(&mut s.db, &record, "laptop", &judgement).unwrap();
    }
    let judged = "dropped: this connection has already been judged";
    let expected = [
        format!("1 {judged}"),
        format!("2 {judged}"),
        format!("3 {judged}"),
    ];
    assert_eq!(s.check(&lines), expected);
    s.rebuild();
    assert_eq!(s.check(&lines), expected);
    assert_eq!(s.events("proposal").len(), 3);
}

struct Cli {
    root: PathBuf,
    config: Value,
}

impl Cli {
    fn new(name: &str) -> Cli {
        let root = std::env::temp_dir().join(format!(
            "engine-zero-propose-cli-{}-{name}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&root);
        write(
            &root.join("record/sources/laptop/claude/garden/garden.jsonl"),
            &[prompt(
                "garden",
                "rose",
                Some(GARDEN),
                "The roses need more sun than the beans do",
            )],
        );
        write(
            &root.join("record/sources/laptop/claude/pond/pond.jsonl"),
            &[prompt(
                "pond",
                "pump",
                Some(POND),
                "The pond pump hums like a beehive at noon",
            )],
        );
        let config = json!({
            "machine": "laptop", "remote": root.join("bucket"), "sources": {},
            "t3_database": root.join("t3.sqlite"), "record_folder": root.join("record"),
            "database": root.join("engine.sqlite"), "proposals_folder": root.join("proposals"),
            "project_parents": ["/home/someone/code"],
        });
        Cli { root, config }
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

    fn propose(&self, file: &Path) -> Output {
        self.run(&["propose", file.to_str().unwrap(), "--caller", "gardener"])
    }
}

impl Drop for Cli {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

#[test]
fn propose_reads_only_files_inside_the_proposals_folder() {
    let mut c = Cli::new("folder");
    assert!(c.run(&["read"]).status.success());
    let entry = [proposal(SAME, &[ROSE, PUMP])];
    let inside = c.root.join("proposals/evening.jsonl");
    let outside = c.root.join("outside.jsonl");
    write(&inside, &entry);
    write(&outside, &entry);
    let link = c.root.join("proposals/link.jsonl");
    std::os::unix::fs::symlink(&outside, &link).unwrap();
    for path in [
        outside.clone(),
        c.root.join("proposals/../outside.jsonl"),
        link,
    ] {
        let out = c.propose(&path);
        assert!(!out.status.success());
        assert!(
            String::from_utf8_lossy(&out.stderr).contains("outside the proposals folder"),
            "{out:?}"
        );
        assert!(out.stdout.is_empty());
    }
    assert!(!c.root.join("record/events").exists());

    let out = c.propose(&inside);
    assert!(out.status.success(), "{out:?}");
    assert_eq!(String::from_utf8_lossy(&out.stdout), "1 kept\n");
    let month = fs::read_dir(c.root.join("record/events/laptop"))
        .unwrap()
        .next()
        .unwrap()
        .unwrap();
    let event: Value =
        serde_json::from_str(fs::read_to_string(month.path()).unwrap().trim()).unwrap();
    assert_eq!(event["writer"]["caller"], "gardener");

    let missing = c.propose(&c.root.join("proposals/none.jsonl"));
    assert!(!missing.status.success());

    c.config.as_object_mut().unwrap().remove("proposals_folder");
    assert!(c.run(&["read"]).status.success());
    let out = c.propose(&inside);
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("proposals_folder"));

    let repository = Path::new(env!("CARGO_MANIFEST_DIR")).join("../private-test");
    c.config["proposals_folder"] = json!(repository.join("proposals"));
    let out = c.run(&["read"]);
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("repository"));
}
