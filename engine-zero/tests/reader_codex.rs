use engine_zero::ingest::{READERS, Report, read};
use engine_zero::store::{self, text_hash};
use rusqlite::Connection;
use serde_json::{Value, json};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

const TOP: &str = "sources/vm/codex/2026/01/02/rollout-2026-01-02T03-04-05-session-1.jsonl";
const OTHER: &str = "sources/vm/codex/2026/01/02/rollout-2026-01-02T04-05-06-session-2.jsonl";
const FORK: &str = "sources/vm/codex/2026/01/02/rollout-2026-01-02T05-06-07-fork-1.jsonl";
const T3: &str = "sources/vm/t3/messages.jsonl";

struct Setup {
    root: PathBuf,
    db: Connection,
}

impl Setup {
    fn new(name: &str) -> Setup {
        let root =
            std::env::temp_dir().join(format!("engine-zero-codex-{}-{name}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("record")).unwrap();
        let db = store::open(&root.join("engine.sqlite")).unwrap();
        Setup { root, db }
    }

    fn write(&self, relative: &str, lines: &[String]) {
        let path = self.root.join("record").join(relative);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, lines.concat()).unwrap();
    }

    fn append(&self, relative: &str, line: &str) {
        let path = self.root.join("record").join(relative);
        let mut file = OpenOptions::new().append(true).open(path).unwrap();
        file.write_all(line.as_bytes()).unwrap();
    }

    fn read(&mut self) -> Report {
        read(&mut self.db, &self.root.join("record"), READERS).unwrap()
    }

    fn authors(&self) -> Vec<(String, String)> {
        let mut statement = self
            .db
            .prepare("SELECT key, author FROM items ORDER BY key")
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

fn pairs(expected: &[(&str, &str)]) -> Vec<(String, String)> {
    expected
        .iter()
        .map(|(key, author)| (format!("codex:{key}"), author.to_string()))
        .collect()
}

fn record(ordinal: i64, kind: &str, payload: Value, extra: Value) -> String {
    let mut value = json!({
        "timestamp": "2026-01-02T03:04:05.000Z", "type": kind, "ordinal": ordinal, "payload": payload,
    });
    for (field, v) in extra.as_object().unwrap() {
        value[field] = v.clone();
    }
    format!("{value}\n")
}

fn meta(id: &str, extra: Value) -> String {
    let mut payload = json!({
        "id": id, "cwd": "/home/someone/code/garden", "originator": "T3 Code",
        "git": {"branch": "main", "repository_url": "https://example.com/someone/garden.git"},
    });
    for (field, v) in extra.as_object().unwrap() {
        payload[field] = v.clone();
    }
    record(0, "session_meta", payload, json!({}))
}

fn item(ordinal: i64, kind: &str, part: &str, text: &str) -> String {
    let item = json!({"type": kind, "id": format!("item-{ordinal}"), "content": [{"type": part, "text": text}]});
    record(
        ordinal,
        "event_msg",
        json!({"type": "item_completed", "turn_id": "turn-1", "item": item}),
        json!({}),
    )
}

fn user_message(ordinal: i64, text: &str) -> String {
    item(ordinal, "UserMessage", "text", text)
}

fn agent_reply(ordinal: i64, text: &str) -> String {
    item(ordinal, "AgentMessage", "Text", text)
}

fn message(ordinal: i64, role: &str, text: &str, extra: Value) -> String {
    let part = if role == "assistant" {
        "output_text"
    } else {
        "input_text"
    };
    let payload =
        json!({"type": "message", "role": role, "content": [{"type": part, "text": text}]});
    record(ordinal, "response_item", payload, extra)
}

fn agent_message(ordinal: i64, recipient: &str, text: &str, extra: Value) -> String {
    let payload = json!({
        "type": "agent_message", "author": "/root/planner", "recipient": recipient,
        "content": [{"type": "input_text", "text": text}, {"type": "encrypted_content", "encrypted_content": "fake"}],
    });
    record(ordinal, "response_item", payload, extra)
}

fn t3_row(session: &str, created_by: &str, text: &str) -> String {
    let row = json!({
        "messageId": format!("t3-{created_by}-{}", text.len()), "threadId": "thread-a",
        "sessionId": session, "createdBy": created_by, "creationSource": "web",
        "createdAt": "2026-01-02T03:04:00.000Z", "textSha256": text_hash(text),
    });
    format!("{row}\n")
}

#[test]
fn a_user_message_takes_its_author_from_the_matching_t3_row() {
    let mut s = Setup::new("t3-match");
    s.write(
        TOP,
        &[
            meta("session-1", json!({})),
            user_message(1, "Plant the roses by the gate"),
            user_message(2, "Water the beans"),
        ],
    );
    s.write(
        T3,
        &[
            t3_row("session-1", "user", "Plant the roses by the gate"),
            t3_row("session-1", "agent", "Water the beans"),
        ],
    );
    s.read();
    assert_eq!(
        s.authors(),
        pairs(&[("session-1:1", "person"), ("session-1:2", "other_agent")])
    );
}

#[test]
fn without_a_t3_row_only_a_terminal_prompt_is_the_persons() {
    let mut s = Setup::new("terminal");
    s.write(
        TOP,
        &[
            meta("session-1", json!({"originator": "codex-tui"})),
            user_message(1, "Prune the pear tree"),
        ],
    );
    s.write(
        OTHER,
        &[
            meta("session-2", json!({"originator": "codex_exec"})),
            user_message(1, "Prune the plum tree"),
        ],
    );
    s.read();
    assert_eq!(
        s.authors(),
        pairs(&[("session-1:1", "person"), ("session-2:1", "unknown")])
    );
}

#[test]
fn a_user_message_read_before_its_t3_row_becomes_the_persons_later_and_resumes_in_its_session() {
    let mut s = Setup::new("t3-later");
    s.write(
        TOP,
        &[
            meta("session-1", json!({})),
            user_message(1, "Fix the gate"),
        ],
    );
    s.write(
        T3,
        &[t3_row("session-1", "user", "Something else entirely")],
    );
    s.read();
    assert_eq!(s.authors(), pairs(&[("session-1:1", "unknown")]));
    s.append(T3, &t3_row("session-1", "user", "Fix the gate"));
    s.append(TOP, &user_message(2, "Oil the hinge"));
    s.append(T3, &t3_row("session-1", "user", "Oil the hinge"));
    s.read();
    assert_eq!(
        s.authors(),
        pairs(&[("session-1:1", "person"), ("session-1:2", "person")])
    );
}

#[test]
fn a_turn_stores_its_prompt_once_and_its_reply_and_nothing_injected_or_internal() {
    let mut s = Setup::new("turn");
    let prompt = "Mulch the strawberries";
    let input_order = json!({"metadata": {"user_input_order": 0}});
    s.write(
        TOP,
        &[
            meta("session-1", json!({})),
            message(1, "developer", "<permissions>garden rules</permissions>", json!({})),
            message(2, "user", "<environment_context>a shed</environment_context>", json!({})),
            message(3, "user", prompt, input_order),
            user_message(4, prompt),
            record(5, "response_item", json!({"type": "reasoning", "summary": []}), json!({})),
            record(6, "response_item", json!({"type": "function_call", "name": "shell", "arguments": "{}"}), json!({})),
            item(7, "Reasoning", "text", "thinking about mulch"),
            message(8, "assistant", "The strawberries are mulched.", json!({})),
            agent_reply(9, "The strawberries are mulched."),
            agent_message(10, "/root", "The planner finished the beds", json!({})),
            record(11, "compacted", json!({"message": "", "replacement_history": [{"type": "message", "role": "user", "content": [{"type": "input_text", "text": prompt}]}]}), json!({})),
        ],
    );
    let report = s.read();
    assert_eq!(
        s.authors(),
        pairs(&[
            ("session-1:10", "other_agent"),
            ("session-1:4", "unknown"),
            ("session-1:9", "agent"),
        ])
    );
    let skipped = &report.trees["codex"].skipped;
    assert_eq!(skipped["message"], 4);
    for kind in [
        "session_meta",
        "reasoning",
        "function_call",
        "Reasoning",
        "compacted",
    ] {
        assert_eq!(skipped[kind], 1, "{kind}");
    }
}

#[test]
fn a_fork_skips_its_copied_history_and_keeps_messages_addressed_to_its_agent() {
    let mut s = Setup::new("fork");
    s.write(
        TOP,
        &[
            meta("session-1", json!({})),
            agent_reply(1, "The beds are dug"),
        ],
    );
    let inherited = json!({"metadata": {"inherited_user_message": true}});
    s.write(
        FORK,
        &[
            meta(
                "fork-1",
                json!({"forked_from_id": "session-1", "agent_path": "/root/gardener", "subagent_history_start_ordinal": 4}),
            ),
            record(1, "session_meta", json!({"id": "session-1"}), json!({})),
            agent_reply(2, "The beds are dug"),
            agent_message(3, "/root/gardener", "Dig the beds", inherited.clone()),
            agent_message(4, "/root/gardener", "Now sow the carrots", json!({})),
            agent_reply(5, "The carrots are sown"),
            agent_message(6, "/root", "Carrots done", json!({})),
            agent_message(7, "/root/gardener", "Dig the beds", inherited),
        ],
    );
    let report = s.read();
    assert_eq!(
        s.authors(),
        pairs(&[
            ("fork-1:4", "other_agent"),
            ("fork-1:5", "agent"),
            ("session-1:1", "agent"),
        ])
    );
    let skipped = &report.trees["codex"].skipped;
    assert_eq!((skipped["inherited"], skipped["agent_message"]), (5, 1));
}

#[test]
fn an_item_is_keyed_by_session_and_ordinal_and_carries_its_place() {
    let mut s = Setup::new("place");
    s.write(
        TOP,
        &[
            meta("session-1", json!({})),
            agent_reply(1, "Hello"),
            agent_reply(2, "Again"),
        ],
    );
    s.read();
    let row: (String, String, String, String, String, i64, String) =
        s.db.query_row(
            "SELECT tool, session_id, time, cwd, repo_url, line, file FROM items WHERE key = 'codex:session-1:2'",
            [],
            |r| {
                Ok((
                    r.get(0)?,
                    r.get(1)?,
                    r.get(2)?,
                    r.get(3)?,
                    r.get(4)?,
                    r.get(5)?,
                    r.get(6)?,
                ))
            },
        )
        .unwrap();
    assert_eq!(
        row,
        (
            "codex".into(),
            "session-1".into(),
            "2026-01-02T03:04:05.000Z".into(),
            "/home/someone/code/garden".into(),
            "https://example.com/someone/garden.git".into(),
            3,
            TOP.into()
        )
    );
}
