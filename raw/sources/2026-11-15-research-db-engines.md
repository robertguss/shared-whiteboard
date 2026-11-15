# Research: database engine designs
- Retrieved: 2026-11-15
- Stored as: stub of a research pass. Sources stay at their URLs.

## Summary
- Verdict: no single property of the memory workload needs a new storage engine. Each one has a known mechanism somewhere. What no engine offers is the combination as one coherent model.
- The real strain is semantic, not storage: reads that cause writes, scores that change with the clock, and live views that depend on "now". Engines treat all three as application problems.
- Two cheap, proven mechanisms remove most of the strain: forward decay (decay with zero writes) and blind merge writes or an access log (recall without a read-modify-write).
- Spreading activation as a query operator was built once as a research engine (SGDB, 2010) and is claimed by a 2025-26 "cognitive memory engine" (hirn). SGDB sits on an ordinary key-value store; hirn's README says it is built on Lance columnar files.
- At this scale (up to ~1M thoughts) the whole link graph likely fits in RAM. Storage structure matters for durability and history, not for traversal speed. (Own estimate, unverified.)
- A credible "new" thing is a thin engine layer: event log + decayed strengths + activation operator + incremental views, over SQLite, LMDB/redb or RocksDB. Not a new B-tree or LSM.
- Closest existing engine to the whole picture is an embedded Datalog store with time travel and vector indexes (CozoDB). Its maintenance status is unverified.

## Engine families
| Family | How it stores data | Examples | Fit for the memory workload | Read level |
|---|---|---|---|---|
| B-tree, in place | Sorted pages updated in place, WAL for durability | SQLite, Postgres | Good for thoughts and links. Every recall that bumps strength becomes a write transaction. SQLite allows one writer at a time (from memory, unverified). | snippet |
| B+tree, copy-on-write | New page path per commit, old roots kept for open readers | LMDB, redb | Readers never block; snapshots are cheap. Single writer. Old pages are reclaimed, so not a history store by itself (from memory, unverified). | from memory |
| LSM-tree | Writes go to a memtable, flushed to sorted runs, merged by compaction | RocksDB, LevelDB | Strong for high write rates and blind increments via the merge operator. Overkill tuning surface at this scale. | full (merge wiki) |
| Append-only log + indexes | Immutable facts or rows, every change is a new version | Datomic, XTDB, Cozo time travel | Matches "episodes" and "full history" directly. Costs space if usage counters are also versioned. | full (XTDB, Cozo docs) |
| Columnar | Values stored per column, compressed, scanned in blocks | DuckDB, XTDB v2, Kuzu | Good for timeline and aggregate views and consolidation scans. Poor at many tiny point updates (from memory, unverified). | abstract |
| Native graph, pointer records | Node record points to first relationship; relationships form linked lists; newer "block" format co-locates properties | Neo4j | Fast fixed-cost hops. Server and JVM, not a natural embedded fit. | full (store formats doc) |
| Columnar graph + factorized and WCOJ joins | CSR adjacency in columns, factorized intermediate results | Kuzu | Good embedded fit with HNSW built in (NaviX). Upstream stopped active support in Oct 2025. | abstract |
| RDF and hypergraph stores | Triples or n-ary relations with type systems | Triple stores, TypeDB, HyperGraphDB | n-ary typed links fit "a link about a link". Heavy schemas and servers (from memory, unverified). | from memory |
| Datalog with time travel | Relations keyed with a validity timestamp; recursive rules | CozoDB, Datomic, DataScript, XTDB, TerminusDB | Closest single fit: history, recursion, graph algorithms; Cozo adds HNSW, FTS, MinHash. | full (Cozo docs) |
| Multi-model | Documents, graph edges and vectors in one server | ArangoDB, SurrealDB | Broad but server-first; nothing specific to decay or activation (from memory, unverified). | from memory |
| Vector indexes | Navigable proximity graph (HNSW in RAM, DiskANN on SSD) | HNSW, DiskANN, NaviX | Under 1M vectors an in-memory HNSW is routine. The hard part is filtering by graph or structured predicates. | abstract (NaviX) |
| Incremental view maintenance | Changes flow as deltas through operators; outputs updated, not recomputed | Differential dataflow, Materialize, DBSP/Feldera | Exactly the "every view is a live query" need, including recursion. Separate runtimes, not embedded with history (from memory, unverified). | abstract (DBSP) |
| CRDT and local-first | Operation history merged without coordination | Automerge, cr-sqlite, ElectricSQL | Needed for offline multi-device. On one machine a serialized transaction log is simpler. | abstract (essay), snippet |
| "Cognitive" or activation engines | Adjacency plus weight and type in one record; activation query type | SGDB (2010), hirn | Prior art for the exact idea. Both layer on an existing store (Berkeley DB JE; Lance, per the hirn README). | snippet |

## Where existing engines strain
| Workload property | What engines do today | Gap, if any | Confidence |
|---|---|---|---|
| Writes caused by reads | B-tree engines need a read-modify-write transaction per recall. RocksDB merge records a blind operand folded later. Redis LFU bumps a counter on access. | Real but small. No engine treats "recall" as a logged event with a derived strength. Doable with an append-only access table. | high |
| Continuous decay over time | Stored values are static. Naive designs rewrite every row on a timer. | Real. Batch decay rewrites everything and, in a history-keeping store, multiplies history. Forward decay removes the writes. No engine offers a decayed column type that I found. | medium-high |
| Spreading activation as an operator | Expressible as recursive Datalog or Cypher, or as PageRank-style library algorithms run in batch. | Real. No mainstream engine has a budgeted, top-k, weight-normalized activation operator seeded from vector hits. SGDB proposed one in 2010. | medium |
| Live views updated incrementally | DBSP and differential dataflow handle joins, aggregates and recursion incrementally. | Integration gap, not theory. Also: any view that reads the clock changes every tick, which IVM handles poorly (from memory, unverified). | medium |
| Full history with time travel | Datomic, XTDB (bitemporal), Cozo (Validity keys) do this natively. | Little gap. Risk is volume: if recall bumps are versioned facts, history is dominated by usage noise. | high |
| Concurrent human and agent edits | Single machine: one serialized writer is enough. CRDT stores handle multi-device merge. | Small gap. The interesting case is a long consolidation pass racing live edits; a branch-then-merge model (Dolt, TerminusDB style) fits better than locks (from memory, unverified). | medium |
| Vector + graph in one query | NaviX (inside Kuzu) and Cozo run kNN with graph or relational filters. | Little gap at this scale. | high |

## Ideas that could become engine mechanisms
| Idea | What it would do | Already built? where | Confidence |
|---|---|---|---|
| Forward-decayed strength | Store each reinforcement once as a weight relative to a fixed landmark time; divide by one shared factor at read time. Decay costs zero writes. For exponential decay with one rate, ranking order never changes as time passes, so top-k views stay stable. | Yes, in streaming aggregates (forward decay paper, 2009) and counter libraries. Not as a database column type that I found. | high (math), medium (novelty) |
| Recall as an event, strength as a view | Reads append a cheap access event; strength, recency and co-activation (Hebbian link boosts) are derived and folded lazily. Keeps content history clean. | Pieces: RocksDB merge operator, Redis LFU lazy decay on access. Not as one model. | high |
| Activation operator | First-class `activate(seeds, budget, decay, k)` with adjacency, weight and type co-located so one lookup yields a node's whole fan-out. | SGDB (2010, research, no transactions); hirn claims it as a primitive (unverified depth). | medium |
| Incremental views over activation and decay | Board, timeline, status and focus views as maintained queries over deltas; activation results maintained, not recomputed. | IVM theory exists (DBSP). Incremental activation inside it: not found. | medium-low |
| Consolidation as a branch | Agent consolidation runs on a snapshot, emits a reviewable diff, merges with concurrent human edits. | Branching stores exist (Dolt, TerminusDB, from memory, unverified). Not tied to memory consolidation. | medium |
| Two-tier storage | Versioned, bitemporal content log for thoughts and links; unversioned or coarse-snapshotted usage tier for strengths. | Common practice, not a named design. | high |

## Sources
| Title | URL | Year | Read level |
|---|---|---|---|
| Neo4j operations manual: store formats | https://neo4j.com/docs/operations-manual/current/database-internals/store-formats/ | current docs, n.d. | full |
| Kùzu Graph Database Management System (CIDR) | https://www.cidrdb.org/cidr2023/papers/p48-jin.pdf | 2023 | abstract |
| Cozo docs: time travel | https://docs.cozodb.org/en/latest/timetravel.html | n.d. | full |
| Cozo docs: proximity searches (HNSW, MinHash, FTS) | https://docs.cozodb.org/en/latest/vector.html | n.d. | full |
| DBSP: Automatic IVM for Rich Query Languages (PVLDB 16) | https://www.vldb.org/pvldb/vol16/p1601-budiu.pdf | 2023 | abstract |
| Time in XTDB | https://docs.xtdb.com/about/time-in-xtdb.html | n.d. | full |
| RocksDB wiki: Merge Operator | https://github.com/facebook/rocksdb/wiki/merge-operator | n.d. | full |
| Designing Access Methods: The RUM Conjecture (EDBT) | https://openproceedings.org/2016/conf/edbt/paper-12.pdf | 2016 | abstract |
| NaviX: native vector index for graph DBMSs (PVLDB 18) | https://www.vldb.org/pvldb/vol18/p4438-sehgal.pdf | 2025 | abstract |
| Local-first software (essay) | https://www.inkandswitch.com/essay/local-first/ | 2019 | abstract |
| Forward Decay: a practical time decay model for streaming systems | https://www.cs.ox.ac.uk/people/graham.cormode/pubs/papers/fwddecay.pdf | 2009 | abstract |
| Redis source: evict.c (LFU counter and decay) | https://github.com/redis/redis/blob/3f588a31/src/evict.c | n.d. | snippet |
| SGDB: graph database optimized for activation spreading | https://folk.idi.ntnu.no/noervaag/papers/GDM2010.pdf | 2010 | snippet |
| hirn: cognitive memory database engine (README) | https://github.com/hupe1980/hirn | n.d. | snippet |
| Algorithms behind modern storage systems (ACM Queue) | https://dl.acm.org/doi/10.1145/3212477.3220266 | 2018 | snippet |
| KuzuDB graph database abandoned (The Register) | https://www.theregister.com/software/2025/10/14/kuzudb-graph-database-abandoned-community-mulls-options/1142229 | 2025 | snippet |
| cr-sqlite README (crsql_as_crr) | search result, URL not captured | n.d. | snippet |

## Excerpts
- "optimizing in any two areas negatively impacts the third." (RUM Conjecture, 2016)
- "Since this age is constantly changing, such decay is too complex and unwieldy for scalable implementation." (Forward Decay, 2009)
- "SQL and Datalog can both be implemented in DBSP." (DBSP, 2023)
- "We have developed a generic Merge operation as a new first-class operation in RocksDB to capture the read-modify-write semantics." (RocksDB wiki)
- "we propose a graph query type for the activation spreading operation." (SGDB, 2010)
- "spreading activation, temporal indexing, Hebbian plasticity, and consolidation are database primitives — not application-layer bolts." (hirn README, a claim)

## What I could not verify
- "abstract" for papers means abstract plus introduction and some early sections; I did not read results sections or benchmarks.
- hirn: read only README snippets. Its maturity, correctness and whether activation is truly an engine operator are unverified.
- CozoDB maintenance status and release cadence: not checked.
- Claims marked "from memory, unverified": SQLite and LMDB single-writer rules, LMDB page reclamation, TypeDB/HyperGraphDB, ArangoDB/SurrealDB, Materialize and differential dataflow details, IVM weakness with clock-dependent views, Dolt/TerminusDB branching, Automerge history retention.
- Whether any engine offers decayed columns or an activation operator: absence is from a limited search, not proof.
- The RAM estimate (1M thoughts, ~10M links, a few hundred MB) is my own arithmetic, not measured.
- Search queries named public engines (Neo4j, Kuzu, Cozo, XTDB, RocksDB, Redis, cr-sqlite) because the survey required them; no person or user project names were used.
