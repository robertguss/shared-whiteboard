# Research: the storage core as a memory mechanism
- Retrieved: 2026-11-15
- Stored as: stub of a research pass. Sources stay at their URLs.

## Summary
- Verdict: the hypothesis is **partly true**. The storage core shapes memory, but through its *logical* contract (what it keeps, what a read does, how versions relate), not through its *physical* layout. At one person's scale, where everything fits in RAM, physical layout buys speed only.
- Lever 1: an **immutable, versioned, content-addressed core** (every change is a new version; versions share unchanged parts). It makes "run consolidation on a branch, read the diff, then accept" and "show me the graph as of last week" cheap and natural. This changes what the layer above can express.
- Lever 2: **recall stored as an event, strength computed from those events**. Cache theory and the psychology of forgetting meet here: Anderson and Schooler (1991) found human forgetting tracks how likely an item is to be needed, which is exactly what eviction policies estimate. A core that logs use, not a stored strength number, lets you change the decay law later and recompute.
- Lever 3: **an explicit eviction policy for the agent's working set** (what gets put in front of the agent). ARC-style policies, which balance recency against frequency and remember what they recently evicted, are a tested theory of forgetting. This lever lives above the page cache, at the context-window tier, as MemGPT does.
- Compaction (the background merge pass in log-structured stores) is a **poor home for consolidation**. It sees one key at a time, runs at times the engine chooses, must not change meaning when it combines updates, and cannot be reviewed. Steal the idea of an engine-owned background pass, not the mechanism.
- "Stored together, recalled together" is **mostly metaphor at this scale**. Graph reordering gives real speedups on graphs larger than CPU cache, but its own cost can exceed the gain when the graph is processed only a few times.
- Memristor and DNA associative memories are real lab results, not laptop parts. Intel wound down Optane persistent memory in 2022.

## Correspondences
| Storage idea | Memory parallel | Best evidence | Matters at one-person scale? | Changes expression or only speed? | Read level |
|---|---|---|---|---|---|
| LSM tree (writes go to an in-memory buffer, then to sorted immutable files merged in the background) | Fast episodic capture, slow consolidation | Compactionary: compaction re-organizes layout and drives write, read and space costs | Little: a million small notes fit in RAM | Speed, plus a write-friendly log | partial |
| Compaction filter (callback that keeps, drops or rewrites a key during compaction) | Forgetting, decay | RocksDB wiki: per-key callback; since 6.0 it runs even if it breaks snapshot views | Works, but only for purging | Little: per-key, timing set by the engine | full |
| Merge operator (store small "add this" deltas, fold them later on read or compaction) | Strength that accumulates with use | RocksDB wiki: atomic read-modify-write; engine chooses when to combine, "without changing the underlying semantics" | Yes, for many agents updating one link at once | Some: strength becomes a fold over use events | partial |
| Tiered vs levelled compaction (merge rarely in batches vs keep each level fully sorted) | Coarse vs fine consolidation rhythm | Compactionary: a tuning trade-off between write cost and read cost | No | Speed only | partial |
| Domain compaction doing semantic work (dedup, rollups) | Merging duplicate memories | InfluxDB's compactor deletes and dedups rows by primary key; Doris cleans deleted rows during compaction | No | Only exact-key dedup; no judgement | snippet |
| Graph reordering (renumber nodes so neighbours sit close in memory), e.g. Gorder | Co-activated memories share cells | Gorder: keep "nodes frequently accessed together stored closely"; NP-hard; beats 9 other orders on 8 large graphs | Weak: 1M thoughts x ~10 links is tens of MB; helps full-graph sweeps only | Speed only | partial |
| Reordering cost model | Is consolidation worth it? | Balaji and Lucia: heavy reordering often causes a net slowdown; lightweight, selective reordering up to 1.75x | Says: do not bother unless profiled | Speed only | abstract |
| Database cracking (each query physically sorts the part of the column it touched) | Recall reshapes memory | Idreos et al.: index built as "a byproduct of query processing"; data organized the way users ask for it | No: columns of numbers, not graphs | Speed only; content never changes | partial (intro) |
| Splay tree / move-to-front (accessed item moves to the root or front) | Recently used is easier to recall | Working-set theorem: access cost O(log t), t = distinct items used since last use | No: in RAM, a hash map is faster anyway | Speed only, but a neat formal model of recency | snippet |
| Buffer pool eviction: LRU-K, 2Q, LRFU, ARC | Forgetting curve; recency and frequency | ARC balances recency and frequency online, is scan-resistant, needs no tuning; LRFU spans LRU to LFU with decay | Yes, applied to the agent's context tier, not to RAM pages | Yes, if it decides what an agent can see | abstract |
| Rational analysis of memory | Forgetting = estimated need | Anderson and Schooler: need probability follows frequency, recency, spacing, same as memory | Yes | Yes: grounds a decay law in use statistics | abstract |
| ACT-R base-level approximation | Strength from use history | Petrov: approximate the full power-law sum with the last k uses plus total count | Yes, cheap per item | Yes: needs use timestamps, i.e. LRU-K's history | snippet |
| Anti-caching (memory is primary; cold records evicted to disk, fetched back on touch) | Cold memories still retrievable | DeBrabant et al.: evict least-recently accessed records to disk | No: everything fits in RAM | Speed / capacity only | partial (intro) |
| Prolly tree (content-addressed B-tree, Dolt) | Full history, replay, reviewable consolidation | Dolt docs: same tree for any insert order; diff cost scales with the change, not the data; versions share storage | Yes | Yes: branch, diff, as-of become cheap | partial |
| Semantic hashing / LSH (address computed from meaning) | Similar things stored near each other | Salakhutdinov and Hinton: similar documents get nearby 32-bit addresses; look up by flipping a few bits | Weak: brute-force similarity over 1M vectors is feasible in RAM (from memory, unverified) | Mostly speed; a cluster-prefixed key enables "scan this topic" | abstract |
| Agent memory as storage tiers (MemGPT) | Working memory vs long-term memory | MemGPT: OS-style paging between context window and external storage | Yes | Yes: decides what the agent sees | abstract |
| Memristor CAM (hardware that compares a query to all rows at once) | Associative recall in the substrate | Analog CAM in a 180 nm hybrid chip; small arrays | No: lab only | n/a on a laptop | abstract |
| DNA similarity search | Content-addressed recall | Bee et al.: similarity search over 1.6M images by hybridization | No: wet lab, hours per query (from memory, unverified) | n/a | abstract |
| Persistent memory (Optane) | Durable working memory | Intel began winding down Optane in Q2 2022 | No: not sold | n/a | snippet |

## Verdict
| Storage-core choice | Classification | Why |
|---|---|---|
| Immutable, versioned, content-addressed structure (prolly tree, persistent maps, Merkle DAG) | **Design lever** | Branch, diff and as-of reads become cheap. Consolidation can be proposed on a branch and reviewed as a diff. Changes what the memory layer can offer. |
| Use events as first-class records; strength as a fold (merge operator, Petrov last-k approximation) | **Design lever** | The decay law can be changed and history recomputed. Concurrent agents reinforce without lost updates. A stored scalar cannot do either. |
| Eviction and admission policy for the agent-visible working set (ARC-style, with ghost lists of recently forgotten items) | **Design lever** (above the page cache) | Decides what agents recall unprompted. 40 years of tested policy beats ad hoc decay scores. Physical location of bytes is irrelevant. |
| Engine-owned background maintenance pass (what compaction is) | Learning value only | Good shape: incremental, resumable, off the hot path. But compaction itself is per-key, engine-timed and must preserve meaning. |
| Compaction filter / TTL for forgetting | Learning value only | Fine for hard-deleting archived items. Not for judging what to forget. |
| Graph reordering, clustering, space-filling curves | Learning value only | Real speedups on graphs bigger than cache. Speed only, and often not worth the reorder cost. |
| Cracking, splay trees, move-to-front | Learning value only | Reads change the *index*, not the memory. A formal model of recency cost, nothing more here. |
| Semantic addressing (LSH, quantization, cluster-prefixed keys) | Learning value only | It is an index any store can host. Mild expressive gain: "scan this topic" as a range read. |
| Tiered vs levelled compaction, LSM levels as memory systems | Metaphor | L0 is not a hippocampus. The trade-off is write cost vs read cost. |
| "Stored together = associated" | Metaphor at this scale | In RAM, association is the edge, not the address. |
| Memristor CAM, neuromorphic, DNA, persistent memory | Metaphor for now | Lab demos or discontinued. Nothing runs on an ordinary laptop. |

## Design ideas worth prototyping
| Idea | What it would do | Already built? Where | Confidence |
|---|---|---|---|
| Consolidation on a branch | Run merge-and-relink on a copy; show the diff; human or agent accepts | Branch, diff, merge exist in Dolt (tables) and in git-style object stores | High it works; medium it is pleasant at scale |
| As-of recall | "What was salient last Tuesday" = activation run against an old version | Dolt as-of queries; Datomic-style as-of (from memory, unverified) | High |
| Strength as fold of use events | Store each recall or link use as a small record; derive strength with a Petrov-style last-k plus count | ACT-R (Petrov 2006); RocksDB merge operator for the storage side | High |
| Retroactive decay change | Change the forgetting law, recompute every strength from the event log | Follows from the above; not seen built for agent memory | Medium |
| ARC-style working set for agents | Choose which thoughts are "in mind" using recency and frequency lists that self-tune | ARC in storage systems; MemGPT for tiers, but with simpler policies | Medium |
| Ghost list of recently forgotten thoughts | Keep IDs of just-evicted thoughts; a re-hit promotes them fast and can surface "you dropped this" | ARC's ghost lists (from memory, unverified detail) | Medium |
| Compaction-time purge of archived items | Background pass drops items already marked forgotten and past a grace period | RocksDB compaction filter, TTL | High it works; low value |
| Topic-prefixed keys | Key = (cluster, thought) so one range scan returns a topic | IVF-style partitioned vector indexes (from memory, unverified) | Low-medium |

## What a storage core built for this would look like
*My own synthesis, not from a source.*
- An append-only event log is the source of truth: thoughts created, linked, edited, recalled.
- The current graph is a derived, versioned, content-addressed snapshot. Old versions share unchanged parts.
- Every read that counts as recall also appends a small use event. The core is built for that write load.
- Strength is never stored as the truth. It is computed from use events with a swappable decay law, cached per item as last-k timestamps plus a count.
- Consolidation is a normal transaction on a branch. It writes a proposal; review is a diff; accept is a merge.
- A working-set manager, not the page cache, decides what agents see, using a tested eviction policy with ghost entries.
- The engine runs one background maintenance loop: fold events, refresh caches, purge long-forgotten items, optionally reorder.
- Physical layout is an optimization added only after profiling shows a need.
- All of this can sit on an existing embedded store if that store gives cheap snapshots or the layer builds them. The lever is the contract, not new pages and trees.

## Sources
| Title | URL | Year | Read level |
|---|---|---|---|
| Compaction Filter (RocksDB wiki) | https://github.com/facebook/rocksdb/wiki/Compaction-Filter | current | full |
| Merge Operator (RocksDB wiki) | https://github.com/facebook/rocksdb/wiki/Merge-Operator | current | partial (first ~5k chars) |
| Compactionary: A Dictionary for LSM Compactions (Sarkar et al., SIGMOD demo) | https://subhadeep.net/assets/fulltext/Compactionary-A_Dictionary_for_LSM_Compactions.pdf | 2022 | partial (abstract, intro) |
| InfluxData blog post on the compactor (title taken from URL slug) | https://www.influxdata.com/blog/compactor-hidden-engine-database-performance/ | n/a | snippet |
| Compaction principles (Apache Doris) | https://doris.apache.org/docs/dev/admin-manual/trouble-shooting/compaction-principles/ | current | snippet |
| Speedup Graph Processing by Graph Ordering (Gorder, Wei et al.) | https://raw.githubusercontent.com/datourat/Gorder/master/paper.pdf | 2016 (from memory, unverified) | partial (abstract, intro) |
| When is Graph Reordering an Optimization? (Balaji, Lucia, IISWC) | https://ieeexplore.ieee.org/document/8573478 | 2018 | abstract |
| Database Cracking (Idreos, Kersten, Manegold, CIDR) | https://www.cidrdb.org/cidr2007/papers/cidr07p07.pdf | 2007 | partial (abstract, intro, definition) |
| Self-Adjusting Binary Search Trees (Sleator, Tarjan) | https://www.cs.cmu.edu/~sleator/papers/self-adjusting.pdf | 1985 | snippet (working-set theorem via lecture notes) |
| ARC: A Self-Tuning, Low Overhead Replacement Cache (Megiddo, Modha, FAST) | https://www.usenix.org/conference/fast-03/arc-self-tuning-low-overhead-replacement-cache | 2003 | abstract |
| Anti-Caching (DeBrabant et al., VLDB) | https://www.vldb.org/pvldb/vol6/p1942-debrabant.pdf | 2013 | partial (intro excerpts) |
| Reflections of the Environment in Memory (Anderson, Schooler) | https://doi.org/10.1111/j.1467-9280.1991.tb00174.x | 1991 | abstract |
| Computationally Efficient Approximation of the Base-Level Learning Equation in ACT-R (Petrov) | http://alexpetrov.com/pub/iccm06/PetrovICCM06.pdf | 2006 | snippet |
| Prolly Tree (Dolt docs) | https://www.dolthub.com/docs/architecture/storage-engine/prolly-tree/ | current | partial (first ~8k chars) |
| Semantic Hashing (Salakhutdinov, Hinton) | https://www.cs.utoronto.ca/~rsalakhu/papers/semantic_final.pdf | 2009 | abstract |
| MemGPT: Towards LLMs as Operating Systems | https://arxiv.org/html/2310.08560 | 2023 | abstract |
| Analog content-addressable memories with memristors (Nature Comms) | https://www.nature.com/articles/s41467-020-15254-4 | 2020 | abstract |
| Molecular-level similarity search brings computing to DNA data storage (Nature Comms) | https://www.nature.com/articles/s41467-021-24991-z | 2021 | abstract |
| Intel Winding Down Its Optane Memory Business (Forbes) | https://www.forbes.com/sites/tomcoughlin/2022/07/28/intel-winding-down-its-optane-memory-business/ | 2022 | snippet |

## Excerpts
- "RocksDB always invoke filtering for any key, even if it knows it will make a snapshot not repeatable." (RocksDB Compaction Filter wiki)
- "deciding when/how to combine the operands without changing the underlying semantics." (RocksDB Merge Operator wiki)
- "the environmental relationships are the same as the memory relationships." (Anderson and Schooler, 1991)
- "index maintenance should be a byproduct of query processing, not of updates." (Idreos et al., 2007)
- "No matter which order you insert, update, or delete values, the Prolly tree is the same." (Dolt docs)
- "the reordering step imposes significant overheads leading to a net increase in end-to-end execution time." (Balaji and Lucia, 2018)

## What I could not verify
- Gorder's year and venue (SIGMOD 2016) and its exact speedup figures; I did not read the results section.
- ARC's ghost-list mechanics in detail; the fetch returned a citation page, not the paper body.
- That brute-force similarity over a million embeddings is fast enough on a laptop; this is my estimate, not measured here.
- DNA search latency (hours per query) and Datomic-style as-of semantics: from memory.
- Whether anyone has used compaction hooks for *judgement-based* consolidation (summarizing, merging near-duplicates). I found exact-key dedup in TSDB engines only. Agent-memory projects I found (snippet level) all run decay and consolidation as a layer over SQLite.
- No source measures whether event-sourced strength or branch-reviewed consolidation improves brainstorming. The levers are argued from mechanism, not shown by experiment.
