use engine_zero::t3::extract;
use rusqlite::Connection;
use serde_json::{Value, json};
use std::fs;
use std::path::PathBuf;

fn made_up_database(name: &str) -> (PathBuf, Connection) {
    let root = std::env::temp_dir().join(format!("engine-zero-{}-{name}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();
    let db = Connection::open(root.join("t3.sqlite")).unwrap();
    db.execute_batch(include_str!("fixtures/t3/state.sql"))
        .unwrap();
    (root, db)
}

fn lines(path: PathBuf) -> Vec<Value> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect()
}

#[test]
fn the_extract_has_one_row_per_user_message_and_no_message_text() {
    let (root, _db) = made_up_database("extract");
    extract(&root.join("t3.sqlite"), &root.join("out")).unwrap();

    let messages = lines(root.join("out/messages.jsonl"));
    assert_eq!(
        messages,
        vec![
            json!({"messageId": "message-1", "threadId": "thread-a",
                "sessionId": "00000000-fake-session-0001", "createdBy": "user",
                "creationSource": "web", "createdAt": "2026-01-01T10:00:00.000Z",
                "textSha256": "04dadb9197d260ee121518b9a6fcd788d2a0c841e2d57bbe2a5789312a3def5c"}),
            json!({"messageId": "message-3", "threadId": "thread-b",
                "sessionId": "00000000-fake-session-0002", "createdBy": "agent",
                "creationSource": "mcp", "createdAt": "2026-01-01T11:00:00.000Z",
                "textSha256": "53a099e26b1905f26d7ff49407f8dbf89a38c49ba60fb14dbdb2914ef2b5a883"}),
            json!({"messageId": "message-4", "threadId": "thread-c",
                "sessionId": null, "createdBy": "user",
                "creationSource": "mobile", "createdAt": "2026-01-01T12:00:00.000Z",
                "textSha256": "0649574fe7bebea5c16990270e8c57db9039a0f099d55d00994deb9b7f83bdf6"}),
        ]
    );
    assert!(
        !fs::read_to_string(root.join("out/messages.jsonl"))
            .unwrap()
            .contains("Made-up")
    );

    assert_eq!(
        lines(root.join("out/projects.jsonl")),
        vec![
            json!({"projectId": "project-1", "workspaceRoot": "/home/example/code/pretend-garden"}),
            json!({"projectId": "project-2", "workspaceRoot": "/home/example/.t3/scratch"}),
        ]
    );
}

#[test]
fn a_second_extract_appends_nothing_and_a_new_message_appends_one_row() {
    let (root, db) = made_up_database("extract-again");
    let out = root.join("out");
    extract(&root.join("t3.sqlite"), &out).unwrap();
    let first = fs::read_to_string(out.join("messages.jsonl")).unwrap();
    extract(&root.join("t3.sqlite"), &out).unwrap();
    assert_eq!(
        fs::read_to_string(out.join("messages.jsonl")).unwrap(),
        first
    );

    db.execute_batch(
        "INSERT INTO orchestration_v2_projection_messages VALUES
         ('message-5', 'thread-a', 'run-1', 'user', '2026-01-02T09:00:00.000Z',
          '{\"createdBy\":\"user\",\"creationSource\":\"web\",\"text\":\"Another made-up question\"}');",
    )
    .unwrap();
    extract(&root.join("t3.sqlite"), &out).unwrap();
    let after = fs::read_to_string(out.join("messages.jsonl")).unwrap();
    assert!(after.starts_with(&first));
    let added = lines(out.join("messages.jsonl")).split_off(3);
    assert_eq!(added.len(), 1);
    assert_eq!(added[0]["messageId"], "message-5");
}
