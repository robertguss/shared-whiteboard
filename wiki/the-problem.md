---
type: concept
updated: 2026-11-15
sources: [raw/sessions/2026-11-15-why-it-keeps-coming-back.md, raw/sessions/2026-11-15-record-and-experiments.md, raw/sessions/2026-11-15-hand-run-test.md, raw/sessions/2026-11-15-external-brain.md]
---
# The problem, and why it keeps coming back

**Standing:** accepted by Robert for now, tentatively ("let's go with that, I guess, for now"). The answer below is the agent's reading of Robert's own earlier writing, which is private and kept outside this repo.

Robert has returned to this idea many times over about two and a half years, in many forms. They read that as evidence the problem is real and unsolved, and want it solved now.

## The need that stayed the same

Across the attempts, the same things are asked for in nearly the same words. The wording below is paraphrased, because most of the earlier writing is private:

| Need | How it was put |
| --- | --- |
| A place where the person and an AI think together | A space for thinking with the AI as a partner, not a tool that takes orders |
| Nothing starts from zero | Ideas get lost; sessions die; the AI should continue work, not begin again |
| One set of data, read two ways | The person sees a picture; the agent reads structure; it is the same data |
| Across projects and across time | One graph, not a board per project; ideas connect across projects and months |
| Built for one person first | Their own need before anyone else's |

## What kept changing

The form: a note vault, a canvas app, a chat-first app, memory for the coding agent, a canvas backed by a graph database, vaults kept by a named agent, one repository per idea, a board beside the coding agent, a mind map, a room for several models. Each was a reasonable answer to the one before.

## Why it keeps coming back (proposed)

| Reason | Evidence | Strength |
| --- | --- | --- |
| The problem grows with the way Robert works. The more agents do, the more the person is the only place where things join up. | Robert said it in two unrelated projects: once about getting several models to work together, where the person had to carry everything between them, and once here, where connections exist only "in my own human brain". | Strong |
| Two needs were bundled and each form served one: memory for the agent, or a picture for the person. | The forms alternate between the two. The aim of one set of data with two ways of reading it is stated again and again and never reached in use. | Strong |
| Every attempt started empty. | No attempt imported anything from the one before. Each used a new store. Only the vocabulary of idea, question and decision survived. | Strong, and the most useful |
| Building replaced using. | Most attempts were active for days. None has a record of being used. | Medium |
| The AI tooling kept changing underneath. | Several attempts cite a new capability as their reason. But the need is older than all of them. | Weak |

## The central observation

The need is cumulative: connections across projects and across months. A thing that is rebuilt every few weeks, empty, on a new store, can never hold months. The earlier attempts did not fail at design. They broke the one property the problem requires, which is continuity.

## What follows (proposed)

1. **The record is the thing that must never restart.** A plain, durable, append-only record of Robert's thinking, with who wrote each item, kept apart from any engine or view.
2. **Engines and views are experiments over the record.** They can be rewritten as often as Robert likes, and the record survives each rewrite. This keeps the ambition and removes the restart.
3. **The first working thing is the record plus a regular pass that proposes connections,** started from what the hand-run test already extracted, and from the data left in the earlier attempts.
4. **Use is measured from the first day.** See the tests in [hand-run test](hand-run-test.md).

## What "experiments over the record" means

There are three layers, and only the first is permanent.

| Layer | What it is | Can it be thrown away? |
| --- | --- | --- |
| Record | What was thought, decided and tried, in plain files. Each item says who wrote it and when. New items are added; old items are not edited. | Never |
| Engine | A database built by reading the record. It holds the links, the strength of each memory and the indexes that make questions fast. | Yes. Read the record again and build a new one. |
| View | A board, a timeline or a status view, drawn from the engine. | Yes |

Git works the same way. The commits are the record. `git log`, a blame view and a website that shows the repository are all built from the commits, and any of them can be replaced without losing a commit.

The earlier attempts kept the data inside the tool: in one database, then another, then a vault. When the tool was dropped, the data went with it. Here the data sits outside every tool. "Experiment" means an engine can be written, tried, and replaced by a better one, and the second engine starts with everything the first one had.

Forgetting belongs to the engine. A memory that fades is shown less often; it is not removed from the record.

## How large the record gets

Robert raised this. Measured on one machine over 18 days, 29 October to 15 November 2026:

| What is kept | Size in 18 days | Per year at that rate |
| --- | --- | --- |
| Everything the agents produced, with tool output | 2.2 GB | about 45 GB |
| Robert's own typed words | 0.4 MB | about 8 MB |

So the concern is right if the record holds everything, and does not apply if it holds the thinking. The record holds the thinking: what was said, decided, tried and connected, each item pointing to the session or file it came from. The full session files stay where they already are.

The real limit is reading, not disk space. No agent can read the whole record at the start of each session. Finding the right few items is the engine's job, and it is the reason the engine is worth designing.

## A tension to hold

Robert has also written that friction is part of thinking, not an obstacle to it. A tool in which the agent does all the upkeep could remove the thinking along with the chore. The council put the same point differently: "stored" and "understood" are different states. Unresolved.
