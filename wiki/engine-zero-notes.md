---
type: concept
updated: 2026-11-16
sources: [raw/sessions/2026-11-16-engine-zero-built.md]
---
# Engine zero notes

**Where it stands:** the code is built and runs on the first machine's real data. The first judging has not happened, so there is no baseline yet. Source for every number here: `raw/sessions/2026-11-16-engine-zero-built.md`.

## First judging

| Measure | Value |
| --- | --- |
| Connections waiting | 28 |
| From the hand-run test | 23 of 63 |
| Planted false connections | 5 |
| Share marked real and new | waits for Robert |
| Share of planted marked not real | waits for Robert |

40 hand-run connections could not be shown. The rules need two quotes from the person's own session messages in two contexts, and those connections drew on repositories and on one project whose material is not in the sessions.

## First run

| Measure | Value |
| --- | --- |
| Items | 18,056 from 926 sessions |
| The person's items | 1,092, or 6 percent |
| Contexts | 21 projects, 237 sessions on their own |
| Read with nothing new | 0.1 s |
| Rebuild | 21 s |
| Record folder, database | 2.9 GB, 52 MB |

## Slow or awkward operations

| Operation | What was awkward |
| --- | --- |
| Asking | The best 20 matches were all agent text for one question. The person's own words are 6 percent of the items and are outranked. |
| Asking | A question word that starts with `-` needs `--` before it. |
| Reading | A message marked as the person's can hold pasted agent text, such as a brief or a question card. |
| Contexts | Three repository folders outside the configured parents count as their own sessions until corrected by hand. |
| Proposing | 40 of 63 hand-run connections had no session quotes to show. |
| Proposing, asking | Their events reach the bucket only with the daily copy. A mark is uploaded at once. |
| Candidates | The `--from` date is not checked. |
| Rebuild | After a change that adds a column, the database must be rebuilt by hand. |
| All | The data so far covers October and November 2026 from one machine. |

## Limits left open by the code review

None of these has been seen on real data. `unverified` marks what could not be checked.

| Limit | Note |
| --- | --- |
| A machine set up again under the same name must run `pull` before anything else. | Otherwise a new short event file replaces the bucket's longer one. The longer one stays under `old/`, which nothing reads. |
| A mark is accepted from any signed-in user id. | The owner's id is not pinned. The page also needs its key. |
| Agents on the machine run as the same user as the owner. | They could write event files directly. The procedure file forbids it; nothing enforces it. |
| Only item text is masked. | A proposal's statement and a question's words are stored as written. |
| A proposal stays on the page if one of its quotes later stops counting as the person's. | `unverified` whether this happens. |
| A crash in the middle of writing an event joins the next event onto the broken line. | That event is counted as a broken line and skipped. |
| The copy to a real bucket. | `unverified`. Only a local folder has stood in for the bucket. |
| The page handles one request at a time and waits for the upload after a mark. | A stalled upload holds the page. |
