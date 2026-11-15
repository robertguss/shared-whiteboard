# Log

Append-only. Newest at the bottom. Last five entries: `grep "^## \[" wiki/log.md | tail -5`.

## [2026-11-15] session | First brainstorm

Robert described the need: a visual, shared work surface for him and AI agents. Record: `raw/sessions/2026-11-15-first-brainstorm.md`. Built prototype 001 with three shapes. Shape not yet chosen.

## [2026-11-15] ingest | Tool scan

Read READMEs and homepages for canvai, Maproom, Vync, Nimbalyst and Arango. Wrote `existing-tools.md`. Proposed adopting none.

## [2026-11-15] ingest | Karpathy, LLM Wiki

Set this repo up as an LLM wiki. Wrote `AGENTS.md` as the schema. Departure from the source: third-party material is stored as stubs because the repo is public.

## [2026-11-15] session | Wiki seeded

Created `overview.md`, `requirements.md`, `shape-options.md`, `existing-tools.md`, `mo-test-case.md`, `open-questions.md` and `index.md`.

## [2026-11-15] decision | Shape B, board plus views

Robert chose B after trying prototype 001. Record: `raw/sessions/2026-11-15-shape-decision.md`. Updated `shape-options.md`, `overview.md`, `open-questions.md` and `index.md`. Wiki pages now refer to Robert as "they"; pronouns were never stated.

## [2026-11-15] session | Data first, and a database modelled on the brain

Robert proposed designing the data first, with the board and other layouts as displays of it, and modelling the data on the human brain. Record: `raw/sessions/2026-11-15-data-first-reframe.md`. Nothing new decided.

## [2026-11-15] ingest | Rieseberg, TheBrain, brain-inspired memory scan

Three stubs in `raw/sources/`. Wrote `memory-model.md`. Added TheBrain and a second look at Arango to `existing-tools.md`. Added two questions to `open-questions.md`. Contradiction flagged: the overview said "no graph database"; the data is a graph, so it now says "no Arango or other server database".

## [2026-11-15] decision | Structure plus memory behaviour; engine question opened

Robert chose linked structure plus memory behaviour, confirmed shape B after opening the prototype, and asked for research into database engines and into computer science and biology papers before deciding whether to design an engine. Record: `raw/sessions/2026-11-15-engine-research-kickoff.md`. The agent's earlier "not a new storage engine" is withdrawn as a conclusion and kept as one side of an open question.

## [2026-11-15] ingest | Four research passes on engines, memory, biology and ideas

Four subagents each wrote one stub in `raw/sources/2026-11-15-research-*.md`. Wrote `engine-research.md`. Contradiction found and fixed before commit: two passes disagreed on what hirn is built on; its README says Lance. Most papers were read at abstract level only.

## [2026-11-15] decision | Storage core in scope; north star open

Robert keeps the storage core in the research and does not accept "helps produce new ideas" as the measure of success. The north star is now the first open question. Wiki commits are pushed as they are made from here on. Record: `raw/sessions/2026-11-15-north-star-open.md`.

## [2026-11-15] ingest | Research pass on the storage core

One subagent tested Robert's hypothesis that storage-core design shapes memory. Stub: `raw/sources/2026-11-15-research-storage-core.md`. Added a section to `engine-research.md`. Contradiction flagged: the first pass treated the storage core as irrelevant to the memory layer; the second finds three choices that change what the layer can offer.

## [2026-11-15] decision | An external brain across all projects

Robert widened the scope from one project to all their projects and ideas: a database that acts as an external form of their brain, in which the agent makes or proposes connections. Not to be framed as a second brain. Record: `raw/sessions/2026-11-15-external-brain.md`. Rewrote the top of `overview.md` and the README. Closed the "one brain or one per project" question and added three: what a valuable connection is, what makes this deeper than a second brain, and where the content comes from.

## [2026-11-15] session | Hand-run test with a three-model council

At Robert's suggestion, agents read their agent sessions on this machine and their GitHub repositories, and a council of three models checked the result. Record: `raw/sessions/2026-11-15-hand-run-test.md`. Wrote `hand-run-test.md` with the design lessons only; the findings are private and kept outside the repo. Contradiction flagged: `engine-research.md` puts the project's value in engine operations, while all three council members advise running a simple version daily before engine work.

## [2026-11-15] session | Why the idea keeps coming back

Robert set aside why earlier attempts stopped and asked why the idea recurs. An agent read what Robert wrote at the start of each earlier attempt; the reading is private. Wrote `the-problem.md`. Central observation: the need is cumulative and every attempt started empty. Record: `raw/sessions/2026-11-15-why-it-keeps-coming-back.md`.

## [2026-11-15] decision | Problem statement accepted for now

Robert accepted the statement tentatively, asked what "engines and views are experiments" means, and raised the size of an append-only record. Added two sections to `the-problem.md`. Measured: 2.2 GB of agent session files in 18 days against 0.4 MB of Robert's own words, so the record holds the thinking and points to the session files. Record: `raw/sessions/2026-11-15-record-and-experiments.md`.

## [2026-11-15] session | Agent transcripts as the record

Robert proposed using the agent session files from both machines as the append-only record, copied to one bucket, and starting on the engine. The agent agreed with three changes: the copy never deletes, judgements get their own log, and the first engine stage reads the tools' formats. Found that the tool deletes session files after 30 days by default. Record: `raw/sessions/2026-11-15-transcripts-as-the-record.md`.

## [2026-11-15] decision | SQLite first

Robert decided the first engine is a throwaway on SQLite, run on their real session files, with a custom storage core designed from what it shows. Session retention on this machine set to 100 years. Record: `raw/sessions/2026-11-15-sqlite-first.md`.

## [2026-11-15] ingest | cass and cass-memory

Read at Robert's request before confirming the scope of the first engine. Stub: `raw/sources/2026-11-15-cass.md`. Added a section to `existing-tools.md`. cass covers the lower half of the first engine: collecting, keeping and reading session files. cass-memory solves a different problem, rules for coding agents. Both carry a licence rider against OpenAI and Anthropic; no code is copied here. Neither was installed or run.

## [2026-11-15] decision | Build our own reader

Robert decided the project writes its own reader for agent session files, so that cass is not a dependency, and left the question of a fork to the agent. The agent decided against a fork, because of the licence rider and the size of cass. Record: `raw/sessions/2026-11-15-build-our-own-reader.md`.
