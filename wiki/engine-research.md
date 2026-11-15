---
type: comparison
updated: 2026-11-15
sources: [raw/sources/2026-11-15-research-db-engines.md, raw/sources/2026-11-15-research-computational-memory.md, raw/sources/2026-11-15-research-biology-of-memory.md, raw/sources/2026-11-15-research-idea-generation.md, raw/sources/2026-11-15-research-storage-core.md, raw/sessions/2026-11-15-engine-research-kickoff.md, raw/sessions/2026-11-15-north-star-open.md]
---
# Engine research: what is taken and what is open

**Conclusion (proposed):** brain-like memory for an AI agent is crowded. Memory for a person and an agent thinking together is open. What it should be judged by is not settled: Robert does not accept "helps produce new ideas" as the measure. A new engine has a real reason to exist as a layer of operations and meaning. Nothing found calls for a new way of storing bytes on disk.

**How far to trust this:** two research passes on 15 November 2026. Five lanes, about 90 searches, most papers read at abstract level. "Open" means "not found", which is weaker than "does not exist".

## The four lanes in one line each

| Lane | Finding |
| --- | --- |
| [Database engines](../raw/sources/2026-11-15-research-db-engines.md) | No single property of the memory workload needs a new storage structure. What no engine offers is the combination as one model. |
| [Computational memory](../raw/sources/2026-11-15-research-computational-memory.md) | Decay, strength from use, spreading activation, consolidation and episodic-plus-semantic stores have all been built for agents, mostly in 2025 and 2026. All of it is agent-only. |
| [Biology of memory](../raw/sources/2026-11-15-research-biology-of-memory.md) | Three mechanisms look unbuilt as database behaviour. Several famous ones are too contested to lean on. |
| [Idea generation](../raw/sources/2026-11-15-research-idea-generation.md) | Ideas come from joining distant things. AI helpers make ideas converge. Tools die from upkeep cost, not from bad data models. |

## Taken

| Idea | Who has it |
| --- | --- |
| Strength that rises with use and fades | sharpwave, CortiLoop, hirn, arXiv 2605.08538 |
| Recall by spreading activation | SYNAPSE, hirn, HippoRAG (as Personalized PageRank), SGDB in 2010 |
| Event record plus distilled concept graph | Zep/Graphiti, SYNAPSE, Soar since about 2010 |
| "Sleep" consolidation by a model | sharpwave, CortiLoop, arXiv 2605.08538 |
| Memory operations as database primitives with a query language | hirn. Checked by the agent: created May 2026, no stars, built on Lance. A claim in a README, not a proven system. |
| History with time travel | Datomic, XTDB, CozoDB |

## Open: not found anywhere

| Idea | What it would do | From |
| --- | --- | --- |
| Shared focus feeds recall | What the person has open raises the activation of nearby thoughts, so the agent recalls with the person's attention in view. Agent recall lights up the person's views. | Computational lane |
| Salience you can replay | Strength is computed from the event record, so "what mattered last Tuesday" can be shown, or the whole project replayed. | Computational and engine lanes |
| Distance on every link | The store knows how far apart two linked ideas were. "Find a bridge to something distant" and "this cluster is exhausted, leave it" become queries. | Idea lane |
| Linking by time | Thoughts captured within hours of each other lean towards being linked, with no one filing them. | Biology lane, Cai 2016 |
| Tag and capture | A thought starts provisional. It is kept if something important happens near it in time, and lapses otherwise. | Biology lane, Redondo and Morris 2011 |
| Surprise decides what is kept | A thought that fits what is already known is stored lightly. One that surprises is stored strongly. | Biology lane, Takeuchi 2016 |
| Rough before formal | Nearness on the board is data. It can harden into a typed link later, when the person or agent is ready. | Idea lane, Shipman and Marshall |
| Rejections with reasons | What was dropped, why and by whom stays queryable. | Idea lane |
| A guard against sameness | The store measures how alike an agent's contributions are and flags convergence. | Idea lane, 2024 to 2026 studies |
| Near misses | Recall also returns thoughts just below the threshold, as prompts. | Computational lane |

## Too uncertain to build on

| Idea | Why |
| --- | --- |
| A recalled memory becomes editable (reconsolidation) | Human replications fail. Already built by others anyway. |
| Shrinking links during sleep | Mostly one research group, and no different from ordinary decay. |
| Concepts laid out like physical space | Strong theory, thin evidence. |
| Hopfield networks, hyperdimensional vectors as storage | Never became practical stores. |

## Engine mechanics that hold up

| Mechanism | What it gives |
| --- | --- |
| Forward decay | Fading that costs no writes. Known from streaming systems since 2009; not found as a database feature. |
| Recall as an event | A read appends a small event. Strength is derived from events, which keeps history clean and makes replay possible. |
| Activation as an operator | Bounded, top-k, seeded from similarity search. |
| Views kept up to date incrementally | Each view is a live query. The theory exists (DBSP); applying it to activation was not found. |
| Consolidation as a branch | The agent's reorganising pass is one unit the person can review and undo. |

## The honest verdict on "build an engine"

| Layer | Reason to build it | New to the world? |
| --- | --- | --- |
| Operations and meaning: the open ideas above | The project's value is here | Yes, as far as this pass found |
| Engine mechanics: decay, events, activation, live views | Needed to make the operations fast and exact | Partly. Each exists somewhere; the combination does not. |
| Storage core: what it keeps, and how versions relate | It decides whether replay, branching and reviewable consolidation are cheap. Robert's hypothesis, partly confirmed by the second pass below. | The choices exist separately (Dolt, Datomic, RocksDB). Not found combined for memory. |
| Storage core: pages, logs, trees written from scratch | Learning and enjoyment | No |

## Second pass: does the storage core shape memory?

Robert's hypothesis: the design of the storage core may influence the memory and association layer. Source: [research: storage core](../raw/sources/2026-11-15-research-storage-core.md).

**Finding (proposed): partly true.** The storage core shapes memory through what it promises to keep and how versions relate. Where the bytes sit does not, at one person's scale, because everything fits in memory.

| Storage-core choice | Verdict | Why |
| --- | --- | --- |
| Never overwrite; keep versions that share their unchanged parts | Design lever | Branching, comparing and "as of last week" become cheap. Consolidation can be proposed on a branch and reviewed. Changes what the layer above can offer. |
| Store each use as an event and compute strength from the events | Design lever | The rule for fading can be changed later and history recomputed. Several agents can strengthen the same link at once without losing updates. A stored number allows neither. |
| A tested policy for what gets put in front of the agent | Design lever | Cache eviction policies are forty years of worked-out theory about what to keep in reach. Anderson and Schooler (1991) found human forgetting tracks how likely an item is to be needed, which is what those policies estimate. |
| Background merge passes as the home of consolidation | Learning value only | They see one record at a time, run when the engine chooses and cannot be reviewed. Borrow the idea of a background pass, not the mechanism. |
| Laying out related records next to each other | Learning value only | Speed only, and only on data larger than this. |
| Reads that reorganise the structure | Learning value only | They reorganise the index, not the memory. |
| "Stored together means associated" | Metaphor at this scale | The association is the link, not the address. |
| Memristors, neuromorphic chips, DNA storage | Metaphor for now | Laboratory results. Nothing runs on an ordinary laptop. |

**What this changes:** the storage core is not irrelevant, as the first pass implied. Two of the open ideas above, replayable salience and reviewable consolidation, depend directly on a core that keeps events and versions. **What it does not change:** all three levers can be built on an existing embedded store, so the case for writing pages and trees from scratch still rests on learning and enjoyment.

## Biggest doubts

- Each mechanism alone is small, perhaps a hundred lines over an ordinary store. The value may be the model and not an "engine".
- The biology is mostly mice. Applying it to notes is an analogy.
- No one has shown that any of this helps a person think. There is no benchmark for that, which is both the opening and the risk.
