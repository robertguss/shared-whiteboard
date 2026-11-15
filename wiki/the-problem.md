---
type: concept
updated: 2026-11-15
sources: [raw/sessions/2026-11-15-why-it-keeps-coming-back.md, raw/sessions/2026-11-15-hand-run-test.md, raw/sessions/2026-11-15-external-brain.md]
---
# The problem, and why it keeps coming back

**Standing:** proposed. Robert asked the question; the answer below is the agent's reading of Robert's own earlier writing, which is private and kept outside this repo. Robert has not confirmed it.

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

## A tension to hold

Robert has also written that friction is part of thinking, not an obstacle to it. A tool in which the agent does all the upkeep could remove the thinking along with the chore. The council put the same point differently: "stored" and "understood" are different states. Unresolved.
