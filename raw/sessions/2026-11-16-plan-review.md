# Session record: the plan is reviewed

Date: 16 November 2026 (UTC). Follows `2026-11-16-scope-confirmed-rust.md`.

## What the agent asked

What to do with the requirements plan: write the implementation plan, build straight away, pressure-test it, or ask more questions.

## What Robert said, in their words

> 3 pressure test and also use the council to review the plan

## What was done

Six reviewers with different roles read the plan. A council of three models from three providers (Claude Opus, GPT Sol, Gemini Carbon) read it separately, with read access to the session files on this machine.

All three council members gave the same verdict: ready after named changes. None found that a settled decision cannot work.

## What they found

- Recall events were stored inside the engine, so a rebuild lost them. Fixed at once: they go in the event log outside the engine.
- A judgement pointed at a connection the engine had numbered. After a rebuild the number means nothing.
- Most Claude Code session files on this machine are subagent sessions, where the "user" is another agent. The plan treated the user role as the person.
- "Project" was not defined, and connections across projects depend on it.
- The copy assumed session files only grow. A rewritten file would have overwritten the bucket copy.
- The check on evidence was weaker than the rule from the hand-run test.
- Privacy stopped at the bucket. Session text also reaches the model step, the page and agents.
- The largest risk named by all three: building replaces using, as in the earlier attempts. The connections from the hand-run test had still not been judged.

## What the agent asked next

How to handle eleven fixes and four decisions. The four decisions, each with the agent's recommendation:

1. First milestone: the copy, the reader, the page, and Robert judging the hand-run connections with false ones mixed in.
2. The reader stores messages only; decisions, attempts and stances wait.
3. The reader masks secrets in what it stores; the bucket stays raw; session text goes only to model services Robert approves.
4. Agents asking the engine stays in this version and is built last.

## What Robert said, in their words

> A

A was "apply the fixes and my recommendation on each decision".

## Decided

- The eleven fixes are applied to the plan.
- The four decisions are settled as recommended.

## Not done

- No second review round was run on the changed plan.
- The council's counts of scratch folders and of duplicate Codex records were not checked by the agent.
