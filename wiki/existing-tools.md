---
type: comparison
updated: 2026-11-15
sources: [raw/sources/2026-11-15-tool-scan.md, raw/sources/2026-11-15-thebrain.md, raw/sources/2026-11-15-cass.md, raw/sessions/2026-11-15-first-brainstorm.md, raw/sessions/2026-11-15-data-first-reframe.md]
---
# Existing tools

**Conclusion (proposed):** adopt none of the visual tools and borrow five ideas from them. For reading and keeping agent sessions, try cass before building anything; see the second section. None of the tools was installed or run; this rests on READMEs and homepages.

| Tool | What it is | Free board | Status on notes | Timeline | Agent edits | Verdict |
| --- | --- | --- | --- | --- | --- | --- |
| canvai | Whiteboard shared with a coding agent, boards as JSON Canvas files in the repo | Yes | Not described | Ordered "rails" | MCP | Closest in shape. 3 stars, no push since July 2026. |
| Maproom | Mind maps with an MCP server | No, automatic layout | Yes | No | MCP | 1 star, one day of activity. |
| Vync | Local-first planning file synced between UI and editors | Not described | Not described | Not described | Edits the file | 1 star, no push since April 2026. |
| Nimbalyst | Desktop workspace for running coding agents, with a mind map editor | Mind map canvas | Not described | Not described | Edits `.mindmap` files | A whole workspace product, larger than this need. |
| Tinderbox | Long-standing note tool with a free map view | Yes | Yes, unverified | Yes, unverified | Recently added | Robert: does the free board well, but old, overcomplicated, AI bolted on. |
| Notion | Pages and databases | No | Yes | Database views | Yes | Robert: tried it, not visual enough. |
| TheBrain | Network of "thoughts" joined by parent, child and jump links, seen around one active thought | No, layout is computed | Not described | Not described | Built-in assistant; outside agents unverified | Robert: reference and inspiration. Best source for the linking model. Shares Tinderbox's weaknesses: 28 years old, "150 new features", AI added in version 15. Free, so cheap to try. |
| Arango | Enterprise graph database platform | No surface at all | n/a | n/a | Queries | Looked at twice. Right kind of data (a graph), wrong weight: a server product for a job a single local file can do. Draws nothing. |

## Ideas worth borrowing

| Idea | From | Why |
| --- | --- | --- |
| JSON Canvas as the file format | canvai, Obsidian | An open format for boards of notes and links. Obsidian opens it. Unverified whether it can carry status and date fields cleanly. |
| A note the person drags stays put | canvai | The person's arrangement carries meaning the agent must not undo. |
| The agent sends structure and never coordinates | canvai | Keeps the agent on meaning and leaves placement to layout and to the person. |
| Three kinds of link: parent, child, jump | TheBrain | A thought can sit under several parents and still link sideways. Simple enough to learn in a minute. |
| A focus view around one active thought | TheBrain | Shows a large network a neighbourhood at a time. A possible answer to scale. |

## Tools that read agent sessions

Looked at on 15 November 2026 at Robert's request, after the decision to start from agent session files. Source: [cass and cass-memory](../raw/sources/2026-11-15-cass.md).

| Tool | What it does | Overlap with the first engine | Verdict (proposed) |
| --- | --- | --- | --- |
| cass | Reads the session files of 34 agent tools into one common form in SQLite, searches them, pulls from other machines, and keeps its own copy because tools delete their logs | The whole lower half: collect, keep, read the formats, search | Try it on this machine before building that half. Active: pushed the day it was read. |
| cass-memory | Turns sessions into rules for coding agents, scored by feedback that fades | Little. Its unit is a rule for an agent. It has no ideas, no links between projects, no person judging, no picture. | Do not adopt. Borrow four ideas. |

What neither does is the part Robert called the interesting problem: connections between ideas across projects, judged by the person.

| Idea | From | Why |
| --- | --- | --- |
| A model proposes; fixed code decides | cass-memory | A model that keeps rewriting its own summaries drifts toward the generic. Fixed rules for accepting and retiring are repeatable and can be checked. |
| Every judgement is a stored event; the score is worked out from the events | cass-memory | The same as "use-events with derived strength" in [engine research](engine-research.md), already working in a real tool. It is also the judgement log this project needs. |
| A proposal must be backed by sessions that can be found | cass-memory | A proposed connection should carry evidence the person can open. |
| The raw session files and a keep-forever copy are the truth; every index is rebuilt from them | cass | The same split as record and engine in [the problem](the-problem.md). |

**Licence.** Both carry a rider that grants no rights to OpenAI, Anthropic or anyone acting for them, and forbids making the code or work derived from it available to them. No code from either is copied into this public repo. Whether an agent running on one of those companies' models may operate the tools for Robert is not settled here; it is Robert's call.

## Gap none of them fills

No README in the scan describes one set of notes shown as a free board, a timeline and a done-versus-next view together.
