# Connection pass

A connection pass reads the person's own messages and proposes connections between ideas they had in different places. The program checks each proposal against fixed rules, and the person later judges the ones that pass.

## Before you start

- Run a pass only when the owner asks for one, and only in the agent session they started it in.
- Session text is private. Do not copy it into any repository, issue, pull request, web request or other tool. Do not send it to any other model or service.

## Steps

1. Run `engine-zero candidates`. Add `--from <YYYY-MM-DD>` to see only messages from that date on. Each printed line is one message: `key`, `time`, `context` and `text`.
2. Restate each message as a mechanism: the idea in plain words, without its project's vocabulary. `wiki/hand-run-test.md`, at the root of this repository, explains how, and lists the six link kinds.
3. Look for pairs or groups of messages from different contexts that share a mechanism or stand in one of the six link kinds.
4. For each one, write one proposal line:
   - `kind`: one of the six link kinds, spelled exactly as in `wiki/hand-run-test.md`.
   - `statement`: one sentence that says what the connection is.
   - `quotes`: at least two quotes. Copy each quote character for character from the `text` of the message it comes from, and give that message's `key`.
   - `origin`: a label that names the pass and its date, such as `connection pass 2026-01-31`.
5. Save the lines as a new file in the private proposals folder named by `proposals_folder` in the config (`~/.config/engine-zero/config.json`). It is usually `~/.local/share/engine-zero/proposals/`. Never put the file inside a repository. The file shape, with an example line, is in the README under "The proposals file".
6. Run `engine-zero propose <file> --caller "<agent name> connection pass"`.
7. Report every printed line to the owner, including each reason for a drop.

Do not rewrite a dropped proposal to get past a rule. A quote that is not found means the quote was not exact.

## What the rules drop

Save your effort: these proposals are always dropped.

- Quotes from an agent. They do not count as the person's.
- A quote not found in the message its key names.
- Fewer than two quotes from the person's messages that have a `time`.
- Quotes from one context only. A message with no context counts toward none.
- One quote inside another. The two count as one.
- A connection the person has already judged.

## Never

- Call `engine-zero ask` during a pass.
- Write or edit any file under the record folder. Only the program writes events.
- Open, fetch or post to the judging page.
- Mark or judge anything.
