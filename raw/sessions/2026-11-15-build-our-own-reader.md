# Session record: build our own reader

Date: 15 November 2026. Follows `2026-11-15-sqlite-first.md` and the ingest of `raw/sources/2026-11-15-cass.md`.

## What the agent asked

How to handle cass, given its licence: Robert installs and runs it, or the project writes its own reader for the session formats.

## What Robert said, in their words

> Build our own because otherwise cass becomes a dependency and I dont want that. We can fork and use it as a base to get started if you want though.

## Decided

- The project writes its own reader for agent session files. cass is not a dependency.
- Whether to fork cass as a starting base was left to the agent.

## The agent's call on the fork

No fork. Three reasons.

1. A fork is a work derived from cass. Its licence rider forbids making derived work available to OpenAI or Anthropic. A fork in a public repository, worked on by an agent that runs on one of those companies' models, sits inside the unclear part of that rider.
2. cass is large: 34 connectors, a terminal interface, its own search engine. The first engine needs two formats read.
3. The ideas are what is wanted, and ideas can be used freely. They are listed in `wiki/existing-tools.md`.
