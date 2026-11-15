---
type: overview
updated: 2026-11-15
sources: [raw/sessions/2026-11-15-first-brainstorm.md, raw/sessions/2026-11-15-shape-decision.md, raw/sessions/2026-11-15-data-first-reframe.md, raw/sessions/2026-11-15-engine-research-kickoff.md, raw/sessions/2026-11-15-north-star-open.md, raw/sources/2026-11-15-tool-scan.md]
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
| The data comes first; the board and other layouts are ways of displaying it. | Decided by Robert |
| The data is modelled on human memory: linked structure plus memory behaviour. | Decided by Robert. See [memory model](memory-model.md). |
| Whether to design a new database engine for it. | Open. Robert wants to consider it. First research pass: [engine research](engine-research.md). |
| Brain-like memory for an agent alone is crowded; memory for a person and an agent thinking together is open. | Proposed, from one research pass |
| The storage core is part of the research, because its design may influence the memory and association layer. | Decided by Robert |
| The north star. | Not known yet. "Helps produce new ideas" is not accepted as the measure. See [open questions](open-questions.md). |
| The project is meant to be ambitious, and learning and enjoyment are goals in themselves. | Stated by Robert |
| No existing tool is adopted; five ideas are borrowed. | Proposed |
| No Arango or other server database. | Proposed |

Details: [requirements](requirements.md), [shape options](shape-options.md), [memory model](memory-model.md), [existing tools](existing-tools.md).

## Next

Find the north star. Research how the storage core could shape the memory layer. Then read the key papers in full, work through the [open questions](open-questions.md), and write a requirements plan.

## An idea to hold

This wiki is itself a set of notes with links between them. The finished tool could draw it. Unverified, and not a commitment.
