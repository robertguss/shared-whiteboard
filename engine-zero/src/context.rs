use crate::config::Config;
use crate::{Result, copy, events, store};
use rusqlite::{Connection, params};
use serde_json::json;

const CORRECTIONS: &str = "
SELECT prefix, context FROM (
  SELECT json_extract(body, '$.prefix') AS prefix, json_extract(body, '$.context') AS context,
    row_number() OVER (PARTITION BY json_extract(body, '$.prefix') ORDER BY time DESC, file DESC, line DESC) AS rank
  FROM events WHERE kind = 'context_correction'
    AND json_extract(body, '$.prefix') IS NOT NULL AND json_extract(body, '$.context') IS NOT NULL)
WHERE rank = 1";

const ROOTS: &str =
    "SELECT DISTINCT workspace_root FROM t3_projects WHERE workspace_root IS NOT NULL";

const PAIRS: &str = "SELECT DISTINCT cwd, repo_url FROM items";

const NAMES: &str = "
CREATE TEMP TABLE IF NOT EXISTS names (cwd TEXT, repo_url TEXT, name TEXT, UNIQUE (cwd, repo_url));
DELETE FROM temp.names";

const APPLY: &str = "
UPDATE items SET context = coalesce(n.name, CASE WHEN items.cwd IS NOT NULL THEN 'session:' || items.session_id END)
FROM temp.names n WHERE n.cwd IS items.cwd AND n.repo_url IS items.repo_url";

pub fn set(config: &Config, prefix: &str, name: &str, caller: &str) -> Result<()> {
    let mut db = store::open(&config.database)?;
    let event = json!({
        "kind": "context_correction", "time": copy::time_now()?,
        "writer": {"by": "command", "command": "context set", "caller": caller},
        "prefix": prefix, "context": name,
    });
    events::append(&mut db, &config.record_folder, &config.machine, &event)?;
    settle(&db, &config.project_parents, &config.temporary_prefixes)?;
    copy::copy_events(config, &copy::stamp_now()?).map_err(|e| {
        format!("the correction is saved on this machine but did not reach the bucket: {e}").into()
    })
}

pub fn settle(db: &Connection, parents: &[String], temporary: &[String]) -> Result<()> {
    let corrections: Vec<(String, String)> = rows(db, CORRECTIONS)?;
    let roots: Vec<String> = db
        .prepare(ROOTS)?
        .query_map([], |row| row.get(0))?
        .collect::<rusqlite::Result<Vec<String>>>()?
        .into_iter()
        .filter(|root| !root.trim_end_matches('/').ends_with("/.t3/scratch"))
        .filter(|root| !temporary.iter().any(|prefix| contains(prefix, root)))
        .collect();
    let pairs: Vec<(Option<String>, Option<String>)> = rows(db, PAIRS)?;
    db.execute_batch(NAMES)?;
    let mut insert = db.prepare("INSERT INTO temp.names VALUES (?1, ?2, ?3)")?;
    for (cwd, url) in pairs {
        let from_cwd = cwd.as_deref().and_then(|cwd| {
            let correction = corrections
                .iter()
                .filter(|(prefix, _)| contains(prefix, cwd))
                .max_by_key(|(prefix, _)| prefix.len());
            correction
                .map(|(_, name)| name.clone())
                .or_else(|| worktree(cwd))
                .or_else(|| project(cwd, &roots, parents))
        });
        let name = from_cwd.or_else(|| url.as_deref().and_then(repository));
        insert.execute(params![cwd, url, name])?;
    }
    db.execute(APPLY, [])?;
    Ok(())
}

fn rows<T: rusqlite::types::FromSql, U: rusqlite::types::FromSql>(
    db: &Connection,
    sql: &str,
) -> Result<Vec<(T, U)>> {
    let mut statement = db.prepare(sql)?;
    let rows = statement.query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?;
    Ok(rows.collect::<rusqlite::Result<_>>()?)
}

fn contains(root: &str, path: &str) -> bool {
    let rest = path.strip_prefix(root.trim_end_matches('/'));
    rest.is_some_and(|rest| rest.is_empty() || rest.starts_with('/'))
}

fn worktree(cwd: &str) -> Option<String> {
    let segments: Vec<&str> = cwd.split('/').collect();
    segments.windows(3).find_map(|window| match window {
        [".t3", "worktrees", repo] | [repo, ".claude", "worktrees"] => Some(repo.to_string()),
        _ => None,
    })
}

fn project(cwd: &str, roots: &[String], parents: &[String]) -> Option<String> {
    let children = parents.iter().filter_map(|parent| {
        let parent = parent.trim_end_matches('/');
        let rest = cwd.strip_prefix(parent)?.strip_prefix('/')?;
        let child = rest.split('/').next().filter(|child| !child.is_empty())?;
        Some(format!("{parent}/{child}"))
    });
    let root = roots
        .iter()
        .filter(|root| contains(root, cwd))
        .map(|root| root.trim_end_matches('/').to_string())
        .chain(children)
        .max_by_key(String::len)?;
    root.rsplit('/').next().map(Into::into)
}

fn repository(url: &str) -> Option<String> {
    let last = url.trim_end_matches('/').rsplit(['/', ':']).next()?;
    let name = last.strip_suffix(".git").unwrap_or(last);
    (!name.is_empty()).then(|| name.to_string())
}
