---
title: A SQLite transaction that reads before it writes fails at once when another process holds the write lock
date: 2026-11-16
category: database-issues
module: engine-zero
problem_type: database_issue
component: database
symptoms:
  - "A mark on the judging page returned an error page saying database is locked while another engine-zero command was writing"
  - "The failure came with no wait, although the connection had a busy timeout of several seconds"
  - "The event line was already in the event file when the command failed"
root_cause: concurrency
resolution_type: code_fix
severity: medium
tags: [sqlite, rusqlite, busy-timeout, transaction, locking]
---

# A SQLite transaction that reads before it writes fails at once when another process holds the write lock

## Problem

Several `engine-zero` commands share one SQLite file: the daily `read`, the judging page, `ask` and `propose`. When two of them wrote at the same time, the second failed at once with "database is locked". It did not wait. For a page mark this meant the judgement was in the event file, but the upload to the bucket was skipped and the owner saw an error.

## Symptoms

- "database is locked" with no delay, while another process holds a write transaction.
- The busy timeout is set and has no effect.
- The event file has the new line, and the database does not.

## What Didn't Work

- Relying on the busy timeout. rusqlite sets one when it opens a connection, and it does make a plain write wait. It does not help here.

## Solution

Open the transaction with the write lock taken up front, in `engine-zero/src/ingest.rs` (`read_path`):

```rust
let tx = db.transaction_with_behavior(TransactionBehavior::Immediate)?;
```

The old code used `db.transaction()`, which is deferred.

## Why This Works

A deferred transaction takes no lock until its first statement. `read_path` first reads the saved position of the file, which takes a read lock, and then inserts, which needs the write lock. SQLite does not call the busy handler when a connection that already holds a read lock asks to upgrade it, because waiting there could deadlock two connections. So the insert fails immediately, whatever the timeout.

An immediate transaction asks for the write lock at `BEGIN`, before any read. The connection holds no lock yet, so the busy handler runs and the command waits its turn.

The event file made the failure worse. `events::append` writes the line to the file first and reads it into the database second, so a failed transaction left the two out of step until the next `read`.

## Prevention

- Any transaction that reads before it writes, on a database that more than one process can open, takes the write lock up front.
- The test `an_event_appended_while_another_command_holds_the_database_waits_and_is_stored` in `engine-zero/tests/rebuild.rs` holds a write lock on a second connection and appends an event. It fails with `DatabaseBusy` on a deferred transaction.
- Write-ahead logging was considered and rejected for this engine: a rebuild deletes the database file, and a leftover `-wal` file beside a new database is a risk.

## Related Issues

- Found by the code review of 16 November 2026. See `wiki/engine-zero-notes.md`.
