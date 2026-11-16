use crate::config::Config;
use crate::{Result, copy, events, store};
use rusqlite::{Connection, OptionalExtension, params};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::io::Read;
use tiny_http::{Header, Response, Server};

const MARKS: [(&str, &str); 3] = [
    ("real_new", "Real and new"),
    ("real_known", "Real and already known"),
    ("not_real", "Not real"),
];

const WAITING: &str = "
SELECT file, line, body FROM (
  SELECT file, line, time, body,
    row_number() OVER (PARTITION BY connection ORDER BY time, file, line) AS rank
  FROM events WHERE kind = 'proposal'
    AND connection NOT IN (SELECT connection FROM holding_judgements))
WHERE rank = 1";

const PROPOSAL: &str = "
SELECT body FROM events WHERE kind = 'proposal' AND file = ?1 AND line = ?2
  AND connection NOT IN (SELECT connection FROM holding_judgements)";

const ITEM: &str = "SELECT context, substr(time, 1, 10) FROM items WHERE key = ?1";

const HEAD: &str = "<!doctype html><html><head><meta charset=\"utf-8\">\
<meta name=\"viewport\" content=\"width=device-width\"><title>Waiting connections</title>\
<style>body{font-family:sans-serif;max-width:44em;margin:1em auto;padding:0 1em}\
section{border-top:1px solid #999;padding:.5em 0}</style></head><body>";

pub struct Reply {
    pub status: u16,
    pub location: Option<String>,
    pub body: String,
}

fn reply(status: u16, body: String) -> Reply {
    Reply {
        status,
        location: None,
        body,
    }
}

pub fn run(config: &Config, port: u16) -> Result<()> {
    let mut db = store::open(&config.database)?;
    let secret: String = db.query_row("SELECT lower(hex(randomblob(16)))", [], |row| row.get(0))?;
    let server = Server::http(("0.0.0.0", port))
        .map_err(|e| format!("cannot listen on port {port}: {e}"))?;
    println!("/?key={secret}");
    for mut request in server.incoming_requests() {
        let user = request
            .headers()
            .iter()
            .find(|header| header.field.equiv("X-ExeDev-UserID"))
            .map(|header| header.value.to_string());
        let mut body = String::new();
        let _ = request.as_reader().take(1 << 16).read_to_string(&mut body);
        let method = request.method().as_str().to_string();
        let url = request.url().to_string();
        let answer = handle(
            &mut db,
            config,
            &secret,
            &method,
            &url,
            user.as_deref(),
            &body,
        )
        .unwrap_or_else(|e| reply(500, escape(&e.to_string())));
        let mut response = Response::from_string(answer.body)
            .with_status_code(answer.status)
            .with_header(header("Content-Type", "text/html; charset=utf-8"));
        if let Some(location) = answer.location {
            response.add_header(header("Location", &location));
        }
        let _ = request.respond(response);
    }
    Ok(())
}

fn header(field: &str, value: &str) -> Header {
    Header::from_bytes(field.as_bytes(), value.as_bytes()).unwrap()
}

pub fn handle(
    db: &mut Connection,
    config: &Config,
    secret: &str,
    method: &str,
    url: &str,
    user: Option<&str>,
    body: &str,
) -> Result<Reply> {
    let (path, query) = url.split_once('?').unwrap_or((url, ""));
    let fields = decode(if method == "POST" { body } else { query });
    if fields.get("key").map(String::as_str) != Some(secret) {
        return Ok(reply(403, String::new()));
    }
    match (method, path) {
        ("GET", "/") => Ok(reply(200, page(db, secret)?)),
        ("POST", "/") => mark(db, config, secret, user, &fields),
        _ => Ok(reply(404, String::new())),
    }
}

fn page(db: &Connection, secret: &str) -> Result<String> {
    let mut statement = db.prepare(&format!("{WAITING} ORDER BY time, file, line"))?;
    let rows = statement.query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))?;
    let mut html = String::from(HEAD);
    let mut empty = true;
    for row in rows {
        let (file, line, body): (String, i64, String) = row?;
        let connection = &serde_json::from_str::<Value>(&body)?["connection"];
        html += &format!(
            "<section><h2>{}</h2><p>{}</p><ul>",
            escape(text(&connection["kind"])),
            escape(text(&connection["statement"]))
        );
        for quote in connection["quotes"].as_array().into_iter().flatten() {
            let item: Option<(Option<String>, Option<String>)> = db
                .query_row(ITEM, [text(&quote["key"])], |row| {
                    Ok((row.get(0)?, row.get(1)?))
                })
                .optional()?;
            let (context, date) = item.unwrap_or_default();
            html += &format!(
                "<li><q>{}</q> <small>{}, {}</small></li>",
                escape(text(&quote["text"])),
                escape(context.as_deref().unwrap_or("-")),
                escape(date.as_deref().unwrap_or("-"))
            );
        }
        html += &format!(
            "</ul><form method=\"post\" action=\"/\"><input type=\"hidden\" name=\"key\" value=\"{}\">\
             <input type=\"hidden\" name=\"file\" value=\"{}\"><input type=\"hidden\" name=\"line\" value=\"{line}\">",
            escape(secret),
            escape(&file)
        );
        for (value, label) in MARKS {
            html += &format!("<button name=\"mark\" value=\"{value}\">{label}</button> ");
        }
        html += "</form></section>";
        empty = false;
    }
    if empty {
        html += "<p>Nothing is waiting.</p>";
    }
    Ok(html + "</body></html>")
}

fn mark(
    db: &mut Connection,
    config: &Config,
    secret: &str,
    user: Option<&str>,
    fields: &HashMap<String, String>,
) -> Result<Reply> {
    let Some(user) = user.filter(|user| !user.is_empty()) else {
        return Ok(reply(403, String::new()));
    };
    let field = |name: &str| fields.get(name).map(String::as_str).unwrap_or_default();
    let mark = field("mark");
    if !MARKS.iter().any(|(value, _)| *value == mark) {
        return Ok(reply(400, format!("{} is not a mark", escape(mark))));
    }
    let proposal: Option<String> = match field("line").parse::<i64>() {
        Ok(line) => db
            .query_row(PROPOSAL, params![field("file"), line], |row| row.get(0))
            .optional()?,
        Err(_) => None,
    };
    let Some(proposal) = proposal else {
        return Ok(reply(404, "That connection is not waiting.".into()));
    };
    let proposal: Value = serde_json::from_str(&proposal)?;
    let mut event = json!({
        "kind": "judgement", "time": copy::time_now()?,
        "writer": {"by": "page", "user": user},
        "mark": mark, "connection": proposal["connection"],
    });
    if let Some(origin) = proposal.get("origin") {
        event["origin"] = origin.clone();
    }
    events::append(db, &config.record_folder, &config.machine, &event)?;
    let back = format!("/?key={secret}");
    match copy::copy_events(config, &copy::stamp_now()?) {
        Ok(()) => Ok(Reply {
            status: 303,
            location: Some(back),
            body: String::new(),
        }),
        Err(e) => Ok(reply(
            200,
            format!(
                "{HEAD}<p>The mark is saved on this machine but did not reach the bucket: {}. \
                 <a href=\"{}\">Back to the list</a></p></body></html>",
                escape(&e.to_string()),
                escape(&back)
            ),
        )),
    }
}

pub fn stats(db: &Connection) -> Result<String> {
    let mut out = String::new();
    for (label, sql) in [
        ("mark", "json_extract(body, '$.mark')"),
        ("origin", "coalesce(json_extract(body, '$.origin'), 'none')"),
    ] {
        let mut statement = db.prepare(&format!(
            "SELECT {sql} AS name, count(*) FROM holding_judgements GROUP BY name ORDER BY name"
        ))?;
        let rows = statement.query_map([], |row| {
            Ok((row.get::<_, Option<String>>(0)?, row.get::<_, i64>(1)?))
        })?;
        for row in rows {
            let (name, count) = row?;
            out += &format!("{label} {}: {count}\n", name.unwrap_or_default());
        }
    }
    let waiting: i64 = db.query_row(&format!("SELECT count(*) FROM ({WAITING})"), [], |row| {
        row.get(0)
    })?;
    Ok(out + &format!("waiting: {waiting}\n"))
}

fn text(value: &Value) -> &str {
    value.as_str().unwrap_or_default()
}

fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn decode(form: &str) -> HashMap<String, String> {
    form.split('&')
        .filter_map(|pair| pair.split_once('='))
        .map(|(name, value)| (unescape(name), unescape(value)))
        .collect()
}

fn unescape(text: &str) -> String {
    let bytes = text.as_bytes();
    let mut out = vec![];
    let mut index = 0;
    while index < bytes.len() {
        let hex = bytes
            .get(index + 1..index + 3)
            .filter(|hex| hex.iter().all(u8::is_ascii_hexdigit))
            .and_then(|hex| std::str::from_utf8(hex).ok())
            .and_then(|hex| u8::from_str_radix(hex, 16).ok());
        match (bytes[index], hex) {
            (b'%', Some(byte)) => {
                out.push(byte);
                index += 3;
            }
            (b'+', _) => {
                out.push(b' ');
                index += 1;
            }
            (byte, _) => {
                out.push(byte);
                index += 1;
            }
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}
