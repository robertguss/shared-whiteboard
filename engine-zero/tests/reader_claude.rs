use engine_zero::ingest::{READERS, Report, read};
use engine_zero::store::{self, text_hash};
use rusqlite::Connection;
use serde_json::{Value, json};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

const MAIN: &str = "sources/vm/claude/-home-someone-code-garden/session-1.jsonl";
const SUBAGENT: &str =
    "sources/vm/claude/-home-someone-code-garden/session-1/subagents/agent-a1.jsonl";
const T3: &str = "sources/vm/t3/messages.jsonl";

struct Setup {
    root: PathBuf,
    db: Connection,
}

impl Setup {
    fn new(name: &str) -> Setup {
        let root =
            std::env::temp_dir().join(format!("engine-zero-claude-{}-{name}", std::process::id()));
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
        read(&mut self.db, &self.root.join("record"), READERS, &[], &[]).unwrap()
    }

    fn author(&self, uuid: &str) -> String {
        self.db
            .query_row(
                "SELECT author FROM items WHERE key = ?1",
                [format!("claude:{uuid}")],
                |row| row.get(0),
            )
            .unwrap()
    }

    fn count(&self) -> i64 {
        store::count_items(&self.db).unwrap()
    }
}

impl Drop for Setup {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn record(kind: &str, uuid: &str, content: Value, extra: Value) -> String {
    let mut value = json!({
        "type": kind, "uuid": uuid, "parentUuid": null, "sessionId": "session-1",
        "timestamp": "2026-01-02T03:04:05.000Z", "cwd": "/home/someone/code/garden",
        "entrypoint": "sdk-ts", "message": {"role": kind, "content": content},
    });
    for (field, v) in extra.as_object().unwrap() {
        value[field] = v.clone();
    }
    format!("{value}\n")
}

fn user(uuid: &str, text: &str, extra: Value) -> String {
    record("user", uuid, json!(text), extra)
}

fn assistant(uuid: &str, block: Value) -> String {
    record("assistant", uuid, json!([block]), json!({}))
}

fn t3_row(created_by: &str, text: &str) -> String {
    let row = json!({
        "messageId": format!("t3-{created_by}-{}", text.len()), "threadId": "thread-a",
        "sessionId": "session-1", "createdBy": created_by, "creationSource": "web",
        "createdAt": "2026-01-02T03:04:00.000Z", "textSha256": text_hash(text),
    });
    format!("{row}\n")
}

#[test]
fn prompts_from_a_parent_agent_are_another_agents() {
    let mut s = Setup::new("other-agent");
    s.write(
        SUBAGENT,
        &[user(
            "opening",
            "Survey the north field",
            json!({"isSidechain": true}),
        )],
    );
    s.write(
        MAIN,
        &[user(
            "coordinated",
            "Sort the seed packets",
            json!({"origin": {"kind": "coordinator"}}),
        )],
    );
    s.read();
    assert_eq!(s.author("opening"), "other_agent");
    assert_eq!(s.author("coordinated"), "other_agent");
}

#[test]
fn a_task_notification_in_the_users_turn_is_a_tools() {
    let mut s = Setup::new("notification");
    s.write(
        MAIN,
        &[user(
            "note",
            "<task-notification>\n<status>completed</status>\nThe shed is painted.\n</task-notification>",
            json!({"origin": {"kind": "task-notification"}, "promptSource": "system"}),
        )],
    );
    s.read();
    assert_eq!(s.author("note"), "tool");
}

#[test]
fn a_command_output_wrapper_with_a_human_marker_is_a_tools() {
    let mut s = Setup::new("command-output");
    s.write(
        MAIN,
        &[user(
            "stdout",
            "<local-command-stdout>Compost turned</local-command-stdout>",
            json!({"promptSource": "typed", "origin": {"kind": "human"}}),
        )],
    );
    s.read();
    assert_eq!(s.author("stdout"), "tool");
}

#[test]
fn records_carrying_a_tool_result_are_not_stored() {
    let mut s = Setup::new("tool-result");
    let block = json!([{"type": "tool_result", "tool_use_id": "toolu_fake", "content": "ok"}]);
    s.write(
        MAIN,
        &[
            record("user", "block", block, json!({})),
            user("result", "ok", json!({"toolUseResult": {"stdout": "ok"}})),
            user(
                "source",
                "ok",
                json!({"sourceToolAssistantUUID": "fake-uuid"}),
            ),
        ],
    );
    let report = s.read();
    assert_eq!(s.count(), 0);
    assert_eq!(report.trees["claude"].skipped["tool_result"], 3);
}

#[test]
fn meta_records_and_compaction_summaries_are_not_stored() {
    let mut s = Setup::new("meta");
    s.write(
        MAIN,
        &[
            user(
                "meta",
                "Base directory for this skill",
                json!({"isMeta": true}),
            ),
            user(
                "summary",
                "This session is being continued",
                json!({"isCompactSummary": true, "isVisibleInTranscriptOnly": true}),
            ),
        ],
    );
    let report = s.read();
    assert_eq!(s.count(), 0);
    let skipped = &report.trees["claude"].skipped;
    assert_eq!((skipped["meta"], skipped["compact_summary"]), (1, 1));
}

#[test]
fn a_plain_prompt_takes_its_author_from_the_matching_t3_row() {
    let mut s = Setup::new("t3-match");
    let blocks = json!([{"type": "text", "text": "Plant the roses by the gate"}]);
    s.write(
        MAIN,
        &[
            record("user", "roses", blocks, json!({"promptSource": "sdk"})),
            user("beans", "Water the beans", json!({"promptSource": "sdk"})),
        ],
    );
    s.write(
        T3,
        &[
            t3_row("user", "Plant the roses by the gate"),
            t3_row("agent", "Water the beans"),
        ],
    );
    s.read();
    assert_eq!(s.author("roses"), "person");
    assert_eq!(s.author("beans"), "other_agent");
}

#[test]
fn a_t3_linked_prompt_follows_the_rows_present_at_each_read() {
    let mut s = Setup::new("t3-later");
    s.write(
        MAIN,
        &[user("gate", "Fix the gate", json!({"promptSource": "sdk"}))],
    );
    s.write(T3, &[t3_row("user", "Something else entirely")]);
    s.read();
    assert_eq!(s.author("gate"), "unknown");
    s.append(T3, &t3_row("user", "Fix the gate"));
    s.read();
    assert_eq!(s.author("gate"), "person");
    s.append(T3, &t3_row("agent", "Fix the gate"));
    s.read();
    assert_eq!(s.author("gate"), "unknown");
}

#[test]
fn two_same_prompts_with_a_person_row_and_an_agent_row_are_both_unknown() {
    let mut s = Setup::new("t3-mixed");
    s.write(
        MAIN,
        &[
            user("first", "Weed the beds", json!({"promptSource": "sdk"})),
            user("second", "Weed the beds", json!({"promptSource": "sdk"})),
        ],
    );
    s.write(
        T3,
        &[
            t3_row("user", "Weed the beds"),
            t3_row("agent", "Weed the beds"),
        ],
    );
    s.read();
    assert_eq!(s.author("first"), "unknown");
    assert_eq!(s.author("second"), "unknown");
}

#[test]
fn a_plain_prompt_in_an_sdk_session_without_t3_is_unknown() {
    let mut s = Setup::new("sdk");
    s.write(
        MAIN,
        &[user(
            "sdk",
            "Count the apples",
            json!({"promptSource": "sdk", "turnOrigin": "sdk"}),
        )],
    );
    s.read();
    assert_eq!(s.author("sdk"), "unknown");
}

#[test]
fn a_terminal_prompt_is_the_persons_only_with_a_human_marker() {
    let mut s = Setup::new("terminal");
    let cli = |uuid: &str, marker: Value| {
        let mut extra = json!({"entrypoint": "cli"});
        for (field, v) in marker.as_object().unwrap() {
            extra[field] = v.clone();
        }
        user(uuid, &format!("Prune the {uuid} tree"), extra)
    };
    s.write(
        MAIN,
        &[
            cli("origin", json!({"origin": {"kind": "human"}})),
            cli("typed", json!({"promptSource": "typed"})),
            cli("turn", json!({"turnOrigin": "human"})),
            cli("bare", json!({})),
        ],
    );
    s.read();
    for uuid in ["origin", "typed", "turn"] {
        assert_eq!(s.author(uuid), "person", "{uuid}");
    }
    assert_eq!(s.author("bare"), "unknown");
}

#[test]
fn an_assistant_text_block_is_the_agents_and_thinking_and_tool_calls_are_not_stored() {
    let mut s = Setup::new("assistant");
    s.write(
        MAIN,
        &[
            assistant(
                "thinking",
                json!({"type": "thinking", "thinking": "hmm", "signature": "fake"}),
            ),
            assistant(
                "call",
                json!({"type": "tool_use", "id": "toolu_fake", "name": "Bash", "input": {}}),
            ),
            assistant(
                "reply",
                json!({"type": "text", "text": "The roses are planted."}),
            ),
        ],
    );
    let report = s.read();
    assert_eq!(s.count(), 1);
    assert_eq!(s.author("reply"), "agent");
    assert_eq!(report.trees["claude"].skipped["no_text"], 2);
}

#[test]
fn an_api_request_blob_copy_of_a_message_adds_no_item() {
    let mut s = Setup::new("blob");
    let blob = json!({
        "type": "api-request-blob", "sessionId": "session-1", "hash": "fakehash",
        "message": {"role": "user", "content": "Fix the gate"},
    });
    s.write(
        MAIN,
        &[user("gate", "Fix the gate", json!({})), format!("{blob}\n")],
    );
    let report = s.read();
    assert_eq!(s.count(), 1);
    assert_eq!(report.trees["claude"].skipped["api-request-blob"], 1);
}

#[test]
fn an_item_carries_its_time_working_directory_file_line_and_key() {
    let mut s = Setup::new("place");
    s.write(
        MAIN,
        &[
            assistant("first", json!({"type": "text", "text": "Hello"})),
            assistant("second", json!({"type": "text", "text": "Again"})),
        ],
    );
    s.read();
    let row: (String, String, String, String, i64, String) =
        s.db.query_row(
            "SELECT tool, session_id, time, cwd, line, file FROM items WHERE key = 'claude:second'",
            [],
            |r| {
                Ok((
                    r.get(0)?,
                    r.get(1)?,
                    r.get(2)?,
                    r.get(3)?,
                    r.get(4)?,
                    r.get(5)?,
                ))
            },
        )
        .unwrap();
    assert_eq!(
        row,
        (
            "claude".into(),
            "session-1".into(),
            "2026-01-02T03:04:05.000Z".into(),
            "/home/someone/code/garden".into(),
            2,
            MAIN.into()
        )
    );
}

#[test]
fn a_slash_command_prompt_is_stored_as_typed_and_matches_either_form_t3_sent() {
    let mut s = Setup::new("command");
    let wrapped = |name: &str, args: &str| {
        format!(
            "<command-message>{name}</command-message>\n<command-name>/{name}</command-name>\n<command-args>{args}</command-args>"
        )
    };
    s.write(
        MAIN,
        &[
            user("u1", &wrapped("tidy", "the shed first"), json!({})),
            user(
                "u2",
                &wrapped("tools:plant", "three rows of beans"),
                json!({}),
            ),
            user("u3", &wrapped("water", "the far bed"), json!({})),
        ],
    );
    s.write(
        T3,
        &[
            t3_row("user", "/tidy the shed first"),
            t3_row("user", "$tools:plant three rows of beans"),
        ],
    );
    s.read();
    assert_eq!(s.author("u1"), "person");
    assert_eq!(s.author("u2"), "person");
    assert_eq!(s.author("u3"), "unknown");
    let text: String =
        s.db.query_row("SELECT text FROM items WHERE key = 'claude:u2'", [], |r| {
            r.get(0)
        })
        .unwrap();
    assert_eq!(text, "/tools:plant three rows of beans");
}

#[test]
fn an_interruption_marker_is_not_stored() {
    let mut s = Setup::new("interrupted");
    s.write(
        MAIN,
        &[user("u1", "[Request interrupted by user]", json!({}))],
    );
    let report = s.read();
    assert_eq!(s.count(), 0);
    assert_eq!(report.trees["claude"].skipped["interrupted"], 1);
}
