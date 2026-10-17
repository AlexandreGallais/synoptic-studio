---
id: EN-093
epic: E17
feature: F19
title: Every lint and format rule reviewed by the agent for its own work
status: ready
points: 3
---

# E17 · F19 · EN-093 — Every lint and format rule reviewed by the agent for its own work

Once every rule runs in oxlint or the project's Rust linter, go over all of them and over the formatter's settings, and decide, as the one who writes and reads this code, how it wants JavaScript and Markdown written: drop a rule that only slows the work without making the code better, add one that would help, change a setting that makes files slower to read, edit or merge. Asked by the Product Owner on 2026-10-17: the quality of the code must not drop, but the agent is free to choose the ways of writing that suit it — for instance flat Markdown lists that keep Git diffs small rather than ones laid out for the eye, or the line width of 100 when a longer line reads better.

## Acceptance criteria

- Given every rule run by oxlint and the project's linter, when the review ends, then each is listed with what it guards and the agent's verdict — kept, changed, or dropped with its reason — and a dropped rule is never one that guards correctness, the domain's rules (`docs/domain/`) or an ADR's decision without a new ADR.
- Given the formatter's settings (`.oxfmtrc.json`, the table pass), when the review ends, then each is kept or changed with the reason in terms of reading speed, edit size or merge conflicts, measured on a sample of files (lines changed by a typical edit, conflicts on two parallel edits).
- Given a rule or setting added, when it lands, then it has its source or its reason, its tests, and the tree passes it.
- Given the result, when the story ends, then `docs/conventions/` says the rules kept and why, an ADR records the change of style if any, and the lint and format times are written before and after (ADR-0069).

## Tasks

One task = one commit, referenced as `EN-093.Tn`.

- [ ] T1 — The review of every rule and setting, the verdicts (2 h)
- [ ] T2 — The changes applied, the tree fixed, the tests (2 h)
- [ ] T3 — The conventions, the ADR if any, the timings (1 h)
