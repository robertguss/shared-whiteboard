# Shared whiteboard

A visual work surface that a person and AI agents both read and edit: part whiteboard, part roadmap, part project history.

**Status:** exploring. Nothing is built yet beyond one throwaway prototype. `shared-whiteboard` is a working name.

## Where to look

| You want | Open |
| --- | --- |
| The current thinking in one page | [wiki/overview.md](wiki/overview.md) |
| Every page in the knowledge base | [wiki/index.md](wiki/index.md) |
| What happened, in order | [wiki/log.md](wiki/log.md) |
| The prototype | [prototypes/001-status-and-history/three-shapes.html](prototypes/001-status-and-history/three-shapes.html), download and open in a browser |
| How agents work in this repo | [AGENTS.md](AGENTS.md) |

## How this repo works

It is an LLM wiki. Sources and session records go in `raw/` and are never changed. An AI agent reads them and keeps the pages in `wiki/` current. The person reads, asks and decides.
