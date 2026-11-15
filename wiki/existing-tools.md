---
type: comparison
updated: 2026-11-15
sources: [raw/sources/2026-11-15-tool-scan.md, raw/sessions/2026-11-15-first-brainstorm.md]
---
# Existing tools

**Conclusion (proposed):** adopt none. Borrow three ideas. None of the tools was installed or run; this rests on READMEs and homepages.

| Tool | What it is | Free board | Status on notes | Timeline | Agent edits | Verdict |
| --- | --- | --- | --- | --- | --- | --- |
| canvai | Whiteboard shared with a coding agent, boards as JSON Canvas files in the repo | Yes | Not described | Ordered "rails" | MCP | Closest in shape. 3 stars, no push since July 2026. |
| Maproom | Mind maps with an MCP server | No, automatic layout | Yes | No | MCP | 1 star, one day of activity. |
| Vync | Local-first planning file synced between UI and editors | Not described | Not described | Not described | Edits the file | 1 star, no push since April 2026. |
| Nimbalyst | Desktop workspace for running coding agents, with a mind map editor | Mind map canvas | Not described | Not described | Edits `.mindmap` files | A whole workspace product, larger than this need. |
| Tinderbox | Long-standing note tool with a free map view | Yes | Yes, unverified | Yes, unverified | Recently added | Robert: does the free board well, but old, overcomplicated, AI bolted on. |
| Notion | Pages and databases | No | Yes | Database views | Yes | Robert: tried it, not visual enough. |
| Arango | Enterprise graph database platform | No surface at all | n/a | n/a | Queries | Wrong layer. Stores and retrieves; draws nothing. |

## Ideas worth borrowing

| Idea | From | Why |
| --- | --- | --- |
| JSON Canvas as the file format | canvai, Obsidian | An open format for boards of notes and links. Obsidian opens it. Unverified whether it can carry status and date fields cleanly. |
| A note the person drags stays put | canvai | The person's arrangement carries meaning the agent must not undo. |
| The agent sends structure and never coordinates | canvai | Keeps the agent on meaning and leaves placement to layout and to the person. |

## Gap none of them fills

No README in the scan describes one set of notes shown as a free board, a timeline and a done-versus-next view together.
