# Research: the biology of memory
- Retrieved: 2026-11-15
- Stored as: stub of a research pass. Sources stay at their URLs.

## Summary
- Three mechanisms look like real, unbuilt database behaviour: **tag-and-capture** (a write is provisional until a strong event nearby in time makes it stick), **linking by time of encoding** (things written within hours share storage and become linked, and a later salient event can link backwards to them), and **surprise-gated persistence** (novelty and fit with what is already known decide how strongly and how fast something is kept).
- Strength that rises with use and fades with time is not new. Decay, weight normalisation and "sleep downscaling" are close to existing decay and regularisation in computing. Biology adds detail (keep the strongest ~20%, shrink the rest in proportion), not a new idea.
- Biology treats forgetting as an active process with its own machinery, separate from learning. It is a running "race" against consolidation, not passive loss. That is a design stance more than a new algorithm.
- Reconsolidation (a recalled memory becomes editable) is the most tempting idea and the least safe to lean on. Its trigger conditions are vague, and human replications fail.
- "Concept maps use the same machinery as space" (grid cells, Tolman-Eichenbaum Machine) is strong theory with growing evidence. It is not settled biology. The successor representation (store "what tends to come next") is the most usable piece.
- Most cell-level evidence comes from mice and fear conditioning. Translating it to notes and ideas is an analogy, not a finding.

## Mechanisms
Terms: *engram* = the set of cells that holds one memory. *Ensemble* = a group of cells active together. *LTP / LTD* = long-term potentiation / depression, lasting strengthening / weakening of a synapse (connection between cells). *Consolidation* = making a new memory stable.

| Mechanism | What biology shows | How settled | Read level |
|---|---|---|---|
| Engrams | A memory is held by a sparse set of cells. Killing them removes the memory; switching them on artificially recalls it. Seen across many labs and brain areas, almost all in rodents. | established (rodents) | abstract + opening sections (Josselyn & Tonegawa 2020) |
| Allocation by excitability | Cells compete to join an engram. The more excitable (easier to fire) cells win. | supported (many labs, one main group) | abstract |
| Silent engrams | In amnesic mice the engram still exists. Natural cues cannot recall it; direct stimulation can. Existence and retrievability are separate. | supported, debated | abstract |
| Linking by time (Cai 2016) | Two contexts learned 5 hours apart share more cells than ones 7 days apart. Fear of one then transfers to the other. Old mice lose this; raising excitability restores it. | supported (one lab plus follow-ups) | abstract + figure legends |
| Retrospective linking (Zaki 2024) | A strong bad experience reactivates a neutral memory from 2 days earlier during rest, linking them. Linking runs backwards, not forwards, and happens more in quiet waking than sleep. | one lab's result | abstract + excerpts |
| LTP / LTD, STDP | Connections strengthen or weaken with use. In spike-timing-dependent plasticity (STDP), order matters: input-then-output within tens of ms strengthens, the reverse weakens. Window width varies by synapse. | established | abstract + excerpt |
| Synaptic tagging and capture | A weak event leaves only a temporary "tag". It lasts only if proteins made by a strong event, before or after it, are captured. Behaviourally, a weak memory lasts if something novel happens nearby in time. | established in vitro; supported in behaviour | abstract + key points |
| Metaplasticity | Past activity changes how easily a synapse can change later. It is a learning rate with its own memory, there to prevent saturation. | established as a phenomenon; mechanisms open | abstract / snippet |
| Indexing theory | The hippocampus stores a pointer ("index") to the cortical pieces of an event, not the content. *from memory, unverified* (Teyler & Rudy not fetched). | supported, classic | not read |
| Pattern separation / completion | Dentate gyrus makes similar inputs more distinct when stored. CA3 recurrent network rebuilds a whole memory from a partial cue. | supported (convergent lesion, recording, fMRI) | abstract + excerpts |
| Complementary learning systems | A fast system stores single episodes (hippocampus). A slow system extracts structure through interleaved replay (cortex). Replay can be weighted by reward or novelty. | established framework | abstract |
| Systems consolidation, replay, sharp-wave ripples | During rest and sleep, recent sequences are replayed in fast bursts (ripples). Disrupting ripples slows learning of the preceding task. Novel experiences get more ripples. | established (causal in rodents) | abstract |
| Sleep downscaling (SHY) | Waking raises total synaptic strength; sleep scales it back. After sleep, synapse contact area was ~18% smaller, in proportion to size. The largest ~20% were spared. | supported, mostly one group; whether it is *the* function of sleep is debated | abstract |
| Schema-fast consolidation (Tse 2007) | Rats with an existing "schema" (framework of related knowledge) made new one-trial memories independent of the hippocampus within a day, not weeks. | one lab; interpretation disputed (Rudy & Sutherland) | abstract |
| Reconsolidation | Recall can make a memory unstable again, so it can be changed and stored again. It reportedly needs a moderate prediction error. Too little gives plain recall; too much gives new "extinction" learning. | contested: vague boundary conditions, failed human replications | abstract + excerpts |
| Active forgetting (Davis & Zhong) | Flies have "forgetting cells". These dopamine neurons, through Rac1, constantly erode new memories. Blocking them extends memory. Forgetting is separate from learning, not its reverse. | supported (flies, strong); mammals less clear | abstract + excerpts |
| Neurogenesis-driven forgetting | Adding new neurons to the dentate gyrus after learning causes forgetting in adult mice. Reducing it in infants saves memories. | supported (one main lab) | abstract |
| Forgetting as useful (Richards & Frankland) | Argued, not measured: forgetting keeps decisions current and prevents overfitting to single past events. | theory/review | abstract |
| Interference vs decay | Both happen. Interference (new learning disrupts old) is itself listed as an active mechanism. Their relative share is unresolved. *Partly from memory, unverified.* | contested | snippet |
| Place / grid cells, cognitive maps | Place cells fire at locations; grid cells tile space in a lattice. Behrens 2018 argues the same codes organise non-spatial knowledge. | established for space; supported/theory for concepts | abstract |
| Tolman-Eichenbaum Machine | A model that splits structure (entorhinal "basis") from content (sensory). Trained, it reproduces grid and place cells. Its prediction that structure carries across environments was confirmed. | model + one confirming dataset | abstract |
| Successor representation | Place cells may encode expected future locations, not current position. Grid cells may be a compressed basis for this. | theory with supporting fits | abstract |
| Concept cells | Human neurons fire for one person or idea ("Jennifer Aniston neurons"). Related concepts share ~4-5% of cells, unrelated ones <1%. A model says overlap must fall within a band for linking to work. | supported (human recordings; model) | abstract |
| Novelty / dopamine gating | Novelty shortly before or after learning makes a weak memory last 24 h. In mice this runs via locus coeruleus dopamine. "Common" vs "distinct" novelty may route to semantic vs episodic memory. | Takeuchi 2016 supported; two-system split is an opinion piece | abstract / snippet |

## Translation to a data store
| Mechanism | What the store would do | Seen in computing already? | Confidence it is new and useful |
|---|---|---|---|
| Tag and capture | Every write starts *provisional*, with a tag that expires in hours. A later or earlier "strong" write within the window (high salience, user emphasis, decision) promotes nearby provisional writes to durable. Otherwise they lapse. | Partly: TTL caches, staging, generational promotion. None promote on a *neighbouring-in-time unrelated* event. | high |
| Linking by time of encoding | Recently touched nodes carry raised "excitability" for hours. New writes are biased to link to, or share members with, them. The effect fades on an hours scale. | Partly: session co-occurrence in recommenders, temporal graphs. Not as a write-path allocation rule. | medium-high |
| Retrospective linking | A high-salience event triggers an offline job that co-replays notes from the last few days and links backwards only. | Not that I found | medium (biology is one lab) |
| Prediction error stored | Write strength scales with surprise against what the store already predicts. Expected notes are stored weakly or as deltas. | Partly: prioritised replay in RL, delta encoding, novelty detection. | medium-high |
| Schema-gated consolidation | Notes that fit an existing concept cluster get promoted to the concept graph at once. Misfits stay episodic and wait for more evidence. | Rarely as a policy | medium |
| Reconsolidation on read | A read that comes with a mismatch flag opens the record for edit for a short window, then re-saves it. Plain reads do not. | Partly: read-repair, MVCC, copy-on-read. The mismatch-gated window is new. | low (biology contested) |
| Sleep-phase downscaling | A periodic pass scales all weak and medium link weights down in proportion and spares the top fraction. Total strength stays bounded. | Yes, broadly: weight decay, normalisation, forward decay | low novelty |
| Active forgetting | Forgetting is its own background process with its own rate, not decay as a side effect. Writes and consolidation race against it. | Partly: eviction and TTL, which serve capacity, not quality | medium |
| Forgetting improves generalisation | Decay tuned to improve recall quality (less overfitting to one-off notes), measured as such | In ML yes (regularisation). In databases, no. | medium |
| Silent vs active | Keep "exists" separate from "reachable by cue". A record can be kept but hidden from spreading activation, and later revived. | Partly: soft delete, archive tiers | medium |
| Metaplasticity | Each node or link has its own learning rate, which drops after heavy recent change | Adaptive optimisers in ML; not in databases | medium |
| Pattern separation / completion | Push near-duplicates apart at write; rebuild the full note from a partial cue at read | Yes: hashing, deduplication, content-addressable and Hopfield memories | low novelty |
| Concept overlap as link | Association strength = shared members between two clusters, held within a band so concepts stay distinct | Partly: Jaccard links, overlapping communities | medium |
| Map-like coordinates | Store a structural "basis" (position in a relation space) separate from content, plus a successor index of what tends to follow what | Partly: graph embeddings, SR in RL | medium (theory-grade biology) |

## Memory outside neurons
| System | What it shows | Storage idea | Status |
|---|---|---|---|
| Slime mould (*Physarum*) | Food location is written into tube diameters. A softening agent flows through the network, thick tubes grow and thin ones shrink. The new signal is routed by the old hierarchy. | Memory as the capacity of the links themselves. Reads and writes use the same flow. | abstract + excerpts (Kramar & Alim 2021) |
| Immune memory | Clonal selection: re-exposure expands matching cells and refines them through affinity maturation. Memory improves with recall. | Recall that refines the stored item, plus a population of variants | from memory, unverified |
| CRISPR in bacteria | New virus fragments are inserted at one end of an array, in order. It works as a time-ordered append-only log of infections. | Ordered append-only log with recency at the head | from memory, unverified |
| Epigenetic memory | Chromatin marks hold a cell's state through divisions, e.g. plants "remembering" winter. Bistable, per cell. | Bistable flags that survive copying | from memory, unverified |

## Sources
| Title | URL | Year | Read level |
|---|---|---|---|
| A shared neural ensemble links distinct contextual memories encoded close in time (Cai et al.) | https://pubmed.ncbi.nlm.nih.gov/27251287/ | 2016 | abstract (+ figure legends) |
| Offline ensemble co-reactivation links memories across days (Zaki et al.) | https://www.nature.com/articles/s41586-024-08168-4 | 2024 | abstract (+ discussion excerpts) |
| Memory engrams: recalling the past and imagining the future (Josselyn & Tonegawa) | https://pmc.ncbi.nlm.nih.gov/articles/PMC7577560/ | 2020 | abstract (+ opening sections) |
| The Persistence and Transience of Memory (Richards & Frankland) | https://pubmed.ncbi.nlm.nih.gov/28641107/ | 2017 | abstract |
| Making memories last: the synaptic tagging and capture hypothesis (Redondo & Morris) | https://www.nature.com/articles/nrn2963 | 2011 | abstract (+ key points, conclusion excerpt) |
| Metaplasticity: tuning synapses and networks for plasticity (Abraham) | https://www.nature.com/articles/nrn2356 | 2008 | snippet (key points) |
| Metaplasticity: the plasticity of synaptic plasticity (Abraham & Bear) | https://www.cell.com/trends/neurosciences/abstract/S0166-2236(96)80018-X | 1996 | abstract |
| Spike timing-dependent plasticity: from synapse to perception (Dan & Poo) | https://journals.physiology.org/doi/full/10.1152/physrev.00030.2005 | 2006 | abstract (+ excerpt) |
| Pattern separation in the hippocampus (Yassa & Stark) | https://pmc.ncbi.nlm.nih.gov/articles/PMC3183227/ | 2011 | abstract (+ excerpts) |
| What learning systems do intelligent agents need? CLS updated (Kumaran, Hassabis, McClelland) | https://doi.org/10.1016/j.tics.2016.05.004 | 2016 | abstract |
| The hippocampal sharp wave-ripple in memory retrieval and consolidation (Joo & Frank) | https://www.nature.com/articles/s41583-018-0077-1 | 2018 | abstract |
| Disruption of ripple-associated hippocampal activity during rest impairs spatial learning (Ego-Stengel & Wilson) | https://pmc.ncbi.nlm.nih.gov/articles/PMC2801761/ | 2010 | abstract |
| Sharp-wave ripples as a signature of hippocampal-prefrontal reactivation (Tang & Jadhav) | https://pmc.ncbi.nlm.nih.gov/articles/PMC6039287/ | 2018 | snippet |
| Sleep and the price of plasticity (Tononi & Cirelli) | https://pmc.ncbi.nlm.nih.gov/articles/PMC3921176/ | 2014 | abstract (+ excerpts) |
| Ultrastructural evidence for synaptic scaling across the wake/sleep cycle (de Vivo et al.) | https://www.science.org/doi/10.1126/science.aah5982 | 2017 | abstract |
| Structural synaptic plasticity across sleep and wake (Bellesi & de Vivo) | https://pmc.ncbi.nlm.nih.gov/articles/PMC7326289/ | 2020 | snippet |
| Schemas and memory consolidation (Tse et al.) | https://www.science.org/doi/10.1126/science.1135935 | 2007 | abstract |
| Does assimilation into schemas involve systems or cellular consolidation? (Tse et al. reply) | https://pubmed.ncbi.nlm.nih.gov/18055228/ | 2008 | abstract |
| Appraising reconsolidation theory and its empirical validation (Schroyens, Beckers, Luyten) | https://pmc.ncbi.nlm.nih.gov/articles/PMC7614440/ | 2022 | snippet |
| Demarcating the boundary conditions of memory reconsolidation: an unsuccessful replication (Stemerding et al.) | https://www.nature.com/articles/s41598-022-06119-5 | 2022 | abstract (+ excerpts) |
| The challenge of memory destabilisation (Milton, Das, Merlo) | https://doi.org/10.1016/j.brainresbull.2023.01.010 | 2023 | snippet |
| The biology of forgetting: a perspective (Davis & Zhong) | https://pmc.ncbi.nlm.nih.gov/articles/PMC5657245/ | 2017 | abstract (+ excerpts) |
| Hippocampal neurogenesis regulates forgetting during adulthood and infancy (Akers et al.) | https://www.science.org/doi/10.1126/science.1248903 | 2014 | abstract |
| What is a cognitive map? (Behrens et al.) | https://www.cell.com/neuron/fulltext/S0896-6273(18)30856-0 | 2018 | abstract (+ excerpts) |
| The Tolman-Eichenbaum Machine (Whittington et al.) | https://www.cell.com/cell/fulltext/S0092-8674(20)31388-X | 2020 | abstract |
| The hippocampus as a predictive map (Stachenfeld, Botvinick, Gershman) | https://www.nature.com/articles/nn.4650 | 2017 | abstract |
| Concept cells: the building blocks of declarative memory functions (Quiroga) | https://www.nature.com/articles/nrn3251 | 2012 | abstract |
| When shared concept cells support associations (Gastaldi et al.) | https://journals.plos.org/ploscompbiol/article?id=10.1371%2Fjournal.pcbi.1009691 | 2021 | abstract (+ excerpt) |
| Concept neurons flexibly represent abstract relations between concepts (Bausch et al.) | https://www.nature.com/articles/s41467-021-26327-3 | 2021 | abstract |
| Locus coeruleus and dopaminergic consolidation of everyday memory (Takeuchi et al.) | https://www.nature.com/articles/nature19325 | 2016 | abstract |
| Novelty and dopaminergic modulation of memory persistence: a tale of two systems (Duszkiewicz et al.) | https://www.cell.com/trends/neurosciences/fulltext/S0166-2236%2818%2930273-X | 2019 | snippet |
| Encoding memory in tube diameter hierarchy of living flow network (Kramar & Alim) | https://www.pnas.org/doi/10.1073/pnas.2007815118 | 2021 | abstract (+ excerpts) |

## Excerpts
- "the induction of synaptic potentiation creates only the potential for a lasting change in synaptic efficacy, but not the commitment to such a change." (Redondo & Morris 2011)
- "contextual memories encoded close in time are linked by directing storage into overlapping ensembles." (Cai et al. 2016)
- "Fear specifically links retrospectively, but not prospectively, to neutral memories across days." (Zaki et al. 2024)
- "Intrinsic forgetting may be the default state of the brain, constantly promoting memory erasure and competing with processes that promote memory stability" (Davis & Zhong 2017)
- "prevents overfitting to specific past events, thereby promoting generalization." (Richards & Frankland 2017)
- "boundary conditions cannot be translated into specific criteria that can be used to make precise predictions about when memories will become susceptible to change." (Schroyens et al. 2022)

## What I could not verify
- Hippocampal indexing theory (Teyler & Rudy): stated from memory, no source fetched.
- Immune memory, CRISPR arrays and epigenetic memory: all from memory, unverified.
- Specific critiques of the synaptic homeostasis hypothesis, such as evidence for potentiation during sleep, beyond the hedges in Bellesi & de Vivo 2020: not fetched.
- Whether Cai 2016's temporal linking has been independently replicated outside the Silva/Cai lineage: not checked. Zaki 2024 is from the same lab.
- Whether Tse 2007 schema effects were independently replicated: not checked. Only the authors' reply to one critique was seen.
- Human evidence for any cell-level mechanism here, except concept cells: thin. Nearly all of it is in mice, rats or flies.
- The "seen in computing already?" column comes from my own knowledge of computing, not from this pass's sources. A computing-side lane should check it.
