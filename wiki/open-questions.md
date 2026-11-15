---
type: question
updated: 2026-11-15
sources: [raw/sessions/2026-11-15-first-brainstorm.md]
---
# Open questions

In rough order of how much each one changes what gets built.

| # | Question | Why it matters | Agent's lean |
| --- | --- | --- | --- |
| 1 | Which shape: A, B, C or a mix? | Everything else hangs on it. | B. See [shape options](shape-options.md). |
| 2 | How does the person edit: dragging and clicking in the browser, asking the agent, or both? | Editing in the browser needs something running that can save. Asking the agent needs only a file and a viewer. This decides "HTML file or app". | Both, with dragging limited to placement at first. |
| 3 | Where is it used: desktop only, or phone and tablet too? | Touch and small screens change the design. | Unknown. |
| 4 | Is the board the source of truth for a project, or a picture of other files such as Mo's `docs/ROADMAP.md`? | Two records drift apart. A picture cannot be edited back. | The board is the source and the roadmap text is generated from it. |
| 5 | When the agent adds notes to a free board, where do they go? | The person's arrangement must survive. | A tray, or next to a related note, never moving what the person placed. |
| 6 | Which attributes does a note carry? | Too few and the views are empty; too many and it becomes Tinderbox. | Status, date, kind, parent, links. Nothing else at first. |
| 7 | Can JSON Canvas carry those attributes, or is a custom file needed? | Decides whether Obsidian can open the boards. | Unverified. Needs a read of the spec. |
| 8 | How does a board for a whole project stay readable at hundreds of notes? | The prototype has about 30. | Unknown. Needs a prototype at Mo's real size. |
| 9 | How does Mo's history get onto the board the first time? | 181 documents and 826 commits. | The agent builds it once from the repo, and the person corrects it. |
| 10 | Is a brainstorm board the same thing as a project board? | One tool or two. | The same thing with a shorter life. |
