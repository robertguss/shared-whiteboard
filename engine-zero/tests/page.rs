use engine_zero::config::Config;
use engine_zero::events::append;
use engine_zero::ingest::{READERS, read};
use engine_zero::page::{Reply, handle, stats};
use engine_zero::propose::check;
use engine_zero::store;
use rusqlite::Connection;
use serde_json::{Value, json};
use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};

const SECRET: &str = "0123456789abcdef0123456789abcdef";
const SAME: &str = "Same mechanism in unrelated domains";
const SOLVES: &str = "One project could solve another's problem";
const ROSE: &str = "Roses & <beans> share the bed";
const PUMP: &str = "The pond pump hums like a beehive";
const APPLE: &str = "The apple trees drop fruit in a storm";

struct Setup {
    root: PathBuf,
    db: Connection,
    config: Value,
}

impl Setup {
    fn new(name: &str) -> Setup {
        let root =
            std::env::temp_dir().join(format!("engine-zero-page-{}-{name}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        let record = root.join("record");
        for (session, folder, text) in [
            ("rose", "garden", ROSE),
            ("pump", "<b>pond", PUMP),
            ("apple", "orchard", APPLE),
        ] {
            let path = record.join(format!("sources/laptop/claude/{session}/{session}.jsonl"));
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            let line = json!({"type": "user", "uuid": session, "sessionId": session, "promptSource": "typed",
                "timestamp": "2026-11-02T03:04:05.000Z", "cwd": format!("/home/someone/code/{folder}"),
                "message": {"role": "user", "content": text}});
            fs::write(path, format!("{line}\n")).unwrap();
        }
        let parents = ["/home/someone/code".to_string()];
        let mut db = store::open(&root.join("engine.sqlite")).unwrap();
        read(&mut db, &record, READERS, &parents, &[]).unwrap();
        let proposals = [
            json!({"kind": SAME, "statement": "Both <em>beds</em> fight for light", "origin": "evening pass",
                "quotes": [{"key": "claude:rose", "text": ROSE}, {"key": "claude:pump", "text": PUMP}]}),
            json!({"kind": SOLVES, "statement": "The pump could water the orchard",
                "quotes": [{"key": "claude:pump", "text": PUMP}, {"key": "claude:apple", "text": APPLE}]}),
            json!({"kind": SAME, "statement": "Roses and apples share a storm",
                "quotes": [{"key": "claude:rose", "text": ROSE}, {"key": "claude:apple", "text": APPLE}]}),
        ];
        let lines: String = proposals.iter().map(|p| format!("{p}\n")).collect();
        let outcomes = check(&mut db, &record, "laptop", "gardener", &lines).unwrap();
        assert_eq!(outcomes, ["1 kept", "2 kept", "3 kept"]);
        let config = json!({
            "machine": "laptop", "remote": root.join("bucket"), "sources": {},
            "t3_database": root.join("t3.sqlite"), "record_folder": record,
            "database": root.join("engine.sqlite"),
        });
        Setup { root, db, config }
    }

    fn get(&mut self, url: &str) -> Reply {
        let config: Config = serde_json::from_value(self.config.clone()).unwrap();
        handle(&mut self.db, &config, SECRET, "GET", url, None, "").unwrap()
    }

    fn post(&mut self, user: Option<&str>, body: &str) -> Reply {
        let config: Config = serde_json::from_value(self.config.clone()).unwrap();
        handle(&mut self.db, &config, SECRET, "POST", "/", user, body).unwrap()
    }

    fn place(&self, index: usize) -> (String, i64) {
        let mut statement = self
            .db
            .prepare("SELECT file, line FROM events WHERE kind = 'proposal' ORDER BY line")
            .unwrap();
        let rows = statement
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
            .unwrap();
        rows.map(Result::unwrap).nth(index).unwrap()
    }

    fn form(&self, index: usize, key: &str, mark: &str) -> String {
        let (file, line) = self.place(index);
        format!(
            "key={key}&file={}&line={line}&mark={mark}",
            file.replace('/', "%2F")
        )
    }

    fn events(&self, kind: &str) -> Vec<Value> {
        let mut statement = self
            .db
            .prepare("SELECT body FROM events WHERE kind = ?1 ORDER BY file, line")
            .unwrap();
        let rows = statement
            .query_map([kind], |row| row.get::<_, String>(0))
            .unwrap();
        rows.map(|body| serde_json::from_str(&body.unwrap()).unwrap())
            .collect()
    }

    fn judge(&mut self, index: usize, mark: &str, time: &str) {
        let connection = self.events("proposal")[index]["connection"].clone();
        let judgement = json!({"kind": "judgement", "time": time, "writer": {"by": "page", "user": "gardener"},
            "mark": mark, "connection": connection});
        append(
            &mut self.db,
            &self.root.join("record"),
            "laptop",
            &judgement,
        )
        .unwrap();
    }
}

impl Drop for Setup {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn shows(reply: &Reply, setup: &Setup, index: usize) -> bool {
    let (file, line) = setup.place(index);
    reply.body.contains(&format!("value=\"{file}\""))
        && reply.body.contains(&format!("value=\"{line}\""))
}

#[test]
fn the_page_shows_each_waiting_connection_as_escaped_text_without_its_origin_and_only_with_the_key()
{
    let mut s = Setup::new("show");
    s.judge(2, "not_real", "2026-11-03T08:00:00Z");
    s.db.execute("DELETE FROM items WHERE key = 'claude:apple'", [])
        .unwrap();

    let page = s.get(&format!("/?key={SECRET}"));
    assert_eq!(page.status, 200);
    let body = &page.body;
    for text in [
        SAME,
        "Both &lt;em&gt;beds&lt;/em&gt; fight for light",
        "Roses &amp; &lt;beans&gt; share the bed",
        "&lt;b&gt;pond",
        "garden",
        "2026-11-02",
        "The pump could water the orchard",
        &format!("<q>{APPLE}</q> <small>-, -</small>"),
        "<button name=\"mark\" value=\"real_new\">Real and new</button>",
        "<button name=\"mark\" value=\"real_known\">Real and already known</button>",
        "<button name=\"mark\" value=\"not_real\">Not real</button>",
        &format!("name=\"key\" value=\"{SECRET}\""),
    ] {
        assert!(body.contains(text), "{text} is missing from {body}");
    }
    for text in [
        "<em>",
        "<beans>",
        "<b>pond",
        "evening pass",
        "Roses and apples",
    ] {
        assert!(!body.contains(text), "{text} is shown in {body}");
    }
    assert!(shows(&page, &s, 0) && shows(&page, &s, 1));
    assert_eq!(body.matches("<form").count(), 2);

    for url in ["/", "/?key=wrong", &format!("/?other={SECRET}")] {
        let refused = s.get(url);
        assert_eq!((refused.status, refused.body.as_str()), (403, ""), "{url}");
    }
}

#[test]
fn a_mark_writes_a_judgement_holding_the_whole_connection_reaches_the_bucket_and_counts_in_stats() {
    let mut s = Setup::new("mark");
    let marked = s.post(Some("gardener-7"), &s.form(0, SECRET, "real_new"));
    assert_eq!(marked.status, 303);
    assert_eq!(
        marked.location.as_deref(),
        Some(&*format!("/?key={SECRET}"))
    );

    let proposals = s.events("proposal");
    let judgements = s.events("judgement");
    assert_eq!(judgements.len(), 1);
    let judgement = &judgements[0];
    assert_eq!(
        judgement["writer"],
        json!({"by": "page", "user": "gardener-7"})
    );
    assert_eq!(judgement["mark"], "real_new");
    assert_eq!(judgement["connection"], proposals[0]["connection"]);
    assert_eq!(judgement["origin"], "evening pass");
    assert!(judgement["time"].as_str().unwrap().ends_with('Z'));

    let page = s.get(&format!("/?key={SECRET}"));
    assert!(!shows(&page, &s, 0) && shows(&page, &s, 1));

    let month = fs::read_dir(s.root.join("bucket/events/laptop"))
        .unwrap()
        .next()
        .unwrap()
        .unwrap();
    let uploaded = fs::read_to_string(month.path()).unwrap();
    let last: Value = serde_json::from_str(uploaded.lines().last().unwrap()).unwrap();
    assert_eq!(&last, judgement);

    let blocked = s.root.join("blocked");
    fs::write(&blocked, "not a folder").unwrap();
    s.config["remote"] = json!(blocked);
    let saved = s.post(Some("gardener-7"), &s.form(1, SECRET, "not_real"));
    assert_eq!(saved.status, 200);
    assert!(
        saved
            .body
            .contains("saved on this machine but did not reach the bucket"),
        "{}",
        saved.body
    );
    let judgements = s.events("judgement");
    assert_eq!(judgements.len(), 2);
    assert_eq!(judgements[1]["connection"], proposals[1]["connection"]);
    assert!(judgements[1].get("origin").is_none());

    s.judge(1, "real_known", "2999-01-01T00:00:00Z");
    assert_eq!(
        stats(&s.db).unwrap(),
        "mark real_known: 1\nmark real_new: 1\norigin evening pass: 1\norigin none: 1\nwaiting: 1\n"
    );
}

#[test]
fn a_mark_is_refused_and_writes_nothing_without_the_key_or_user_or_with_a_wrong_mark_or_place() {
    let mut s = Setup::new("refuse");
    let good = s.form(0, SECRET, "real_new");
    let cases = [
        (Some("gardener"), good.replace(SECRET, ""), 403),
        (Some("gardener"), good.replace(SECRET, "wrong"), 403),
        (
            Some("gardener"),
            good.replace(&format!("key={SECRET}&"), ""),
            403,
        ),
        (None, good.clone(), 403),
        (Some(""), good.clone(), 403),
        (Some("gardener"), s.form(0, SECRET, "maybe"), 400),
        (Some("gardener"), good.replace("&mark=real_new", ""), 400),
        (
            Some("gardener"),
            s.form(0, SECRET, "real_new").replace("line=", "line=9"),
            404,
        ),
    ];
    for (user, body, status) in cases {
        let refused = s.post(user, &body);
        assert_eq!(refused.status, status, "{body}");
        assert!(refused.location.is_none());
    }
    let config: Config = serde_json::from_value(s.config.clone()).unwrap();
    let in_query = format!("/?key={SECRET}");
    let body = good.replace(&format!("key={SECRET}&"), "");
    let refused = handle(
        &mut s.db,
        &config,
        SECRET,
        "POST",
        &in_query,
        Some("gardener"),
        &body,
    )
    .unwrap();
    assert_eq!(refused.status, 403);
    assert!(s.events("judgement").is_empty());
    assert!(!s.root.join("bucket").exists());

    assert_eq!(s.post(Some("gardener"), &good).status, 303);
    assert_eq!(s.post(Some("gardener"), &good).status, 404);
    assert_eq!(s.events("judgement").len(), 1);
}

struct Server(Child);

impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

fn exchange(port: u16, request: &str) -> String {
    let mut stream = TcpStream::connect(("127.0.0.1", port)).unwrap();
    stream.write_all(request.as_bytes()).unwrap();
    let mut reply = String::new();
    stream.read_to_string(&mut reply).unwrap();
    reply
}

#[test]
fn page_follows_a_rebuild_and_takes_a_mark_over_a_real_socket() {
    let mut s = Setup::new("socket");
    let file = s.root.join("config.json");
    fs::write(&file, s.config.to_string()).unwrap();
    let port = TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port();
    let child = Command::new(env!("CARGO_BIN_EXE_engine-zero"))
        .args(["--config", file.to_str().unwrap(), "page", "--port"])
        .arg(port.to_string())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut server = Server(child);
    let mut first = String::new();
    BufReader::new(server.0.stdout.as_mut().unwrap())
        .read_line(&mut first)
        .unwrap();
    let secret = first.trim().strip_prefix("/?key=").unwrap().to_string();
    assert_eq!(secret.len(), 32);
    assert!(secret.chars().all(|c| c.is_ascii_hexdigit()));

    let get = format!("GET /?key={secret} HTTP/1.1\r\nHost: garden\r\nConnection: close\r\n\r\n");
    fs::remove_file(s.root.join("engine.sqlite")).unwrap();
    assert!(exchange(port, &get).contains("Nothing is waiting."));
    let rebuilt = Command::new(env!("CARGO_BIN_EXE_engine-zero"))
        .args(["--config", file.to_str().unwrap(), "rebuild"])
        .output()
        .unwrap();
    assert!(rebuilt.status.success());
    s.db = store::open(&s.root.join("engine.sqlite")).unwrap();

    let page = exchange(port, &get);
    assert!(page.starts_with("HTTP/1.1 200"), "{page}");
    assert!(page.contains("Both &lt;em&gt;beds&lt;/em&gt;"));

    let form = s.form(0, &secret, "real_known");
    let marked = exchange(
        port,
        &format!(
            "POST / HTTP/1.1\r\nHost: garden\r\nConnection: close\r\nX-ExeDev-UserID: gardener-7\r\n\
             Content-Type: application/x-www-form-urlencoded\r\nContent-Length: {}\r\n\r\n{form}",
            form.len()
        ),
    );
    assert!(marked.starts_with("HTTP/1.1 303"), "{marked}");
    assert!(marked.contains(&format!("/?key={secret}")));
    assert_eq!(s.events("judgement")[0]["writer"]["user"], "gardener-7");
}
