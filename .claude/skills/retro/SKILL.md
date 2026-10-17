---
name: retro
description: Run the epic retrospective (RET story) — improvement log, what helped and hurt, evolvability review, playbook update. Use after an epic review, or for a story of type RET.
argument-hint: "[RET id]"
---

# Epic retrospective

Retrospective: $ARGUMENTS. Decision: ADR-0023. Questions: `docs/process/README.md` (evolvability review). Facts come before opinions, and every change proposed carries its evidence.

1. **Check all the epic produced**: `npm run epic:check -- <epic> <first day>` (check:all, branches, pull requests, issues, failed calls and their errors, calls left running, backlog, records); its table goes into the record, each "to look at" line explained or fixed. Then the published site and the playground once by hand (`npm run docs:dev`, `npm run dev`).
2. **Gather the facts**: `npm run metrics` (commands by count and time, slowest calls, time and tokens per story and task, the sources read most, where the time of the latest turns went), the epic's `git log`, the audits' findings, the improvement log, CI failures (`gh run list --status failure`), the duration of `check:all`, files and functions per layer. Read them as an engineer: a command run often or slowly asks for a script, a cache or fewer runs; a story far above the others asks why; tokens growing per story ask what made the agent read more; a source read often may be summarized once in the docs; the model's share between calls says whether a tool would save more than a faster check (`docs/tooling/agent-tools.md`). Then write the agent's own account of the epic: what it did, where time and tokens went, what to change to go faster, spend less or do better.
3. **Open with one table** of every row of the improvement and feedback logs, a recommendation per row, for the Product Owner to decide row by row. Each row says in plain words what would change for them, with an example: a row they cannot picture cannot be decided. Then ask what helped, what hurt, what they want changed, and write their answers in neutral words.
4. **The agent's own review**: recurring mistakes, guards hit, rules ignored, context lost, slow checks; the rows of `docs/process/feedback.md` grouped into the habits of `docs/playbook/agent.md` (Lessons from the Product Owner), then moved to the record.
5. **Evolvability review**: the six questions answered with figures; every "considered for later" trigger and every override of `package.json` checked again (still needed?).
6. **Method and tools** (ADR-0074): what the agent wants to change in its own work — its method, the backlog method where it does not fit an agent, new methods and Rust tools — each with its evidence (metrics, incidents, log rows, sources read). List in `docs/tooling/agent-tools.md` every tool that would have spared the agent's writing during the epic, with its signal and whether Rust is worth it, and read the agent's tools table of `npm run metrics` for the tools already written.
7. **Decide** with the Product Owner: each change becomes an ADR, a story, a rule, a check, or "not now" with its reason.
8. **Write** `docs/process/retrospectives/E0N-retrospective.md` from the template; move the treated rows out of the improvement log.
9. **Fresh research** on how projects built by a coding agent are run (official agent documentation, engineering blogs, method guides), read and recorded before use: what changed since the last retrospective, what to adopt.
10. **The playbook** (`docs/playbook/`): new lessons with the check that prevents each, principles or setup steps changed, each rule linked to its source; `docs/playbook/setup.md` kept copy-ready (files to copy, cadences, references to read first, use cases met).
11. **Prune the agent's configuration** (`AGENTS.md`, `CLAUDE.md`, rules, skills, agents): a repeated mistake gets a check, not a sentence. Every four to five epics, and at the project's end, question **every** rule against how the project went: harden by default, relax only with a reason (`docs/process/agile.md`).
12. Branch, commits per task and pull request as for a story (`/story` steps 3 to 9).
13. `npm run records:save`: the epic's transcripts and metrics in the private records (ADR-0041, ADR-0044).
14. **Release, when the Product Owner asks** (ADR-0072, steps in `/run`, Validation, step 8): once the retrospective is merged, the pull request `develop` → `main`, title `chore(release): E<nn> — <epic>`, body the epic's features and changes; read its CI and CodeQL, fix a failure on `develop`. The Product Owner merges it with a merge commit; then `develop` is brought up to `main`.
