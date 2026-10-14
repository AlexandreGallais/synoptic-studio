---
name: retro
description: Run the epic retrospective (RET story) — improvement log, what helped and hurt, evolvability review, playbook update. Use after an epic review, or for a story of type RET.
argument-hint: "[RET id]"
---

# Epic retrospective

Retrospective: $ARGUMENTS. Decision: ADR-0023. Questions: `docs/process/README.md` (evolvability review).

1. **Full check of what the epic produced** (Product Owner, RET-002): `npm run epic:check -- <epic> <first day>` — check:all, branches, pull requests, issues, failed calls and their errors, calls left running, backlog, records; its table goes into the record, each "to look at" line is explained or fixed. Then check the published site and the playground once by hand (`npm run docs:dev`, `npm run dev`).
2. **Gather facts** before opinions: `npm run metrics` (commands by count and time, slowest calls, time and tokens per story and task), `git log` of the epic, the audits' findings tables, the improvement log, CI failures (`gh run list --status failure`), `check:all` duration, files and functions per layer. Read the metrics as an engineer: a command run often or slowly asks for a script, a cache or a check run less often; a story far above the others asks why (its commits); tokens growing per story ask what made the agent search or read more. Read the **Sources** section too: which pages were consulted most, and why; what could be summarized once in the docs so that it is not read again.
   Read **Where the time of the latest turns went** too: the share of the model between calls says whether a tool would save more than a faster check (`docs/tooling/agent-tools.md`). Then write the agent's own account of the epic: what it did, where time and tokens went, what to change to go faster, spend fewer tokens or do better work.
3. **Open with one table** of every item of the improvement log and the feedback log, a recommendation per row; the Product Owner decides row by row. Each row says, in plain words, what would change for them and gives an example: a row they cannot picture cannot be decided (Product Owner, RET-002). Then ask (AskUserQuestion when useful): what helped, what hurt, what they want changed; write their answers in neutral words.
4. **Agent's own review**: recurring mistakes, guards hit, rules ignored, context losses, slow checks; the rows of `docs/process/feedback.md`, grouped into the habits sheet of `docs/playbook/agent.md` (Lessons from the Product Owner), then moved to the retrospective record.
5. **Evolvability review**: answer the six questions with figures; re-check every "considered for later" trigger (e.g. stryker-js issue 6210 still open?) and every override of `package.json` (still needed?).
6. **Decide** with the Product Owner: each change becomes an ADR, a story, a rule, a check or "not now" with its reason.
7. **Write** `docs/process/retrospectives/E0N-retrospective.md` from the template; move treated rows out of the improvement log.
8. **Fresh research** on best practices for projects built by a coding agent (official agent documentation, engineering blogs, method guides; read and recorded before use): what changed since the last retrospective, what to adopt.
9. **Playbook**: update `docs/playbook/` — new lessons (with the check that prevents them), principles or setup steps changed; keep each rule linked to its source.
10. **Starter kit**: keep `docs/playbook/setup.md` copy-ready — files to copy, rhythm (cadences), references worth reading first, use cases met.
11. **Agent configuration**: prune `AGENTS.md`, `CLAUDE.md`, rules, skills and agents; a repeated mistake gets a check, not a sentence. Every four to five epics, and at the end of the project, question **every** rule against how the project went: harden by default, relax only with a reason (`docs/process/agile.md`).
12. Branch, commits per task and pull request as for a story (`/story` steps 3–10).
13. `npm run records:save`: the epic's transcripts and metrics in the private records, `records/` (ADR-0041, ADR-0044).
