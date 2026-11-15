---
type: overview
updated: 2026-11-15
sources: [raw/sessions/2026-11-15-first-brainstorm.md, raw/sessions/2026-11-15-shape-decision.md, raw/sessions/2026-11-15-data-first-reframe.md, raw/sources/2026-11-15-tool-scan.md]
---
# Overview

**What this is:** a visual work surface that a person and AI agents both read and edit. Part whiteboard, part roadmap, part project history.

**Where it stands:** exploring. One prototype exists and the shape is chosen: a free board plus timeline and status views. Nothing is planned for build.

## The problem

Brainstorming with AI produces markdown faster than a person can hold it in their head. The Mo project produced 181 documents and 826 commits in five weeks. In text and tables it is hard to see how ideas connect, how the project got here, what is done and what is next.

## What is known

| Statement | Standing |
| --- | --- |
| It must be visual first. Pages and databases did not work. | Decided by Robert |
| Both the person and the agent read and edit the same thing. | Decided by Robert |
| The agent does the upkeep. | Decided by Robert |
| The main surface is a free board where notes can be placed anywhere. | Decided by Robert |
| Notes also carry status and date, so timeline and status are other views of the same notes. | Decided by Robert (shape B) |
| The board is a plain file in the project's repo. | Proposed |
| The data comes first; the board and other layouts are ways of displaying it. | Raised by Robert, being explored |
| The data is modelled on how human memory works. | Raised by Robert, being explored. See [memory model](memory-model.md). |
| That means a new data model, not a new storage engine. | Proposed |
| No existing tool is adopted; five ideas are borrowed. | Proposed |
| No Arango or other server database. | Proposed |

Details: [requirements](requirements.md), [shape options](shape-options.md), [memory model](memory-model.md), [existing tools](existing-tools.md).

## Next

Settle how literally "like the brain" is meant, then work through the rest of the [open questions](open-questions.md), then write a requirements plan.

## An idea to hold

This wiki is itself a set of notes with links between them. The finished tool could draw it. Unverified, and not a commitment.
