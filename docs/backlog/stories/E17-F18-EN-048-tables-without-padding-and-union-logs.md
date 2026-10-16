---
id: EN-048
epic: E17
feature: F18
title: Tables without padding, append-only logs merged by union
status: ready
points: 2
---

# E17 · F18 · EN-048 — Tables without padding, append-only logs merged by union

Stop Markdown tables from being rewritten whole when one cell grows, and let parallel branches add log rows without conflict (ADR-0065, decisions 1 and 2; F18.AC4).

## Acceptance criteria

- Given Prettier's `proseWrap: "never"`, when a cell wider than the others is added to a table, then only that row changes.
- Given `merge=union` on `journal.md`, `improvements.md` and `feedback.md`, when two branches each add a row, then the merge keeps both without conflict.
- Given a log, when a row appears twice after a union merge, then a test fails and names it.

## Tasks

One task = one commit, referenced as `EN-048.Tn`.

- [ ] T1 — `proseWrap: "never"` and the docs reformatted once, in a commit of its own (1 h)
- [ ] T2 — `.gitattributes` union lines and the uniqueness test, mutation-checked (1 h)
