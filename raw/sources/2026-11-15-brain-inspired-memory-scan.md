# Scan: memory systems for AI agents modelled on human memory

- Retrieved: 2026-11-15, one web search
- Stored as: stub. Only search excerpts were read. No paper was read in full and no code was run.

## What turned up

| Work | Where | What it borrows from the brain |
| --- | --- | --- |
| HippoRAG | https://arxiv.org/abs/2405.14831 | Hippocampal indexing theory. An LLM turns documents into a knowledge graph that acts as the index. A question activates its key concepts and Personalized PageRank spreads from them to find related passages. |
| Human-Inspired Memory Architecture for LLM Agents | https://arxiv.org/abs/2605.08538 | Three tiers: short-term cache, episodic store, long-term knowledge graph. Offline "sleep" consolidation merges episodes into the graph. Importance decays, with a reported half-life of about 29 days. |
| SYNAPSE | https://aclanthology.org/2026.findings-acl.1108.pdf | Episodic and semantic memory joined by spreading activation. Title only. |
| CortiLoop, sharpwave, hirn | github.com/shenchengtsi/CortiLoop, github.com/Enlightened-Republic/sharpwave, github.com/hupe1980/hirn | Open-source memory engines. Excerpts list Hebbian link strengthening, spreading activation, forgetting curves, sleep consolidation and reconsolidation. Unverified which excerpt belongs to which repository. |

## Excerpts

> rapid episodic encoding (hippocampus) and slow semantic extraction (neocortex) serve different roles.

> runs the Personalized PageRank (PPR) algorithm on the KG, using the query concepts as the seeds

> When an event is consolidated, the full-fidelity episodic record remains immediately retrievable, while a summarized semantic version is created in the knowledge graph

## What the scan shows

- Modelling an agent's memory on human memory is an active research area with working code.
- Every item found is memory for the agent alone. None describes a surface a person looks at, rearranges and shares with the agent.
- None builds a new storage engine as its contribution. The excerpts mention a vector store and a knowledge graph. Unverified beyond that.
