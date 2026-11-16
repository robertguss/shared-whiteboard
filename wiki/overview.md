---
type: overview
updated: 2026-11-16
sources: [raw/sessions/2026-11-15-first-brainstorm.md, raw/sessions/2026-11-15-shape-decision.md, raw/sessions/2026-11-15-data-first-reframe.md, raw/sessions/2026-11-15-engine-research-kickoff.md, raw/sessions/2026-11-15-north-star-open.md, raw/sessions/2026-11-15-external-brain.md, raw/sessions/2026-11-15-why-it-keeps-coming-back.md, raw/sessions/2026-11-15-transcripts-as-the-record.md, raw/sessions/2026-11-15-sqlite-first.md, raw/sessions/2026-11-15-build-our-own-reader.md, raw/sessions/2026-11-16-scope-confirmed-rust.md, raw/sessions/2026-11-16-plan-review.md, raw/sessions/2026-11-16-implementation-plan.md, raw/sessions/2026-11-16-engine-zero-built.md, raw/sources/2026-11-15-tool-scan.md]
---
# Overview

**What this is:** a database that acts as an external form of one person's brain, across all their projects and ideas, which AI agents can read, understand and add to. The agent makes or proposes connections the person would not see unaided. Visual views show the same data as a board, a timeline and a status view.

**Where it stands:** the first engine, [engine zero](../engine-zero/README.md), is built from its [plan](../docs/plans/2026-11-16-0113-feat-engine-zero-plan.md) and runs on the first machine's session files: 18,056 items from 926 sessions. 28 proposed connections wait for Robert to judge, so nothing is known yet about whether the connections are real. See [engine zero notes](engine-zero-notes.md). The project began on 15 November 2026 as a visual whiteboard and moved, the same day, down to the data and the engine under it.

**What it is not:** a "second brain" or a personal knowledge management tool. Robert sees the overlap and wants something bigger and deeper than linked notes.

## The problem

One person's projects and ideas overlap and inform one another. There is no single place where those connections can be made except the person's own head, and even there they are hard to see. Brainstorming with AI makes it worse: it produces text faster than anyone can hold it. One project alone, Mo, produced 181 documents and 826 commits in five weeks.

## What is known

| Statement | Standing |
| --- | --- |
| It must be visual first. Pages and databases did not work. | Decided by Robert |
| Both the person and the agent read and edit the same thing. | Decided by Robert |
| The agent does the upkeep. | Decided by Robert |
| The main surface is a free board where notes can be placed anywhere. | Decided by Robert |
| Notes also carry status and date, so timeline and status are other views of the same notes. | Decided by Robert (shape B) |
| The board is a plain file in the project's repo. | Proposed |
| The data comes first; the board and other layouts are ways of displaying it. | Decided by Robert |
| The data is modelled on human memory: linked structure plus memory behaviour. | Decided by Robert. See [memory model](memory-model.md). |
| Whether to design a new database engine for it. | Open. Robert wants to consider it. First research pass: [engine research](engine-research.md). |
| The first engine is a throwaway on SQLite, run on Robert's real session files. A custom storage core is designed from what it shows. | Decided by Robert |
| The source of the record is the agent session files from both of Robert's machines, copied to one bucket. | Decided by Robert |
| The project writes its own reader for session files. cass is neither a dependency nor a fork. | Decided by Robert |
| The first engine is written in Rust. | Decided by Robert |
| The first engine has no board. Robert judges connections on a plain page. | Decided by Robert, on the agent's proposal |
| The first milestone is the copy, the reader, the page, and Robert judging the hand-run connections. | Decided by Robert, on the agent's proposal |
| The first reader stores messages only. Decisions, attempts and stances wait. | Decided by Robert, on the agent's proposal |
| Secrets are masked in what the engine stores. The bucket copy stays raw. | Decided by Robert, on the agent's proposal |
| Agents asking the engine stays in the first version and is built last. | Decided by Robert, on the agent's proposal |
| A session with no repository counts as its own context when a connection needs two. | Decided by Robert, on the agent's proposal |
| One Rust program beside rclone; a coding agent runs the model step; a message is Robert's only on positive evidence. | Proposed, in the plan |
| Brain-like memory for an agent alone is crowded; memory for a person and an agent thinking together is open. | Proposed, from one research pass |
| The storage core is part of the research, because its design may influence the memory and association layer. | Decided by Robert |
| It covers all of Robert's projects and ideas in one place. Mo is one part of it. | Decided by Robert |
| The contents of the brain are private; the design and the engine are public. | Proposed |
| The north star. | Closer. Proposed wording: one memory across everything you work on, in which you and the agent both think, that surfaces the connections neither would see alone. "Helps produce new ideas" was not accepted. See [open questions](open-questions.md). |
| The project is meant to be ambitious, and learning and enjoyment are goals in themselves. | Stated by Robert |
| No existing tool is adopted; five ideas are borrowed. | Proposed |
| No Arango or other server database. | Proposed |
| Robert has returned to this idea many times; the recurrence is evidence the problem is real, and it is to be solved now. | Stated by Robert |
| The record of Robert's thinking must never restart; engines and views are experiments over it. | Accepted by Robert for now, tentatively. See [the problem](the-problem.md). |

Details: [the problem](the-problem.md), [requirements](requirements.md), [shape options](shape-options.md), [memory model](memory-model.md), [existing tools](existing-tools.md).

## Next

Robert judges the 28 waiting connections on the page. After that the agent writes the baseline in [engine zero notes](engine-zero-notes.md) and runs a first real connection pass. Robert provides the bucket and its credentials, and still has to check whether old session files exist on the second machine and set its retention; until then a local folder stands in for the bucket. One question is open in the plan: what to do with hand-run connections that have no session quotes. The engine research continues beside it.

## An idea to hold

This wiki is itself a set of notes with links between them. The finished tool could draw it. Unverified, and not a commitment.
