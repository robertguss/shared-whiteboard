# Research: computational models of memory
- Retrieved: 2026-11-15
- Stored as: stub of a research pass. Sources stay at their URLs.
- Read levels: `full` = whole page or README; `abstract` = abstract or opening sections only; `snippet` = search highlights only.

## Summary
- The memory mechanisms on the wish list are not new. Decay, strengthening with use, spreading activation, episodic-to-semantic consolidation, sleep-like passes, reconsolidation and typed weighted links have all been built for LLM agents. Most of this work is from 2025 and 2026.
- The "make them database primitives" angle is taken too. hirn (Rust, on Lance) says spreading activation, Hebbian plasticity, temporal indexing and consolidation are "database primitives", and it has its own query language (REMEMBER, RECALL, THINK, CONSOLIDATE).
- Classic cognitive architectures did become storage systems, but only on top of SQLite. Soar keeps semantic and episodic memory in SQLite. ACT-R's equations are well specified, but ACT-R is a simulator, not a store.
- Associative-memory models (Hopfield, SDM, BAM, VSA/HDC, NTM/DNC) live inside neural networks or hardware. None became a practical general-purpose database. Modern Hopfield retrieval turned out to be the same maths as transformer attention.
- Everything found is built for the agent alone. The evaluations (LoCoMo, LongMemEval, DMR) score an agent answering questions. Where a human-facing view exists (CortiLoop's D3 graph and dashboard), it is for inspection only. Nothing found uses what the human is looking at as an input to recall.
- What looks open is the human side: the human's focus feeding activation, activation history you can replay over time, and judging memory by whether it helps people think. The memory side does not look open.

## Models
| Model | Core mechanism in one line | Built as a storage system? | Read level |
|---|---|---|---|
| ACT-R declarative memory | A_i = B_i + Σ_j W_j·S_ji + ε; B_i = ln(Σ_j t_j^−d), d≈0.5; S_ji = S − ln(fan_j); P = 1/(1+e^−(A−τ)/s); T = F·e^−A | No. A cognitive simulator (Lisp, Python ports). Equations are now copied into agent memories | full (equations page), snippet (tutorial) |
| Soar semantic memory | Long-term declarative graph of long-term identifiers; cue-based retrieval; activation bias (base-level, spreading: from memory, unverified) | Yes, on SQLite, in memory or on disk; inverted index plus query optimisation | abstract |
| Soar episodic memory | Snapshots of working memory; cue retrieval returns the most recent best-matching episode | Yes, on SQLite with interval indexing (Derbinsky & Laird) | snippet |
| CLARION | Two levels: explicit symbolic rules plus implicit neural nets (from memory, unverified) | No store found | from memory, unverified |
| Collins & Loftus semantic network (1975) | Concepts as nodes; activation spreads along links and fades with distance | No engine. It is the theory most agent memories now cite | from memory, unverified (cited in SYNAPSE) |
| Spreading activation in IR (Crestani 1997) | Pulses through a network of terms and documents, with distance, fan-out and path limits | Research IR prototypes only | snippet |
| Hopfield network (1982) | Energy minimisation recovers a stored binary pattern from a partial cue | No; capacity is far too small (from memory, unverified) | from memory, unverified |
| Modern Hopfield / dense associative memory | Continuous states; exponential capacity; one-step retrieval; update rule = transformer attention | Only as a neural layer (hopfield-layers) | abstract |
| Sparse distributed memory (Kanerva) | Write and read spread over many random hard addresses within Hamming distance | FPGA and co-processor research builds; attention approximates SDM (Bricken & Pehlevan) | snippet |
| Bidirectional associative memory (Kosko) | Two-layer heteroassociative recall of pattern pairs | No (from memory, unverified) | from memory, unverified |
| VSA / HDC (HRR, BSC, MAP) | Bind (⊗) variable to value, bundle (+) into one hypervector, unbind to query, clean up against a codebook | Toy key-value "records" and libraries only; bundle capacity about O(d/log d) | snippet |
| NTM / DNC | Controller reads and writes a memory matrix by content and location; DNC adds usage-based allocation and a temporal link matrix | No. Synthetic tasks only (graph queries, copy). Displaced by attention and RAG (from memory, unverified) | abstract |
| Memory networks (Weston et al.) | Attention over stored sentence embeddings | No; became part of RAG (from memory, unverified) | from memory, unverified |
| HTM / Thousand Brains (Numenta) | Sparse distributed sequence memory; cortical columns learning object models in reference frames | No. Monty (Thousand Brains) leaves HTM out and has no temporal component yet | snippet |

## Agent memory systems
| System | Brain mechanism claimed | What it implements | Storage underneath | Evaluation reported | Read level |
|---|---|---|---|---|---|
| Generative Agents (2023) | Memory stream, reflection | Score = recency (0.995 per game-hour decay since last retrieval) + LLM importance 1–10 + cosine relevance; periodic LLM reflections | In-process natural-language list (from memory, unverified) | Human believability ratings in the Smallville sandbox (from memory, unverified) | abstract + snippet |
| MemGPT / Letta | Operating-system virtual memory, not biology | LLM pages text between context and archival or recall stores using tool calls | Postgres/pgvector in Letta (from memory, unverified) | Document QA and multi-session chat; introduced DMR | abstract |
| HippoRAG 2 | Hippocampal indexing theory; neocortex = LLM | OpenIE triples become a KG with passage nodes; Personalized PageRank from seeds; LLM filters triples ("recognition memory") | Graph plus embeddings in-process (from memory, unverified) | +7 points average over standard RAG on associativity tasks; no loss on factual or sense-making | abstract |
| Zep / Graphiti | Episodic vs semantic | Temporal KG: episodes → entities → facts with bi-temporal validity windows; old facts invalidated, not deleted | Neo4j, FalkorDB or Neptune; Zep's own graph service "Konig" | DMR 94.8% vs MemGPT 93.4%; LongMemEval up to +18.5% accuracy, −90% latency | full (README), abstract |
| A-MEM | Zettelkasten (not brain) | LLM writes note attributes, links to similar notes, "evolves" older notes | Vector store (from memory, unverified) | LoCoMo (from memory, unverified) | abstract |
| Mem0 / Mem0g | None specific | LLM extract, then ADD/UPDATE/DELETE consolidation; graph variant | Vector DB, plus graph DB for Mem0g (from memory, unverified) | LOCOMO vs six baseline types; +26% vs OpenAI memory (from memory, unverified) | abstract |
| SYNAPSE (ACL Findings 2026) | Spreading activation (Collins & Loftus, Anderson), lateral inhibition, temporal decay | Episodic nodes plus LLM-made semantic nodes; temporal and abstraction edges; activation over the graph fused with embeddings ("Triple Hybrid") | Not stated in what I read | LoCoMo +7.2 F1 over SOTA, +23% multi-hop, −95% tokens vs full context | abstract |
| Human-Inspired Memory Architecture (arXiv 2605.08538) | Sleep consolidation (sharp-wave ripple), interference forgetting, engram maturation, reconsolidation | Hot cache → episodic vector store with TTL → semantic KG; batch consolidation every 6 h; six-level fidelity decay down to a "tombstone" | "Unified data layer", vendor not named in what I read | VSCode issues (13K issues, 120K events): 97.2% retention precision, 58% smaller store; LongMemEval M: 70.1% vs 71.2% raw | abstract |
| CortiLoop | Hebbian learning, spreading activation, sleep consolidation, Ebbinghaus decay, reconsolidation, "neuromodulators" | Seven-layer lifecycle; knowledge graph with co-occurrence, temporal and causal edges; background consolidation and pruning worker; FSRS metrics per node; MCP tools; D3 viz | SQLite (optional usearch HNSW, PostgreSQL+pgvector) | Self-reported 92% LongMemEval | full |
| sharpwave | Sharp-wave replay; slow-wave and REM sleep | FSRS-6 stability and retrievability per memory; power-law decay out of recall; background replay promotes episodes to semantic nodes and schemas | SQLite in ~/.sharpwave (MCP server via npx) | A "Benchmark" heading exists; numbers not read | full |
| hirn | Hebbian plasticity, consolidation, spreading activation with lateral inhibition | Property graph + spreading activation + PPR; co-retrieval strengthens edges; decay I·exp(−λh/(1+α·ln(1+n))); HirnQL; multi-agent isolation | Lance 4.0 (IVF-HNSW, BM25 via Tantivy) | LoCoMo, DMR and LongMemEval harnesses; superiority claims marked "in progress" | full |
| Shodh-Memory | Cognitive, edge-native | BM25 + vector + graph spreading activation fused | Tantivy plus local store | Not read | snippet |

## Crowded versus open
| Idea | Crowded or open | Why |
|---|---|---|
| LLM-extracted KG + vector hybrid recall | Crowded | Mem0, Zep, HippoRAG, A-MEM, SYNAPSE, hirn and dozens of repos |
| Decay and strengthen-on-use (Ebbinghaus, FSRS, ACT-R base-level) | Crowded | sharpwave, CortiLoop, hirn, 2605.08538, ACT-R-in-LLM papers |
| Spreading activation as retrieval | Crowded since 2026 | SYNAPSE, hirn, CortiLoop, Shodh, arXiv 2611.13127; PPR in HippoRAG |
| Offline "sleep" consolidation by an LLM | Crowded | sharpwave, CortiLoop, 2605.08538, hirn |
| Episodic log + distilled semantic graph | Crowded | Zep episodes→facts, SYNAPSE, Soar since about 2010 |
| Memory ops as engine primitives / query language | Taken | hirn (HirnQL); Soar smem/epmem commands |
| Bi-temporal fact validity | Taken | Graphiti |
| Full ACT-R equation (fan term, threshold, latency) as query semantics | Thin | Papers I found apply base-level only and say spreading was left out |
| Human focus as an activation source | Open (none found) | All systems take only the agent's query as the cue |
| Replayable activation history (what was salient when) | Open (none found) | Graphiti versions facts over time, not salience |
| VSA/HDC as the storage format | Open but low value | Only libraries and toy records; capacity limits; nothing for agents |
| Benchmarks for human thinking or brainstorming | Open | All benchmarks are agent QA recall |

## Ideas that could become engine mechanisms
| Idea | What it would do | Already built? where | Confidence |
|---|---|---|---|
| Shared focus as source activation (ACT-R W_j) | What the human has open in a view injects activation. Agent recall sees it, and agent recall lights up the human's views | Not found | medium (absence of evidence) |
| Event-sourced activation | Strength is a pure function of the append-only event log. Any past moment's salience can be recomputed and shown | Not found; Soar epmem stores episodes; Graphiti versions facts | medium |
| Below-threshold "near misses" | Return items just under τ as "tip of the tongue" prompts for brainstorming, not just top-k | Not found | low-medium |
| Fan-normalised association (S − ln fan) | Hub thoughts spread less, so popular nodes stop dominating recall | Not seen in agent systems read; hirn uses energy pruning and lateral inhibition instead | low-medium |
| Retrieval changes neighbours (interference, inhibition) | Recall weakens competitors, as in retrieval-induced forgetting | Partly: SYNAPSE (query time), 2605.08538 (forgetting) | medium |
| Reconsolidation window | A recalled thought is editable or mergeable for a while, then settles again | Built: 2605.08538, CortiLoop claims it | high |
| Consolidation as a first-class transaction | An agent's merge/relink pass is one reviewable, revertible unit | Partly: hirn CONSOLIDATE; review or revert not seen | low |

## Sources
| Title | URL | Year | Read level |
|---|---|---|---|
| ACT-R Declarative Memory (reference page) | https://mathematicalpsychology.com/ACT_R_Declarative | n.d. | full |
| ACT-R subsymbolic tutorial (probability, latency) | https://abrsvn.github.io/files/ACT-R_subsymbolic_3.pdf | n.d. | snippet |
| Soar manual: Semantic Memory | https://soar.eecs.umich.edu/soar_manual/06_SemanticMemory/ | current | abstract |
| Soar manual: Episodic Memory | https://soar.eecs.umich.edu/soar_manual/07_EpisodicMemory/ | current | snippet |
| Derbinsky & Laird, Efficiently Implementing Episodic Memory | https://derbinsky.info/_custom/files/research/publications/iccbr2009/paper.pdf | 2009 | snippet |
| Derbinsky et al., Towards Efficiently Supporting Large Symbolic Declarative Memories | https://raw.githubusercontent.com/SoarGroup/website-downloads/main/pubs/iccm2010_smem.pdf | 2010 | snippet |
| Crestani, Spreading Activation Techniques in IR | https://link.springer.com/article/10.1023/A:1006569829653 | 1997 | snippet |
| Ramsauer et al., Hopfield Networks is All You Need | https://arxiv.org/abs/2008.02217 | 2020 | abstract |
| Configurable SDM hardware implementation | https://doi.org/10.1109/iscas.1991.176198 | 1991 | snippet |
| Hyperdimensional computing (Journal of Big Data) | https://link.springer.com/article/10.1186/s40537-024-01010-8 | 2024 | snippet |
| Schlegel et al., A comparison of VSAs | https://www.arxiv.org/pdf/2001.11797v3 | 2020 | snippet |
| Graves et al., Differentiable Neural Computer | https://www.nature.com/articles/nature20101 | 2016 | abstract |
| Thousand Brains Project, Monty docs and FAQ | https://github.com/thousandbrainsproject/tbp.monty | 2024–26 | snippet |
| Park et al., Generative Agents | https://arxiv.org/abs/2304.03442 | 2023 | abstract |
| Packer et al., MemGPT | https://arxiv.org/abs/2310.08560 | 2023 | abstract |
| Gutiérrez et al., HippoRAG 2 (From RAG to Memory) | https://arxiv.org/abs/2502.14802 | 2025 | abstract |
| Rasmussen et al., Zep temporal KG | https://arxiv.org/abs/2501.13956 | 2025 | abstract |
| Graphiti README | https://github.com/getzep/graphiti | 2026 | full |
| Xu et al., A-MEM | https://arxiv.org/abs/2502.12110 | 2025 | abstract |
| Chhikara et al., Mem0 | https://arxiv.org/abs/2504.19413 | 2025 | abstract |
| Jiang et al., SYNAPSE | https://arxiv.org/abs/2601.02744 (ACL: https://aclanthology.org/2026.findings-acl.1108/) | 2026 | abstract |
| Human-Inspired Memory Architecture for LLM Agents | https://arxiv.org/abs/2605.08538 | 2026 | abstract |
| CortiLoop | https://github.com/shenchengtsi/CortiLoop | 2026 | full |
| sharpwave | https://github.com/EnlightenedRepublic/sharpwave | 2026 | full |
| hirn | https://github.com/hupe1980/hirn | 2026 | full |
| Shodh-Memory paper | https://www.shodh-memory.com/shodh_memory.pdf | 2026 | snippet |
| Activation-Guided Graph Retrieval for Cognitive Memory Reconstruction | https://arxiv.org/abs/2611.13127 | 2026 | snippet (title only) |
| Pyeon & Tehranchi, LLM agent with modified ACT-R base-level | https://dl.acm.org/doi/full/10.1145/3765766.3765803 | 2026 | snippet |
| Memory in the Age of AI Agents (survey) | https://arxiv.org/html/2512.13564v1 | 2025 | abstract |

## Excerpts
- "graph-native cognitive memory where spreading activation, temporal indexing, Hebbian plasticity, and consolidation are database primitives" (hirn README)
- "relevance emerges from spreading activation rather than pre-computed links" (SYNAPSE abstract)
- "Semantic memory uses SQLite to facilitate efficient and standardized storage and querying of knowledge." (Soar manual, ch. 6)
- "we treat recency as an exponential decay function over the number of sandbox game hours since the memory was last retrieved" (Generative Agents)
- "The new update rule is equivalent to the attention mechanism used in transformers." (Ramsauer et al.)
- "Token-level memory is also the most common memory form and the one with the largest body of existing work." (Memory in the Age of AI Agents)

## What I could not verify
- Storage backends for HippoRAG 2, A-MEM, Mem0, Generative Agents, Letta and SYNAPSE. These are from memory or not stated in what I read.
- Mem0's headline numbers, A-MEM's benchmark, and Generative Agents' evaluation are from memory.
- Whether Soar smem's base-level and spreading activation match ACT-R exactly. The manual sections I read did not reach the activation part.
- CLARION, BAM, the original Hopfield network, Collins & Loftus, NTM and memory networks are described from memory. I fetched no primary source for them.
- "Abstract" for SYNAPSE and 2605.08538 means the abstract plus roughly the first 12,000 characters. I did not read their results tables in full.
- All benchmark numbers from repos (CortiLoop's 92% LongMemEval) are self-reported and not reproduced.
- "Not found" claims rest on about 18 searches. GitHub has many small memory repos, so one of the open ideas may already exist somewhere. The search for human–agent shared memory returned only product landing pages (Obsidian-editable Markdown memory). I did not assess them.
- I did 5 full reads against a target of 8–12. The rest are abstract or snippet level.
