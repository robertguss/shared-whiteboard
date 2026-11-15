# Research: how ideas form, and tools for thinking
- Retrieved: 2026-11-15
- Stored as: stub of a research pass. Sources stay at their URLs.

## Summary
- The best-supported idea in this field is old and plain: new ideas come from joining things that sit far apart in memory. More creative people's word networks are more joined-up and less split into separate clusters. These are group averages from a few labs, not something measured in any one person.
- Memory search behaves like foraging. People pick a cluster clean, then jump to the next one, and those who jump at about the right moment recall more. A store should make "leave this cluster" a first-class move, not leave it to luck.
- Group brainstorming loses mainly because people must wait their turn (production blocking). Seeing other people's ideas without waiting helps. So shared writing works better than a live round of talk, as in brainwriting (writing ideas down in parallel and passing them on).
- 2024 to 2026 studies agree that ideating with an LLM gives each person more, and more polished, ideas. Across people, though, the ideas converge, and AI examples can anchor people on the first suggestion. A store fed by agents needs a diversity and distance signal, or it will fill up with near-duplicates.
- Spatial and informal structure come before formal structure. Users resist being forced to type their links early (Shipman and Marshall). Layout itself carries meaning (Kirsh). A store must accept "these sit near each other" as data and let it harden into typed links later.
- Incubation (stepping away) has a real but modest effect. The light stage at the edge of sleep (N1) helped in one strong study. Brain-network work on creativity is suggestive, not something to design against.
- Earlier tools died from upkeep cost and late payoff, not from bad data models. No tool records where an idea came from, what it answered, what was rejected and why, or how far apart two linked ideas are.

## Findings
| Finding | Evidence | How solid | Read level |
|---|---|---|---|
| Creativity = joining remote associates (Mednick) | Kenett 2014 restates Mednick's idea in network terms. Less creative people have "steep" hierarchies (a few strong links per word); more creative people have "flat" ones (many weak links). | mixed (theory old; tests indirect) | full |
| High-creative semantic networks are less modular, with shorter paths | Group-level networks built from free associations. The low-creative network was more spread out and more modular, and broke into more parts. | mixed (group-level networks; small samples) | full |
| High-creative networks survive link removal better | Percolation test (removing weak links one threshold at a time). High-creative networks broke apart later. | mixed (single lab line) | snippet |
| Forward flow measures distance travelled in thought | Average semantic distance from each word in a free-association chain to all earlier words. Small to medium correlations with creativity. A critique (Rossiter 2020) and a reply exist. | mixed | snippet |
| Memory recall follows optimal foraging | Animal-naming task. People switch clusters when the yield in the current one drops to the overall average (the marginal value theorem). Those closer to this rule recalled more. | strong (replicated idea; one task type) | snippet |
| Production blocking causes most group brainstorming loss | Diehl and Stroebe. Blocking alone halved idea counts. Free riding and fear of judgement explained less. | strong | full (truncated) |
| Exposure to others' ideas can stimulate once blocking is removed | Stroebe, Nijstad and Rietzschel's review traces the shift from "groups lose" to "cognitive stimulation", which is the case for brainwriting and electronic brainstorming. | mixed | full (truncated) |
| LLM help homogenises ideas across people | 36 people, 1,271 ideas. ChatGPT users' ideas were more alike as a group; each person's own spread was the same as without it. They felt less responsible for the ideas. | mixed (small N; replicated in direction) | full |
| LLM ideas raise individual creativity but reduce collective novelty | Story-writing experiment. AI ideas helped less creative writers most, but stories became more similar to each other. | strong-ish (preregistered online experiment) | abstract |
| AI images increase design fixation | N=60. AI image support led to more fixation on the first example, fewer ideas, less variety and lower originality. | mixed (one study) | abstract |
| LLM idea pools are less diverse than human pools, but prompting helps | GPT-4 pools were less diverse than human group pools; chain-of-thought prompting narrowed the gap. | mixed | snippet |
| Incubation helps, modestly and conditionally | Meta-analysis found a positive effect, larger for divergent tasks and after longer preparation. Effect depends on what fills the break. | mixed | abstract |
| The edge of sleep (N1) boosts insight | N=103. At least 15 s of N1 sleep tripled discovery of a hidden rule (83% vs 30%); deeper sleep removed the effect. | mixed (single strong study) | abstract |
| Creative idea production couples the default and executive brain networks | The default network (spontaneous, memory-based thought) proposes ideas; the executive network (goal control) steers and judges them. | weak for design (correlational fMRI) | snippet |
| Analogy = mapping shared relational structure | Structure-mapping theory: aligning relations, not surface features, with a preference for deep connected systems ("systematicity"). SME implements this; MAC/FAC (its retrieval model) reproduces the human tendency to retrieve surface matches. | strong (decades of data) | full (truncated) |
| Conceptual blending | Two input spaces project into a "blend" that has emergent structure. Rich descriptive theory; hard to test. | weak (descriptive) | snippet |
| Space is used to think | Kirsh lists three uses: arrangements that simplify choice, simplify perception, and simplify internal computation. | strong as theory; observational | full |
| Users reject early formalisation | Reasons: cognitive overhead, tacit knowledge, premature structure, situational structure. Remedy: incremental, system-assisted formalisation. | strong (repeated across systems) | full |
| Spatial parsing can recover implicit structure | VIKI's parser found lists, stacks and piles in layouts and offered formal structure at the user's initiative. | mixed (prototype) | snippet |
| Concept maps aid learning | Meta-analysis of 55 studies (5,818 learners). Retention went up; effects ranged from small to large. | strong (for learning, not ideation) | abstract |
| Mind maps | Few controlled studies. Mostly learning outcomes, not idea generation. | weak (from memory, unverified) | none |
| People abandon information systems over 5 factors | Visibility, integration, co-adoption, scalability, return on investment. | mixed (interview study) | snippet |

## Operations of idea work
| Operation | What it is | Supported today by | Gap |
|---|---|---|---|
| Capture | Get a thought down fast, untyped | Every notes app, sticky-note whiteboards | Origin and context are dropped |
| Place / cluster | Put near related things; form piles | Miro, Tinderbox, VIKI/VKB (spatial hypertext) | Nearness is pixels, not data a query can use |
| Link with type | Say how two ideas relate | TheBrain, Tinderbox, concept maps | Typing is forced early or never |
| Formalise later | Harden a pile into a typed group or link | VIKI suggestions (research only) | No mainstream store does this |
| Leave the patch | Jump from an exhausted cluster to a distant one | None (graph views only show) | Missing as a query and as a prompt |
| Bridge distant clusters | Make a link across far-apart regions | Manual links anywhere | No measure of how far the link reaches |
| Analogise | Map a relational structure from one area to another | SME (research), LLM chat | No store keeps the mapping (what corresponds to what) |
| Blend / combine | Fuse two ideas into a third | Manual | The child-to-parents relation is lost |
| Reframe | Restate the problem; change which ideas matter | None | Frames are not objects |
| Diverge with others / agents | Parallel contributions without blocking | Brainwriting, shared boards, LLMs | No guard against convergence |
| Incubate / revisit | Set aside; come back later | Spaced review (Anki); "random note" plug-ins | No resurfacing that knows what is unfinished |
| Evaluate / prune | Judge, reject, archive | Status fields, tags | Rejections and reasons are deleted |
| Promote to plan | Turn ideas into tasks | Kanban, Notion databases | The link back to the idea graph breaks |

Least supported, in my judgement: "leave the patch / bridge distant clusters". The science points to it most directly (foraging, remote associates), and no tool treats it as an operation.

## What a store for ideas should record
| Thing to record | Why | Recorded by any tool today? |
|---|---|---|
| Origin: who or which agent, which prompt or session | Homogenisation and responsibility effects; trust | Partly (edit history), not as data that can be queried |
| What it responded to (reply-to edge) | Building on others is the main group benefit | Chat threads only; lost on a board |
| Semantic distance of each link | Remote links are the creative ones; enables "bridge" queries | No |
| Cluster membership over time | Foraging needs to know the current patch and when it is exhausted | No |
| Spatial position and nearness as soft links | Layout carries meaning (Kirsh; VIKI) | Stored as coordinates only |
| Formality level of each link (implicit → typed) | Lets structure harden later | No |
| Rejected ideas plus reason plus rejecter | Avoid repeats; reopen when conditions change | No (deleted or archived without reasons) |
| Analogy mappings (A:B :: C:D correspondences) | Analogies get re-used and checked | No |
| Parents of a combined or blended idea | Lineage of combined ideas | No |
| Frame or question an idea answers | Reframing changes what is relevant | Rarely (a parent page at most) |
| Last visit and the reason for a revisit | Incubation and revisits | Access time only |
| Diversity of each batch from an agent | Catch convergence early | No |

## Lessons from earlier tools
| Tool or idea | What it got right | Why people left or it stayed niche |
|---|---|---|
| Memex (Bush 1945) | Associative trails as the main object | Never built; trails as shareable objects still rare (from memory) |
| NLS (Engelbart) | Linked, structured, shared outlines; augmenting the person's intellect | Steep learning curve; too far ahead of its hardware (from memory, unverified) |
| Zettelkasten (Luhmann) | Fixed IDs, branching in place, links over a topic tree; aim is surprise | Heavy writing discipline; works for a rare few (from memory) |
| Concept maps (Novak) | Labelled links; measurable learning gains | Slow to build; used in teaching more than in ideation |
| Mind maps (Buzan) | Quick radial capture | A tree with no typed or cross links; weak evidence |
| Spatial hypertext (VIKI, VKB) | Layout as implicit structure; formalise later | Research prototypes; never shipped widely |
| Tinderbox | Spatial plus typed links plus agents (rules that act on notes) | Power-user complexity; niche (from memory) |
| TheBrain | Associative graph with focus navigation | Hierarchy-heavy links; graph lacks meaning at scale (from memory) |
| Roam / Obsidian | Cheap bidirectional links; local files | Gardening cost; graph views look nice but get little use; late payoff (from memory, unverified) |
| General PIM | n/a | Visibility, integration, co-adoption, scalability, return on investment |
| Mnemonic medium (Matuschak and Nielsen) | Builds memory into the medium | Benefit is delayed; "anti-product" next to engagement-driven apps |

## Sources
| Title | URL | Year | Read level |
|---|---|---|---|
| Investigating the structure of semantic networks in low and high creative persons (Kenett et al.) | https://www.frontiersin.org/journals/human-neuroscience/articles/10.3389/fnhum.2014.00407/full | 2014 | full (truncated) |
| Flexibility of thought in high creative individuals represented by percolation analysis (Kenett et al.) | https://www.pnas.org/doi/10.1073/pnas.1717362115 | 2018 | snippet |
| Optimal foraging in semantic memory (Hills, Jones, Todd) | https://doi.org/10.1037/a0027373 | 2012 | snippet |
| "Forward flow": a new measure to quantify free thought (Gray et al.) | https://doi.org/10.1037/amp0000391 | 2019 | snippet |
| Does incubation enhance problem solving? A meta-analytic review (Sio, Ormerod) | https://doi.org/10.1037/a0014212 | 2009 | abstract |
| Sleep onset is a creative sweet spot (Lacaux et al.) | https://www.science.org/doi/10.1126/sciadv.abj5866 | 2021 | abstract |
| Default and executive network coupling supports creative idea production (Beaty et al.) | https://www.nature.com/articles/srep10964 | 2015 | snippet |
| Productivity loss in brainstorming groups (Diehl, Stroebe) | https://doi.org/10.1037//0022-3514.53.3.497 | 1987 | snippet |
| Beyond productivity loss in brainstorming groups (Stroebe, Nijstad, Rietzschel) | https://www.crema-research.ch/wp-content/uploads/2022/01/beyond-productivity-loss-in-brainstorming-groupsthe-evolution-of-a-question.pdf | 2010 | full (truncated) |
| Homogenization effects of LLMs on human creative ideation (Anderson, Shah, Kreminski) | https://arxiv.org/abs/2402.01536 | 2024 | full |
| Generative AI enhances individual creativity but reduces collective diversity (Doshi, Hauser) | https://www.science.org/doi/10.1126/sciadv.adn5290 | 2024 | abstract |
| The effects of generative AI on design fixation and divergent thinking | https://dl.acm.org/doi/10.1145/3613904.3642919 | 2024 | abstract |
| Prompting diverse ideas: increasing AI idea variance (Meincke, Mollick, Terwiesch) | https://arxiv.org/abs/2402.01727 | 2024 | snippet |
| AI and its impact on creativity and diversity: LLM-generated product ideas (Terwiesch et al.) | https://journals.sagepub.com/doi/10.1177/10591478261474243 | 2026 | snippet |
| The Structure-Mapping Engine: a multidecade interaction (Gentner, Forbus) | https://groups.psych.northwestern.edu/gentner/papers/GentnerForbus-2025-SME.pdf | 2025 | full (truncated) |
| The Structure-Mapping Engine: algorithm and examples | https://groups.psych.northwestern.edu/gentner/papers/FalkenhainerForbusGentner89.pdf | 1989 | snippet |
| Conceptual integration networks (Fauconnier, Turner), Cognitive Science 22(2) | URL not captured | 1998 | snippet |
| The intelligent use of space (Kirsh) | https://interactivity.ucsd.edu/articles/Space/AIJ1.html | 1995 | full |
| Formality considered harmful (Shipman, Marshall) | https://people.engr.tamu.edu/shipman/formality-paper/harmful.html | 1999 | full |
| VIKI: spatial hypertext supporting emergent structure (Marshall, Shipman, Coombs) | https://www.semanticscholar.org/paper/VIKI%3A-spatial-hypertext-supporting-emergent-Marshall-Shipman/76dba0d2a151e248c33c54a94b64d7157932df0e | 1994 | snippet |
| Communicating with slip boxes (Luhmann, trans. Kuehn) | https://luhmann.surge.sh/communicating-with-slip-boxes | 1981 | full |
| As we may think (Bush) | https://www.theatlantic.com/magazine/archive/1945/07/as-we-may-think/303881/ | 1945 | snippet (page body not returned) |
| How can we develop transformative tools for thought? (Matuschak, Nielsen) | https://numinous.productions/ttft/ | 2019 | full (truncated) |
| Learning with concept and knowledge maps: a meta-analysis (Nesbit, Adesope) | https://journals.sagepub.com/doi/10.3102/00346543076003413 | 2006 | abstract |
| I give up! Five factors that contribute to the abandonment of information management strategies | https://doi.org/10.1002/meet.2008.14504503115 | 2008 | snippet |

## Excerpts
- "It is impossible to think without writing; at least it is impossible in any sophisticated or networked (anschlußfähig) fashion." (Luhmann)
- "One of the most basic presuppositions of communication is that the partners can mutually surprise each other." (Luhmann)
- "How we manage the spatial arrangement of items around us is not an afterthought: it is an integral part of the way we think" (Kirsh)
- "reasons why users reject formalisms, including issues associated with cognitive overhead, tacit knowledge, premature structure, and situational structure." (Shipman, Marshall)
- "These results point to an increase in individual creativity at the risk of losing collective novelty." (Doshi, Hauser)
- "spending at least 15 s in N1 during a resting period tripled the chance to discover the hidden rule" (Lacaux et al.)

## What I could not verify
- Mednick's Remote Associates Test details and its validity debates: from memory, unverified.
- That Kenett-style networks are built per group rather than per person is from the 2014 method as read; how far later per-person methods confirm it was not checked.
- Which filler activity best helps incubation (low-demand tasks): the abstract was cut off; from memory, unverified.
- Wagner et al. 2004 (sleep doubles insight on a hidden-rule task): from memory, unverified.
- Evidence on mind maps (Buzan) for idea generation: none fetched; from memory, unverified.
- NLS, Tinderbox, TheBrain, Roam and Obsidian rows: builders' own writing was not fetched in this pass; from memory, unverified.
- Bush's text on "trails": the page body was not returned; content is from memory.
- Year of the Stroebe, Nijstad and Rietzschel chapter (2010): from memory, unverified.
- The URL for Fauconnier and Turner 1998 was not captured.
- The claim that no current tool records link distance, rejection reasons or analogy mappings is my judgement from the tools I know, not a survey.
