use crate::config::Config;
use crate::{Result, t3};
use std::ffi::OsStr;
use std::path::Path;
use std::process::Command;

pub fn copy(config: &Config, stamp: &str) -> Result<()> {
    let remote = config.remote.trim_end_matches('/');
    let up = |local: &Path, path: &str| {
        let backup = format!("{remote}/old/{stamp}/{path}");
        let flags = ["--backup-dir", &backup, "--local-no-check-updated"];
        rclone(local, format!("{remote}/{path}"), &flags)
    };
    let machine = &config.machine;
    for (tool, folder) in &config.sources {
        up(folder, &format!("sources/{machine}/{tool}"))?;
    }
    let t3 = format!("sources/{machine}/t3");
    let extracted = t3::extract(&config.t3_database, &config.record_folder.join(&t3));
    for path in [t3, format!("events/{machine}")] {
        let local = config.record_folder.join(&path);
        if local.exists() {
            up(&local, &path)?;
        }
    }
    extracted
}

pub fn pull(config: &Config) -> Result<()> {
    let remote = config.remote.trim_end_matches('/');
    let record = &config.record_folder;
    let own = format!("/events/{}/**", config.machine);
    let others = [
        "--filter",
        &format!("- {own}"),
        "--filter",
        "+ /sources/**",
        "--filter",
        "+ /events/**",
        "--filter",
        "- *",
    ];
    rclone(remote, record, &others)?;
    rclone(remote, record, &["--include", &own, "--ignore-existing"])
}

pub fn stamp_now() -> Result<String> {
    Ok(rusqlite::Connection::open_in_memory()?.query_row(
        "SELECT strftime('%Y-%m-%dT%H%M%SZ', 'now')",
        [],
        |row| row.get(0),
    )?)
}

fn rclone(from: impl AsRef<OsStr>, to: impl AsRef<OsStr>, flags: &[&str]) -> Result<()> {
    let (from, to) = (Path::new(from.as_ref()), Path::new(to.as_ref()));
    let status = Command::new("rclone")
        .arg("copy")
        .arg(from)
        .arg(to)
        .args(flags)
        .status()
        .map_err(|e| format!("cannot run rclone: {e}"))?;
    if !status.success() {
        return Err(format!("rclone copy {} {} failed", from.display(), to.display()).into());
    }
    Ok(())
}
