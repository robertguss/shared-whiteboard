use engine_zero::config::Config;
use engine_zero::copy::{copy, pull};
use serde_json::{Value, json};
use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

struct Setup {
    root: PathBuf,
    config: Value,
}

impl Setup {
    fn new(name: &str) -> Setup {
        let root = std::env::temp_dir().join(format!("engine-zero-{}-{name}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("claude-home")).unwrap();
        fs::create_dir_all(root.join("codex-home")).unwrap();
        let config = json!({
            "machine": "laptop",
            "remote": root.join("bucket"),
            "sources": {"claude": root.join("claude-home"), "codex": root.join("codex-home")},
            "t3_database": root.join("t3.sqlite"),
            "record_folder": root.join("record"),
            "database": root.join("engine.sqlite"),
        });
        Setup { root, config }
    }

    fn path(&self, relative: &str) -> PathBuf {
        self.root.join(relative)
    }

    fn config(&self) -> Config {
        serde_json::from_value(self.config.clone()).unwrap()
    }

    fn run(&self, command: &str) -> Output {
        let file = self.path("config.json");
        fs::write(&file, self.config.to_string()).unwrap();
        Command::new(env!("CARGO_BIN_EXE_engine-zero"))
            .args(["--config", file.to_str().unwrap(), command])
            .output()
            .unwrap()
    }
}

impl Drop for Setup {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn write(path: &Path, content: &str) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, content).unwrap();
}

fn read(path: &Path) -> String {
    fs::read_to_string(path).unwrap()
}

fn utc_now() -> String {
    let out = Command::new("date")
        .args(["-u", "+%Y-%m-%dT%H%M%SZ"])
        .output()
        .unwrap();
    String::from_utf8(out.stdout).unwrap().trim().to_string()
}

#[test]
fn a_file_deleted_at_the_source_stays_in_the_bucket() {
    let s = Setup::new("deleted");
    write(&s.path("claude-home/project/one.jsonl"), "first\n");
    assert!(s.run("copy").status.success());
    fs::remove_file(s.path("claude-home/project/one.jsonl")).unwrap();
    assert!(s.run("copy").status.success());
    assert_eq!(
        read(&s.path("bucket/sources/laptop/claude/project/one.jsonl")),
        "first\n"
    );
}

#[test]
fn a_grown_file_replaces_its_copy_and_the_shorter_one_is_kept_under_the_run_time() {
    let s = Setup::new("grown");
    write(&s.path("claude-home/a.jsonl"), "one\n");
    assert!(s.run("copy").status.success());
    write(&s.path("claude-home/a.jsonl"), "one\ntwo\n");
    let before = utc_now();
    assert!(s.run("copy").status.success());
    let after = utc_now();
    let stamps: Vec<String> = fs::read_dir(s.path("bucket/old"))
        .unwrap()
        .map(|e| e.unwrap().file_name().into_string().unwrap())
        .collect();
    assert_eq!(stamps.len(), 1);
    assert!(before <= stamps[0] && stamps[0] <= after, "{stamps:?}");
    assert_eq!(
        read(&s.path("bucket/sources/laptop/claude/a.jsonl")),
        "one\ntwo\n"
    );
    assert_eq!(
        read(&s.path(&format!(
            "bucket/old/{}/sources/laptop/claude/a.jsonl",
            stamps[0]
        ))),
        "one\n"
    );
}

#[test]
fn a_file_changed_twice_keeps_both_earlier_versions() {
    let s = Setup::new("twice");
    write(&s.path("claude-home/p/a.jsonl"), "v1\n");
    copy(&s.config(), "t1").unwrap();
    write(&s.path("claude-home/p/a.jsonl"), "v2 longer\n");
    copy(&s.config(), "t2").unwrap();
    write(&s.path("claude-home/p/a.jsonl"), "v3 longer still\n");
    copy(&s.config(), "t3").unwrap();
    assert_eq!(
        read(&s.path("bucket/old/t2/sources/laptop/claude/p/a.jsonl")),
        "v1\n"
    );
    assert_eq!(
        read(&s.path("bucket/old/t3/sources/laptop/claude/p/a.jsonl")),
        "v2 longer\n"
    );
    assert_eq!(
        read(&s.path("bucket/sources/laptop/claude/p/a.jsonl")),
        "v3 longer still\n"
    );
}

#[test]
fn a_second_run_with_no_changes_transfers_nothing() {
    let s = Setup::new("unchanged");
    rusqlite::Connection::open(s.path("t3.sqlite"))
        .unwrap()
        .execute_batch(include_str!("fixtures/t3/state.sql"))
        .unwrap();
    write(&s.path("claude-home/a.jsonl"), "one\n");
    write(&s.path("record/events/laptop/2026-01.jsonl"), "{}\n");
    copy(&s.config(), "t1").unwrap();
    let copied = [
        "bucket/sources/laptop/claude/a.jsonl",
        "bucket/sources/laptop/t3/messages.jsonl",
        "bucket/sources/laptop/t3/projects.jsonl",
        "bucket/events/laptop/2026-01.jsonl",
    ];
    let inodes: Vec<u64> = copied
        .iter()
        .map(|p| fs::metadata(s.path(p)).unwrap().ino())
        .collect();
    copy(&s.config(), "t2").unwrap();
    let again: Vec<u64> = copied
        .iter()
        .map(|p| fs::metadata(s.path(p)).unwrap().ino())
        .collect();
    assert_eq!(inodes, again);
    assert!(!s.path("bucket/old").exists());
}

#[test]
fn pull_into_an_empty_record_folder_mirrors_sources_and_events_but_not_old() {
    let s = Setup::new("pull");
    let files = [
        "sources/laptop/claude/p/a.jsonl",
        "sources/desktop/codex/b.jsonl",
        "events/laptop/2026-01.jsonl",
        "events/desktop/2026-01.jsonl",
    ];
    for f in files {
        write(&s.path(&format!("bucket/{f}")), f);
    }
    write(
        &s.path("bucket/old/t1/sources/laptop/claude/p/a.jsonl"),
        "old",
    );
    assert!(s.run("pull").status.success());
    for f in files {
        assert_eq!(read(&s.path(&format!("record/{f}"))), f);
    }
    assert!(!s.path("record/old").exists());
}

#[test]
fn pull_never_replaces_this_machines_event_file_and_restores_a_missing_one() {
    let s = Setup::new("pull-own");
    write(&s.path("bucket/sources/laptop/claude/a.jsonl"), "a\n");
    write(&s.path("bucket/events/laptop/2026-01.jsonl"), "one\n");
    write(&s.path("bucket/events/laptop/2026-02.jsonl"), "feb\n");
    write(&s.path("record/events/laptop/2026-01.jsonl"), "one\ntwo\n");
    pull(&s.config()).unwrap();
    assert_eq!(
        read(&s.path("record/events/laptop/2026-01.jsonl")),
        "one\ntwo\n"
    );
    assert_eq!(read(&s.path("record/events/laptop/2026-02.jsonl")), "feb\n");
}

#[test]
fn pull_never_replaces_this_machines_t3_extract_and_restores_a_missing_one() {
    let s = Setup::new("pull-own-t3");
    write(&s.path("bucket/sources/laptop/t3/messages.jsonl"), "old\n");
    write(&s.path("bucket/sources/laptop/t3/projects.jsonl"), "p\n");
    write(
        &s.path("record/sources/laptop/t3/messages.jsonl"),
        "old\nnew\n",
    );
    pull(&s.config()).unwrap();
    assert_eq!(
        read(&s.path("record/sources/laptop/t3/messages.jsonl")),
        "old\nnew\n"
    );
    assert_eq!(
        read(&s.path("record/sources/laptop/t3/projects.jsonl")),
        "p\n"
    );
}

#[test]
fn a_missing_source_folder_fails_the_copy_after_everything_else_is_copied() {
    let s = Setup::new("missing-source");
    fs::remove_dir_all(s.path("claude-home")).unwrap();
    write(&s.path("codex-home/b.jsonl"), "b\n");
    write(&s.path("record/events/laptop/2026-01.jsonl"), "{}\n");
    assert!(copy(&s.config(), "t1").is_err());
    assert_eq!(read(&s.path("bucket/sources/laptop/codex/b.jsonl")), "b\n");
    assert_eq!(read(&s.path("bucket/events/laptop/2026-01.jsonl")), "{}\n");
}

#[test]
fn pull_succeeds_when_the_bucket_holds_no_events_yet() {
    let s = Setup::new("pull-no-events");
    write(&s.path("bucket/sources/laptop/claude/a.jsonl"), "a\n");
    assert!(s.run("pull").status.success());
    assert_eq!(read(&s.path("record/sources/laptop/claude/a.jsonl")), "a\n");
}

#[test]
fn without_a_t3_database_copy_succeeds() {
    let s = Setup::new("no-t3");
    write(&s.path("claude-home/a.jsonl"), "one\n");
    copy(&s.config(), "t1").unwrap();
    assert_eq!(
        read(&s.path("bucket/sources/laptop/claude/a.jsonl")),
        "one\n"
    );
    assert!(!s.path("bucket/sources/laptop/t3").exists());
}

#[test]
fn with_unexpected_t3_tables_the_files_are_copied_and_the_command_fails() {
    let s = Setup::new("bad-t3");
    rusqlite::Connection::open(s.path("t3.sqlite"))
        .unwrap()
        .execute_batch("CREATE TABLE something_else (x TEXT);")
        .unwrap();
    write(&s.path("claude-home/a.jsonl"), "one\n");
    write(&s.path("record/events/laptop/2026-01.jsonl"), "{}\n");
    let out = s.run("copy");
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("orchestration_v2_projection_messages"));
    assert_eq!(
        read(&s.path("bucket/sources/laptop/claude/a.jsonl")),
        "one\n"
    );
    assert_eq!(read(&s.path("bucket/events/laptop/2026-01.jsonl")), "{}\n");
}

#[test]
fn a_config_without_a_machine_name_is_refused() {
    let mut s = Setup::new("no-machine");
    write(&s.path("claude-home/a.jsonl"), "one\n");
    for machine in [None, Some("")] {
        match machine {
            None => s.config.as_object_mut().unwrap().remove("machine"),
            Some(m) => s
                .config
                .as_object_mut()
                .unwrap()
                .insert("machine".into(), m.into()),
        };
        let out = s.run("copy");
        assert!(!out.status.success());
        assert!(String::from_utf8_lossy(&out.stderr).contains("machine"));
        assert!(!s.path("bucket").exists());
    }
}

#[test]
fn a_config_with_private_files_inside_the_repository_is_refused() {
    let inside = Path::new(env!("CARGO_MANIFEST_DIR")).join("../private-test");
    for key in ["record_folder", "database"] {
        let mut s = Setup::new(&format!("in-repo-{key}"));
        write(&s.path("claude-home/a.jsonl"), "one\n");
        s.config[key] = json!(inside.join(key));
        let out = s.run("copy");
        assert!(!out.status.success());
        assert!(String::from_utf8_lossy(&out.stderr).contains("repository"));
        assert!(!s.path("bucket").exists());
        assert!(!inside.exists());
    }
}
