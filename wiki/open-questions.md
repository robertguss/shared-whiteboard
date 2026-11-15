---
type: question
updated: 2026-11-15
sources: [raw/sessions/2026-11-15-first-brainstorm.md, raw/sessions/2026-11-15-shape-decision.md, raw/sessions/2026-11-15-data-first-reframe.md, raw/sessions/2026-11-15-north-star-open.md]
---
# Open questions

In rough order of how much each one changes what gets built. The shape question is settled: see [shape options](shape-options.md).

| # | Question | Why it matters | Agent's lean |
| --- | --- | --- | --- |
| 1 | What is the north star: the one thing this project is for and is judged by? | Without it there is no way to choose between ideas or to tell whether the engine works. Robert does not accept "helps produce new ideas", because ideas arrive anyway when brainstorming with an agent. | Shared understanding: the person and the agent hold the same current picture of a whole project. From Robert's first message; proposed. |
| 2 | Is a new database engine worth designing, and what would it do that existing engines do badly? | It is the difference between a data model on a borrowed engine and a multi-year engine project. Robert wants to consider it. See [memory model](memory-model.md). | First pass done: see [engine research](engine-research.md). Build the layer of operations and engine mechanics, where the new ideas are. Robert has put the storage core in scope as well, because its design may shape the memory layer; a research pass on that is the next step. |
| 3 | Is it one brain across all of the person's work, or one per project? | Recall by association pays off across projects. A whole personal brain cannot live in a public repo. | One private brain, proven on Mo first. |
| 4 | How does the person edit: dragging and clicking in the browser, asking the agent, or both? | Editing in the browser needs something running that can save. Asking the agent needs only a file and a viewer. This decides "HTML file or app". | Both, with dragging limited to placement at first. |
| 5 | Where is it used: desktop only, or phone and tablet too? | Touch and small screens change the design. | Unknown. |
| 6 | Is the board the source of truth for a project, or a picture of other files such as Mo's `docs/ROADMAP.md`? | Two records drift apart. A picture cannot be edited back. | The board is the source and the roadmap text is generated from it. |
| 7 | When the agent adds notes to a free board, where do they go? | The person's arrangement must survive. | A tray, or next to a related note, never moving what the person placed. |
| 8 | Which attributes does a note carry? | Too few and the views are empty; too many and it becomes Tinderbox. | Status, date, kind, parent, links. Nothing else at first. |
| 9 | Can JSON Canvas carry those attributes, or is a custom file needed? | Decides whether Obsidian can open the boards. | Unverified. Needs a read of the spec. |
| 10 | How does a board for a whole project stay readable at hundreds of notes? | The prototype has about 30. | Unknown. Needs a prototype at Mo's real size. |
| 11 | How does Mo's history get onto the board the first time? | 181 documents and 826 commits. | The agent builds it once from the repo, and the person corrects it. |
| 12 | Is a brainstorm board the same thing as a project board? | One tool or two. | The same thing with a shorter life. |
