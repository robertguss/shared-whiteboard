# Session record: first brainstorm, 15 November 2026

Participants: Robert and an AI agent (Claude Code). This record is public. Details that came from an unpublished planning document for the Mo project are left out.

## What Robert said, in his words

**Opening request**

> i want a tool or a way to visually display ideas when I brainstorm. Something like a mindmap. I am not sure if this can just be an html file that gets updated or if we need to build an app. I do a ton of brainstorming with ai and keeping everything in markdown doesnt really work anymore for me. There are too many files and too much text and its hard for me to see the connections.
>
> I would mucb prefer a visual hierarchy like a mindmap or sometimes a timeline so I can see the history of a project.
>
> A great example is my Mo language project. That is a large project and has even pivoted and been restarted and it would be nice to see all the features and ideas laid out in order and knowing which have been built and which remain and which still need research, etc.
>
> So its part mindmap, roadmap, etc. the key integral part is that it is something both AI agents like claude code and myself can use, edit and modify. The agent needs to be able to read it and undersfand the hierarchy and relationships just like I do. Its a common work surface or shared understanding of the project. The agent and myself are collaborators and this is our shared thinking and working space.

**Asked what he had already tried and why it did not stick**

> I think what I want is something like an infinite whiteboard where notes and things can be moved around freely and organized however makes the most sense. Tinderbox does this well but it feels Very old and has way too many features and is overly complicated and AI has just been bolted on recently.
>
> I have tried Notion but it wasnt visual enough because it was basically just a bunch of pages and databases and it was hard to see the overall picture of the project and how it progressed overtime and what is coming up what has been done etc.
>
> Maintenance and upkeep is not that difficult because the agent does that.

**Asked whether to judge the next decision from a sketch or from text:** he chose a visual sketch, then asked for it as an interactive HTML prototype.

**On Arango**

> https://arango.ai. - would something like this help?

**On this repo**

> After the prototype is built, i want you to create a git repo and a public repo on github for this project we are discussing. I also want to turn this into an llm wiki [Karpathy's gist] Because we are going to be doing lots of research and brainstorming etc

## What the agent found

- The Mo repository (public, `robertguss/mo`) had 826 commits between 11 October and 15 November 2026 and 181 markdown documents outside vendored code, benchmarks and agent worktrees. Its `docs/ROADMAP.md` tracks nine phases, 0 to 8, with phases 0 to 5 finished, v0.1.0 approved on 7 November and v0.2.0 on 11 November.
- Four small tools already aim at a shared human and agent canvas or mind map. See `raw/sources/2026-11-15-tool-scan.md`. All three open-source ones had one to three stars and no pushes since July or October.
- Arango is an enterprise graph database platform with no visual surface.

## What the agent proposed

All of these are proposals. Robert has not confirmed any of them.

1. Do not adopt canvai, Maproom or Vync. Borrow two ideas: the JSON Canvas file format, and the rule that a note the person drags stays where they put it.
2. A free board alone will not show progress over time or done versus coming up. Give each note a few attributes, such as status and date, so a timeline and a status view are other views of the same notes.
3. Do not use Arango. The data is a graph, but a project map of a few hundred notes fits in one file in the repo.
4. One tool covers both brainstorm boards and long-lived project boards.

The agent's first guess, that Notion failed because of upkeep, was wrong. Robert said it failed because it was not visual enough.

## What was built

Prototype 001, `prototypes/001-status-and-history/three-shapes.html`: one page with three switchable shapes using Mo's public roadmap as content.

| Shape | What it is |
| --- | --- |
| A, free board only | One board. Placement is the only structure, colour is the only status, time is laid out by hand. |
| B, board plus views | The same free board. Each note also carries a phase, a status and a date, shown in a timeline view and a status board view. |
| C, auto mind map | A tree laid out by the computer with status dots. No free placement. |

Each shape has a button that simulates an agent adding two notes and changing one status.

In the prototype, phase dates before November are approximations from commit messages, the dashed links between notes are the agent's guesses, and the two notes whose titles start with "Example:" are invented.

## Decided

- Robert: the thing should be visual first. Text and tables did not work.
- Robert: upkeep is the agent's job and is not the hard part.
- Robert: this project gets a public GitHub repo run as an LLM wiki.

## Open at the end of the session

- Which shape: A, B, C or a mix. The agent recommends B.
- Everything listed in `wiki/open-questions.md`.
