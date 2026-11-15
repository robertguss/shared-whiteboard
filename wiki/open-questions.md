---
type: question
updated: 2026-11-15
sources: [raw/sessions/2026-11-15-first-brainstorm.md, raw/sessions/2026-11-15-shape-decision.md, raw/sessions/2026-11-15-data-first-reframe.md, raw/sessions/2026-11-15-north-star-open.md, raw/sessions/2026-11-15-external-brain.md]
---
# Open questions

In rough order of how much each one changes what gets built. Before any of them: what the first working version does for Robert on its first day. Robert has accepted the statement of [the problem](the-problem.md) for now. Settled: the shape of the views ([shape options](shape-options.md)), and that it is one brain across all of the person's work, not one per project.

| # | Question | Why it matters | Agent's lean |
| --- | --- | --- | --- |
| 1 | What is the north star in one sentence, and how is it judged? | Without it there is no way to choose between ideas or to tell whether the engine works. Robert has described the thing: an external form of their brain across all projects, where the agent makes or proposes connections. "Helps produce new ideas" was not accepted as the measure. | One memory across everything you work on, in which you and the agent both think, that surfaces the connections neither would see alone. Judged by connections the person did not know and finds real. Proposed. |
| 2 | What is a valuable connection? Same topic, same mechanism, one idea solving another project's problem, a contradiction? | It defines the core operation. Finding "these are about the same thing" is easy and already common. Finding "this works the same way as that" is hard and is where the depth would be. | Ask Robert for a real example and build the first test from it. |
| 3 | What makes this bigger and deeper than a second brain, in substance? | Robert has said what it is not. Graph views of linked notes show connections and change nothing. | The agent proposes connections with reasons and the person judges them; content arrives from the work itself and is not filed by hand; memory behaviour and history are in the engine. Proposed. |
| 4 | Where does the content come from? | Earlier tools were abandoned because upkeep cost more than they gave back. A brain across all projects is empty until it is filled. | From the work itself: repositories, agent session records, notes. The agent ingests; the person does not file. |
| 5 | Is a new database engine worth designing, and what would it do that existing engines do badly? | It is the difference between a data model on a borrowed engine and a multi-year engine project. Robert wants to consider it. See [memory model](memory-model.md). | First pass done: see [engine research](engine-research.md). Build the layer of operations and engine mechanics, where the new ideas are. Robert has put the storage core in scope as well, because its design may shape the memory layer; a research pass on that is the next step. |
| 6 | How does the person edit: dragging and clicking in the browser, asking the agent, or both? | Editing in the browser needs something running that can save. Asking the agent needs only a file and a viewer. This decides "HTML file or app". | Both, with dragging limited to placement at first. |
| 7 | Where is it used: desktop only, or phone and tablet too? | Touch and small screens change the design. | Unknown. |
| 8 | Is the board the source of truth for a project, or a picture of other files such as Mo's `docs/ROADMAP.md`? | Two records drift apart. A picture cannot be edited back. | The board is the source and the roadmap text is generated from it. |
| 9 | When the agent adds notes to a free board, where do they go? | The person's arrangement must survive. | A tray, or next to a related note, never moving what the person placed. |
| 10 | Which attributes does a note carry? | Too few and the views are empty; too many and it becomes Tinderbox. | Status, date, kind, parent, links. Nothing else at first. |
| 11 | Can JSON Canvas carry those attributes, or is a custom file needed? | Decides whether Obsidian can open the boards. | Unverified. Needs a read of the spec. |
| 12 | How does a board for a whole project stay readable at hundreds of notes? | The prototype has about 30. | Unknown. Needs a prototype at Mo's real size. |
| 13 | How does Mo's history get onto the board the first time? | 181 documents and 826 commits. | The agent builds it once from the repo, and the person corrects it. |
| 14 | Is a brainstorm board the same thing as a project board? | One tool or two. | The same thing with a shorter life. |
