# Session record: engine zero is built

Date: 16 November 2026 (UTC). Follows `2026-11-16-implementation-plan.md`.

## What the agent asked

What to do with the finished plan, `docs/plans/2026-11-16-0113-feat-engine-zero-plan.md`.

## What Robert said, in their words

> Start ce-work (Recommended)

## What was done

The agent built every code unit of the plan in `engine-zero/`, one unit at a time, each by a fresh worker and each checked by the agent on real data before its commit. Two simplification passes and one code review followed.

| Unit | Built | Checked on real data |
| --- | --- | --- |
| U1 copy and pull | yes | Against a local folder standing in for the bucket. No real bucket exists yet. |
| U2 store and reading loop | yes | yes |
| U3 Claude Code reader | yes | yes |
| U4 Codex reader | yes | yes |
| U6 event log and rebuild | yes | yes |
| U5 contexts | yes | yes |
| U7 proposals and rules | yes | On a copy of the record, so the real event log stayed clean. |
| U8 judging page | yes | The page runs. Reaching it through the sign-in was not checked. |
| U9 first judging | prepared | Waits for Robert. |
| U11 connection pass | yes | The real pass waits for the first judging. |
| U10 asking | yes | On a copy of the record. The check from another project waits. |

U10 was built before the first judging, because the judging waits for Robert and the code did not.

## What was measured on the first machine

| Measure | Value |
| --- | --- |
| Session files read | 2,237 |
| Sessions | 926 |
| Items | 18,056 |
| Items by author | agent 12,778; another agent 3,560; person 1,092; tool 294; unknown 332 |
| Person's items from subagent files | 0 |
| Contexts | 21 projects, 237 sessions counted on their own, 0 unknown |
| Record folder | 2.9 GB |
| Database | 52 MB |
| Read with nothing new | 0.1 s |
| Rebuild | 21 s |
| Item and event counts after a rebuild | equal to before |
| Bucket files lost across two copy runs | 0 |
| Tests | 64 pass; line coverage 92 percent |

Among sessions that T3 Code started, the Codex reader marks 233 messages as the person's against 239 measured for the plan, within the 5 percent the plan allows.

## What the first judging will show

Of the 63 connections found by hand on 15 November, 23 could be quoted from Robert's own session messages in two contexts. The other 40 drew on repositories and on a project whose material is not in the sessions, so the rules cannot keep them. The agent added 5 false connections, labelled as planted in the event log and not on the page. 28 connections wait on the page.

## What the code review found

Seven reviewers read the code. A separate validator confirmed five findings.

| Finding | Outcome |
| --- | --- |
| A missing source folder stopped the copy before the event log was uploaded. | Fixed |
| The pull could replace this machine's newer T3 extract with the bucket's older one. | Fixed |
| A mark or a question failed when another command was writing to the database. | Fixed |
| A running page did not follow a rebuild. | Fixed |
| The reading loop takes its list of readers as a parameter. | Kept. The loop's tests pass a small reader of their own. |
| Bash-mode output could count as the person's words. | Guarded. The data on this machine holds no such record. |
| The first judging has not happened. | Open, waits for Robert |

The limits the review left open are listed in `wiki/engine-zero-notes.md`.

## Decided

Nothing new was decided by Robert in this session.

## Stayed open

- Robert marks the 28 connections.
- Three repository folders count as their own sessions until Robert corrects their context.
- The real bucket, its credentials, the second machine and the daily schedule.
- The question in the plan: what to do with hand-run connections that have no session quotes.
- The pointer line for Robert's global agent instructions.

## Not done

- No model from another provider reviewed the code.
- The page was not opened through the exe.dev sign-in.
- No learning was written to `docs/solutions/` yet.
