use crate::config::Config;
use crate::{Result, t3};
use std::ffi::OsStr;
use std::path::Path;
use std::process::Command;

pub fn copy(config: &Config, stamp: &str) -> Result<()> {
    let machine = &config.machine;
    let t3 = format!("sources/{machine}/t3");
    let local = config.record_folder.join(&t3);
    let mut results = vec![t3::extract(&config.t3_database, &local)];
    for (tool, folder) in &config.sources {
        let path = format!("sources/{machine}/{tool}");
        results.push(up(config, stamp, folder, &path));
    }
    if local.exists() {
        results.push(up(config, stamp, &local, &t3));
    }
    results.push(copy_events(config, stamp));
    results.into_iter().collect()
}

pub fn copy_events(config: &Config, stamp: &str) -> Result<()> {
    let path = format!("events/{}", config.machine);
    let local = config.record_folder.join(&path);
    if local.exists() {
        up(config, stamp, &local, &path)?;
    }
    Ok(())
}

fn up(config: &Config, stamp: &str, local: &Path, path: &str) -> Result<()> {
    let remote = config.remote.trim_end_matches('/');
    let backup = format!("{remote}/old/{stamp}/{path}");
    let flags = ["--backup-dir", &backup, "--local-no-check-updated"];
    rclone(local, format!("{remote}/{path}"), &flags)
}

pub fn pull(config: &Config) -> Result<()> {
    let remote = config.remote.trim_end_matches('/');
    let record = &config.record_folder;
    let machine = &config.machine;
    let events = format!("/events/{machine}/**");
    let t3 = format!("/sources/{machine}/t3/**");
    let others = [
        "--filter",
        &format!("- {events}"),
        "--filter",
        &format!("- {t3}"),
        "--filter",
        "+ /sources/**",
        "--filter",
        "+ /events/**",
        "--filter",
        "- *",
    ];
    rclone(remote, record, &others)?;
    let own = ["--include", &events, "--include", &t3, "--ignore-existing"];
    rclone(remote, record, &own)
}

pub fn stamp_now() -> Result<String> {
    now("%Y-%m-%dT%H%M%SZ")
}

pub fn time_now() -> Result<String> {
    now("%Y-%m-%dT%H:%M:%SZ")
}

fn now(format: &str) -> Result<String> {
    Ok(rusqlite::Connection::open_in_memory()?.query_row(
        "SELECT strftime(?1, 'now')",
        [format],
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
