# LLM Wiki, by Andrej Karpathy

- URL: https://gist.github.com/karpathy/442a6bf555914893e9891c11519de94f
- Retrieved: 2026-11-15
- Stored as: stub. The full text stays at the URL.

## Summary

An idea file describing a pattern for personal knowledge bases. Instead of retrieving from raw documents on every question, an LLM builds and maintains a persistent wiki of interlinked markdown files, so knowledge is compiled once and kept current.

Three layers:

| Layer | Role |
| --- | --- |
| Raw sources | Immutable source documents. The LLM reads them and never changes them. |
| The wiki | Markdown pages the LLM writes and maintains: summaries, entity pages, concept pages, comparisons, a synthesis. |
| The schema | A file such as `CLAUDE.md` or `AGENTS.md` that tells the LLM the structure, conventions and workflows. |

Three operations: ingest a source, query the wiki, lint the wiki.

Two navigation files: `index.md`, a catalogue by category that the LLM reads first, and `log.md`, an append-only record with a consistent entry prefix so it can be read with `grep`.

## Excerpts

> the wiki is a persistent, compounding artifact.

> You never (or rarely) write the wiki yourself — the LLM writes and maintains all of it.

> good answers can be filed back into the wiki as new pages.

> Humans abandon wikis because the maintenance burden grows faster than the value.

> This document is intentionally abstract. It describes the idea, not a specific implementation.

## Details worth remembering

- The author browses the wiki in Obsidian while the agent edits, and uses Obsidian's graph view to see the wiki's shape.
- At moderate scale, about 100 sources and hundreds of pages, the index file replaces search infrastructure.
- Optional later tooling: a local markdown search engine such as qmd.
