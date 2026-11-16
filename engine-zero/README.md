# engine-zero

Copies Claude Code and Codex session files, and a small extract of T3 Code's database, from each machine to one private bucket, and pulls the bucket into a local record folder.

The copy never removes or overwrites anything in the bucket. Before rclone replaces a bucket file that grew or changed, it moves the old version to `old/<UTC time>/<same path>`. A file deleted on the machine stays in the bucket.

## Install

1. Install rclone: `brew install rclone` on macOS, or see <https://rclone.org/install/>. It must be on `PATH`.
2. Add the bucket as an rclone remote with `rclone config`. The credentials live in rclone's own config, never in this repository.
3. Build and install the program:

   ```sh
   cd engine-zero
   cargo build --release
   cp target/release/engine-zero ~/.local/bin/
   ```

## Config

The program reads `~/.config/engine-zero/config.json`, or the file given with `--config <path>`. All paths are absolute.

```json
{
  "machine": "linux-box",
  "remote": "r2:bucket-name",
  "sources": {
    "claude": "/home/you/.claude/projects",
    "codex": "/home/you/.codex/sessions"
  },
  "t3_database": "/home/you/.t3/userdata/statev2.sqlite",
  "record_folder": "/home/you/.local/share/engine-zero/record",
  "database": "/home/you/.local/share/engine-zero/engine.sqlite"
}
```

- `machine` has no default. **Each machine must use a different name**, because the bucket keeps each machine's files under `sources/<machine>/` and `events/<machine>/`.
- `remote` is anything rclone accepts as a destination, such as `r2:bucket-name` or a local folder.
- `sources` maps a tool name to its session folder. List only the tools installed on that machine.
- `t3_database` is skipped when the file does not exist.
- `record_folder` and `database` must be outside this repository; the program refuses to run otherwise.

## Commands

- `engine-zero copy` copies each source folder to `sources/<machine>/<tool>/`, writes the T3 extract to `<record_folder>/sources/<machine>/t3/`, then copies the extract and `<record_folder>/events/<machine>/` to the bucket.
- `engine-zero pull` copies `sources/` and `events/` from the bucket into the record folder, leaves `old/` out, and never replaces this machine's own event files; it only restores a missing one.
- `engine-zero read` reads what is new in the record folder's `.jsonl` files into the database, which it creates on first use. Each file resumes where the last read stopped, a last line without a newline waits for a later read, and a file that became shorter is read again from the start. Secrets are masked in what is stored; the record files are never changed. At the end of every read, the author of each prompt is worked out again from the T3 extract. It prints how many items were added, how many broken lines were skipped, and, for each tool, how many records were stored and skipped by record type. The database can be deleted at any time; the next `read` builds it again.

## Daily schedule

Run `copy` once a day on both machines. Create the log folder first: `mkdir -p ~/.local/share/engine-zero`.

Linux, with `crontab -e`:

```cron
17 3 * * * PATH=$HOME/.local/bin:/usr/local/bin:/usr/bin:/bin engine-zero copy >> $HOME/.local/share/engine-zero/copy.log 2>&1
```

macOS: cron skips a run while the Mac sleeps, so use launchd. Save this as `~/Library/LaunchAgents/engine-zero.copy.plist`, replacing `you` with your user name, then run `launchctl load ~/Library/LaunchAgents/engine-zero.copy.plist`:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>Label</key><string>engine-zero.copy</string>
  <key>ProgramArguments</key>
  <array><string>/Users/you/.local/bin/engine-zero</string><string>copy</string></array>
  <key>EnvironmentVariables</key>
  <dict><key>PATH</key><string>/opt/homebrew/bin:/usr/local/bin:/usr/bin:/bin</string></dict>
  <key>StartCalendarInterval</key>
  <dict><key>Hour</key><integer>3</integer><key>Minute</key><integer>17</integer></dict>
  <key>StandardErrorPath</key><string>/Users/you/.local/share/engine-zero/copy.log</string>
</dict>
</plist>
```

## Development

`cargo test` runs rclone between temporary local folders, so rclone must be installed.
