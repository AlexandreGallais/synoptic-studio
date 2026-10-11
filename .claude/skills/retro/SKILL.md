---
name: retro
description: Run the epic retrospective (RET story) — improvement log, what helped and hurt, evolvability review, playbook update. Use after an epic review, or for a story of type RET.
argument-hint: "[RET id]"
---

# Epic retrospective

Retrospective: $ARGUMENTS. Decision: ADR-0023. Questions: `docs/process/README.md` (evolvability review).

1. **Gather facts** before opinions: `git log` of the epic, the audits' findings tables, the improvement log, CI failures (`gh run list --status failure`), time per story if known, `check:all` duration, files and functions per layer (`find src -name '*.ts' -not -name '*.test.ts' | wc -l` per folder).
2. **Ask the Product Owner** (AskUserQuestion when useful): what helped, what hurt, what they want changed in how the agent works; quote their words.
3. **Agent's own review**: recurring mistakes, guards hit, rules ignored, context losses, slow checks; the rows of `docs/process/feedback.md`, grouped into the sheet "when I do this, do that instead" of `docs/playbook/agent.md` (Lessons from the Product Owner), then moved to the retrospective record.
4. **Evolvability review**: answer the six questions with figures; re-check every "considered for later" trigger (e.g. stryker-js issue 6210 still open?).
5. **Decide** with the Product Owner: each change becomes an ADR, a story, a rule, a check or "not now" with its reason.
6. **Write** `docs/process/retrospectives/E0N-retrospective.md` from the template; move treated rows out of the improvement log.
7. **Fresh research** on best practices for projects built by a coding agent (official agent documentation, engineering blogs, method guides; read and recorded before use): what changed since the last retrospective, what to adopt.
8. **Playbook**: update `docs/playbook/` — new lessons (with the check that prevents them), principles or setup steps changed; keep each rule linked to its source.
9. **Starter kit**: keep `docs/playbook/setup.md` copy-ready — files to copy, rhythm (cadences), references worth reading first, use cases met.
10. **Agent configuration**: prune `CLAUDE.md`, rules and skills; a repeated mistake gets a check, not a sentence.
11. Branch, commits per task and pull request as for a story (`/story` steps 3–10).
