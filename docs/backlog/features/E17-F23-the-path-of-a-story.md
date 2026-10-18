---
id: F23
epic: E17
title: The path of a story through the tools
status: in-progress
---

# E17 · F23 — The path of a story through the tools

**Benefit hypothesis**: every story, inside a feature or attached to `develop`, goes from its worktree to its merge by the same tools, its log rows and cards inside its own bubble, so the graph the Product Owner reads holds no commit between the bubbles and no story waits for a merge that never comes.

Drafted at E17's retrospective as E02's first feature, "E17's tool incidents fixed" ([RET-004](../stories/E17-RET-004-retrospective-e17.md), P8 and P9), with REV-006's additions; moved into E17 and cut by theme by the Product Owner on 2026-10-18 (SP-024), its other stories gone to F31, F32 and F33, their ids kept. From the cards C1, C3, C4 and C6, the row I50, the cards of `worktree-new` from the main checkout and of an epic's `SP` ([tool incidents](../../process/tool-incidents.md)), and REV-006's finding: with one story at a time (ADR-0086), the reason that kept a story's rows out of its `done` commit (ADR-0065, stories in parallel) is gone. An epic's refinement `SP` stays merged by the Product Owner (REV-006). A tooling feature, validated by the agent (ADR-0072).

## Acceptance criteria

1. A story's log rows and tool cards reach the logs in its `done` commit, inside its bubble: `story:finish` pushes no commit `move the rows of …`, the pre-push lets the `done` commit change the logs, and the cards form a log checked by its form (EN-137).
2. A story attached to `develop`, a `REV` or an epic's `SP`, gets its worktree, its pull request and its rows by the same tools as any story, the `SP` left to the Product Owner's merge; `worktree-new` run in the main checkout finds a story whose file is only on its feature branch on GitHub; a fix without a story id has one branch form, the same for the auto-merge and the guidance (EN-118).

## Stories

In delivery order.

| ID | Type | Title | Status |
| --- | --- | --- | --- |
| [EN-137](../stories/E17-F23-EN-137-the-log-rows-in-the-done-commit.md) | EN | The log rows and the cards in the done commit | done |
| [EN-118](../stories/E17-F23-EN-118-a-story-outside-a-feature.md) | EN | A story outside a feature, by the same tools | done |
| [AUD-025](../stories/E17-F23-AUD-025-audit-f23.md) | AUD | Audit F23 | done |
| [VAL-024](../stories/E17-F23-VAL-024-validate-f23.md) | VAL | Validate F23 | ready |

## Feature plan

| Acceptance criterion | Realized by | Verified by |
| --- | --- | --- |
| 1 | EN-137 | AUD-025, VAL-024 |
| 2 | EN-118 | AUD-025, VAL-024 |
