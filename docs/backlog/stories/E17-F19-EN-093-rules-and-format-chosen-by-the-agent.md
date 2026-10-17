---
id: EN-093
epic: E17
feature: F19
title: Every rule and tool reviewed by the agent for its own work
status: done
points: 3
---

# E17 · F19 · EN-093 — Every rule and tool reviewed by the agent for its own work

Once every rule runs in oxlint or the project's Rust linter, go over all of them and over the formatter's settings, and decide, as the one who writes and reads this code, how it wants JavaScript and Markdown written: drop a rule that only slows the work without making the code better, add one that would help, change a setting that makes files slower to read, edit or merge. Asked by the Product Owner on 2026-10-17: the quality of the code must not drop, but the agent is free to choose the ways of writing that suit it — for instance flat Markdown lists that keep Git diffs small rather than ones laid out for the eye, or the line width of 100 when a longer line reads better.

Widened by the Product Owner on 2026-10-17 to the whole tooling: every check, hook and convention the agent works under — commitlint's rules, the branch-name check, the pre-commit and pre-push steps, the done-commit footers, the story and commit formats, the scripts of `package.json` — is asked the same questions: does it serve the work, and why; would removing it lower the quality, or was it a toy that only costs time; what is missing that would make the agent think less about how to do a thing, because it is always done the same way (as commitlint makes every commit message alike). The tools F21 rewrites in Rust are reviewed the same way before they are rewritten, so that only what serves is carried.

Left by EN-094 for this story: the docs that still cite the ADRs ADR-0072 superseded (`docs/tooling/git-workflow.md`, `ci.md`, `agent.md`, `releases.md`, `commands.md`, `agent-tools.md`, `versions-and-security.md`, `docs/backlog/README.md`) are moved to ADR-0072, and `docs/process/agile.md`'s "Who does what" table follows ADR-0072 on who merges a feature.

## Acceptance criteria

- Given every rule run by oxlint and the project's linter, when the review ends, then each is listed with what it guards and the agent's verdict — kept, changed, or dropped with its reason — and a dropped rule is never one that guards correctness, the domain's rules (`docs/domain/`) or an ADR's decision without a new ADR.
- Given the formatter's settings (`.oxfmtrc.json`, the table pass), when the review ends, then each is kept or changed with the reason in terms of reading speed, edit size or merge conflicts, measured on a sample of files (lines changed by a typical edit, conflicts on two parallel edits).
- Given a rule or setting added, when it lands, then it has its source or its reason, its tests, and the tree passes it.
- Given every check, hook and convention of the tooling, when the review ends, then each has the same verdict — kept with what it brings, changed, dropped as costing time without adding quality, or added because it makes a frequent task uniform — and F21's rewrite stories carry only what was kept.
- Given the result, when the story ends, then `docs/conventions/` says the rules kept and why, an ADR records the change of style if any, and the lint and format times are written before and after (ADR-0069).

## Tasks

One task = one commit, referenced as `EN-093.Tn`.

- [x] T1 — The review of every rule and setting, the verdicts (2 h)
- [x] T2 — The changes applied, the tree fixed, the tests (2 h)
- [x] T3 — The conventions, the ADR if any, the timings (1 h)
- [x] T4 — The rule catalogue by context, with profiles per kind of project, on paper only (2 h), added by the Product Owner on 2026-10-17

## For the main thread

- **Rule-docs**: no rule of `docs/domain/` changed; the done commit's footer: `Rule-docs: none affected, a review of the tooling's rules and conventions, no business rule changed`.
- **No ADR**: no change of style: the format settings stay, the rules added guard correctness, and the changed conventions (tags on test helpers, the per-task check, `lint:fix`) are written in `docs/conventions/`; ADR-0011 and ADR-0012 kept, measured in `docs/conventions/rules.md`.
- **No settings proposal** for `.claude/settings.json`.
- **`eslint/settings/dropped-rules.ts` header wording**: moot, the file left with EN-085.
- **For F21's feature file** (its draft table of stories, as a note): carry only what EN-093 kept, measured in `docs/conventions/rules.md` (Checks, hooks and conventions). Story 5 (Git hooks): the staged-file runner without ESLint's step; commitlint's rules as they are, with the 14 scopes (`eslint` dropped); **add** a check that a commit on a story branch carries `Refs: <ID>.Tn` or `Closes: <ID>` with the branch's own id; the branch-name pattern as one regular expression. Story 3 (repository tools): `check-all` without a `check` twin to keep in step (the story skill no longer runs `npm run check` per task); `replace-text` kept, it is the agent's most used edit tool. Story 2 (metrics): kept whole; the friction count by rule that EN-093 made by hand from the transcripts would be a cheap addition to `metrics-report` (findings by rule, per epic), so that the next review of the rules reads it instead of grepping.
