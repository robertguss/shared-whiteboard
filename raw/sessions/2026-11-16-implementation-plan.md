# Session record: the implementation plan

Date: 16 November 2026 (UTC). Follows `2026-11-16-plan-review.md`.

## What the agent asked

What to do next with the changed requirements plan. Option 1 was to write the implementation plan.

## What Robert said, in their words

> 1

## What was done

The agent measured both session formats on the first machine, as field names and counts only, and wrote the implementation part of the plan: fourteen technical decisions, ten units, a verification table and a definition of done.

Two design checks and a five-reviewer document review followed. Their fixes are in the plan. The plan now has eleven units.

## What the research found

- Almost all bytes in session files are tool calls, thinking and request copies. The text Robert typed on the first machine is about 0.2 MB.
- A session file does not show whether Robert or an agent sent a prompt through T3 Code. T3 Code's own database does.
- About a third of the messages Robert typed are in sessions with no repository.
- rclone can keep the old version of a file it would overwrite, in a backup folder.
- exe.dev's proxy serves a port only to people with access to the machine, and adds a header that names the signed-in user.

## What the agent asked next

How a session with no repository should count when a connection needs two different projects. The agent recommended that each such session counts on its own.

## What Robert said, in their words

> B. Each scratch session counts on its own (Recommended)

## Decided

- A session with no repository is its own context. Two such sessions can make a connection; two quotes from one cannot.
- By the agent, as planning choices Robert can overturn: one Rust command-line program beside rclone; a coding agent runs the model step and the engine has no model client; a small T3 Code extract with no message text is copied with the session files; a message is Robert's only on positive evidence and is otherwise unknown; the event log is one file per machine per month; the page is served by the program behind exe.dev's private proxy.

## What the review changed

- The backup folder is named to the second and keeps each file's path, so a second run cannot overwrite a saved version.
- The copy no longer stops on a session file that grows while it uploads.
- Authors and contexts are worked out again on every read, so a late T3 row cannot leave a wrong author.
- A repository T3 Code does not know is still one project.
- A mark or a context correction is copied to the bucket at once.
- Proposals files live in a private folder, and the program refuses any other path.
- A mark needs the user header exe.dev's proxy adds.
- Agents that ask are told the returned text is private.
- The connection pass is built after the first judging, not before it.

## Still open

- The hand-run connections drew mostly on repositories. The engine holds sessions only, so some cannot go on the page. Robert has to choose what the first judging does about that.
- Whether T3 Code runs on the second machine.
- Whether exe.dev's proxy reaches a page that listens on the local address only.

## Not done

- No reviewer from another model provider read the implementation part. The council read the requirements only.
- rclone is not installed on the first machine, so no rclone command was run. Two flag behaviours were checked against rclone's documentation.
- The reviewers' claims about older Codex record shapes were not checked. The reader now reports record types it skips.
- Nothing is built.
