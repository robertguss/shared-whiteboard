---
type: concept
updated: 2026-11-15
sources: [raw/sessions/2026-11-15-data-first-reframe.md, raw/sources/2026-11-15-brain-inspired-memory-scan.md, raw/sources/2026-11-15-thebrain.md, raw/sources/2026-11-15-rieseberg-ambition.md]
---
# Memory model: the data under the views

**Standing:** proposed. Robert raised the direction; nothing on this page is confirmed.

**The idea:** design the data first. The board, timeline and status views from [shape B](shape-options.md) are ways of showing it. The data is modelled on how human memory works, so it suits brainstorming.

## Two layers

| Layer | In the brain | Here | Seen as |
| --- | --- | --- | --- |
| Episodes | Memory of events, in order | An append-only record of what happened: sessions, decisions, pivots, status changes | Timeline |
| Concepts | Knowledge distilled from events | Thoughts and the links between them | Board, status view, focus view |

This repo already works this way: `raw/` holds episodes and `wiki/` holds concepts.

## Mechanisms worth borrowing

| Mechanism | What it would do here | Known from |
| --- | --- | --- |
| Association | A thought lives in every context it belongs to. Links are data, with a kind. | TheBrain |
| Recall by spreading | Start from what is in focus and follow links outward to find what is related | HippoRAG, SYNAPSE |
| Strength from use | Thoughts and links used together often grow stronger | Scan, as "Hebbian" |
| Fading | Unused thoughts drop out of sight. Nothing is deleted. | Scan, as forgetting curves |
| Consolidation | The agent periodically turns episodes into concepts, merges duplicates and proposes links | Scan, as "sleep" |
| Focus | Only a few thoughts are in view at once. The board is working memory; the data is long-term memory. | Unverified, from general knowledge |

## A rule to hold (proposed)

Facts are exact and salience is fuzzy. Status, dates and links are plain data. Strength and fading change only what is shown first. A roadmap cannot be allowed to forget that a feature shipped.

## What this is not (proposed)

- Not a new storage engine. A personal brain is thousands of thoughts, which an ordinary single-file database holds easily. The scan found no project whose contribution is a storage engine.
- Not a simulation of a brain. These are models of memory borrowed from psychology, simplified.
- Not Arango. See [existing tools](existing-tools.md).

## What would be new

Every brain-inspired memory system in the scan serves the agent alone. None is also a surface the person looks at and rearranges. A memory the person and the agent share and both shape is the gap.

## How we would know it works

An outside judge, as the Rieseberg article advises: load Mo, write down questions with known answers ("what was dropped at the reset?", "what does intent depend on?"), and compare the agent's answers with and without this memory. Unverified that this is the right test.
