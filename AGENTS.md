# Shared whiteboard: agent guide

This repo holds one project: a database that acts as an external form of one person's brain across all their projects and ideas, which AI agents read, understand and add to. It began as a visual whiteboard, and the visual views remain ways of showing the data. It is the project's knowledge base now and will hold its code later.

The knowledge base is an LLM wiki, after Andrej Karpathy's pattern (`raw/sources/2026-11-15-karpathy-llm-wiki.md`). The agent writes and maintains the wiki. The person supplies sources, asks questions and decides.

## Layout

| Path | What it holds | Who writes it |
| --- | --- | --- |
| `raw/sessions/` | One record per working session | Agent, once, at the end of the session |
| `raw/sources/` | Third-party material | Agent or person, once |
| `wiki/` | Synthesised pages | Agent, continuously |
| `wiki/index.md` | Catalogue of every wiki page | Agent, on every change |
| `wiki/log.md` | Append-only history | Agent, on every change |
| `prototypes/` | Numbered throwaway prototypes | Agent |
| `docs/plans/`, `docs/solutions/` | Compound Engineering plans and learnings, created when first needed | Agent |

## Rules

1. `raw/` is immutable. Never edit or delete a file there after its first commit. Correct a mistake with a new file that names the old one.
2. This repo is public. A third-party source goes in `raw/sources/` as a stub: URL, date retrieved, short quoted excerpts, and a factual summary. Store a full copy only when the licence allows it or the material is ours.
3. Nothing from a private source goes in this repo unless the person says so. When in doubt, leave it out and say what was left out. Private working material lives outside the repo in `~/code/shared-whiteboard-private/`, which is not under version control: extracts of the person's agent sessions, findings about their projects, and names of private repositories.
4. Every claim in `wiki/` either cites a file in `raw/` or is marked `unverified`. A recommendation the person has not confirmed is marked `proposed`.
5. Wiki pages live flat in `wiki/` with kebab-case names. Add folders only when the index stops being readable.
6. Link pages with relative markdown links so they work on GitHub.
7. Keep pages short. Lead with the conclusion and prefer tables to paragraphs. The person thinks visually and finds long text hard to use. Split a page that passes about 150 lines.

## Page format

```markdown
---
type: overview | concept | comparison | entity | question
updated: YYYY-MM-DD
sources: [raw/sessions/..., raw/sources/...]
---
# Title
```

## Workflows

**Start of a session.** Read `wiki/index.md`, then `wiki/overview.md`, then the last five log entries: `grep "^## \[" wiki/log.md | tail -5`.

**Ingest a source.** Save the stub or file in `raw/sources/`. Tell the person the key takeaways. Update every wiki page the source changes, flag any contradiction with what the wiki already says, update the index, append to the log.

**End of a session.** Write the session record in `raw/sessions/`: what the person said in their own words, what was found, what was decided, what stayed open. Then update the wiki from it, the index and the log.

**Answer a question.** Read the index, read the relevant pages, answer with citations. If the answer is worth keeping, file it as a wiki page.

**Lint.** On request, check for contradictions between pages, claims a newer source has overtaken, pages nothing links to, concepts mentioned without a page, and gaps a search could fill.

## Log format

Each entry starts with `## [YYYY-MM-DD] kind | title`, where kind is `ingest`, `session`, `query`, `lint` or `decision`.
