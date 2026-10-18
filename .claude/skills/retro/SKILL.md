---
name: retro
description: Run the method's part of a review (REV story at a return of the Product Owner) — tool incident cards, improvement and feedback logs, what helped and hurt, evolvability review, playbook update. Use inside a REV, after its feature review.
argument-hint: "[REV id]"
---

# The method's part of a review

Review: $ARGUMENTS. Decisions: ADR-0023, ADR-0086 (no epic has a retrospective: the method is reviewed at each return of the Product Owner, in the `REV` that also holds the feature review, `/review`). Questions: `docs/process/README.md` (evolvability review). Facts come before opinions, and every change proposed carries its evidence. The span is everything since the last merge into `main`, a day or two: keep each point to what that span shows.

1. **Check what the span produced**: `target/release/epic-check <epic> <day of the last merge into main>` for each epic its features belong to (check:all, branches, pull requests, issues, failed calls and their errors, calls left running, backlog, records); its table goes into the record, each "to look at" line explained or fixed.
2. **Gather the facts**: `target/release/metrics --since <day of the last merge into main>` (commands by count and time, slowest calls, time and tokens per story and task, the sources read most), the span's `git log`, the audits' findings, the improvement log, the duration of `check:all`. Read them as an engineer: a command run often or slowly asks for a script, a cache or fewer runs; a story far above the others asks why; tokens growing per story ask what made the agent read more. Then write the agent's own account of the span: where time and tokens went, what to change to go faster, spend less or do better.
3. **Open with one table** of every tool incident card and every row of the improvement and feedback logs, a recommendation per row, for the Product Owner to decide row by row. Each row says in plain words what would change for them, with an example: a row they cannot picture cannot be decided. Then ask what helped, what hurt, what they want changed, and write their answers in neutral words.
4. **Method and tools** (ADR-0074, ADR-0085): every card is fixed in this review or decided, a tool that fails being fixed at the next review, never during a feature; then what the agent wants to change in its own work — its method, the backlog method where it does not fit an agent, new Rust tools — each with its evidence. Read the **Tools** section of `target/release/metrics` (each Rust tool's calls, failures and times, EN-102): unused, failing or slow.
5. **The agent's own review**: recurring mistakes, guards hit, rules ignored, context lost; the rows of `docs/process/feedback.md` grouped into the habits of `docs/playbook/agent.md`, then moved to the record.
6. **Evolvability review**, when the span gives it figures: the six questions, each "nothing yet", a story or an ADR; every "considered for later" trigger and every override of `package.json` checked again.
7. **Decide** with the Product Owner: each change becomes an ADR, a story, a rule, a check, or "not now" with its reason.
8. **Write** `docs/process/retrospectives/REV-<nnn>-retrospective.md` from the template; move the treated rows out of the logs.
9. **The playbook** (`docs/playbook/`) and **the agent's configuration** (`AGENTS.md`, `CLAUDE.md`, rules, skills, agents): new lessons with the check that prevents each; a repeated mistake gets a check, not a sentence. At the review after an epic's last feature, fresh research on how projects built by a coding agent are run, read and recorded before use; every four to five epics, question **every** rule against how the project went (`docs/process/agile.md`).
10. `target/release/records-save`: the span's transcripts and metrics in the private records, a local folder (ADR-0081).
