---
id: F32
epic: E17
title: Reports that say what happened
status: in-progress
---

# E17 · F32 — Reports that say what happened

**Benefit hypothesis**: what the status line, `metrics`, `epic-check`, `session-report`, `val-prepare` and `context-size` print is what happened, so the Product Owner and the agent read a run's state and costs without checking them by hand.

Cut on 2026-10-18 by the Product Owner from E17's tool incidents (SP-024): EN-120 from F23, its id kept, split in two stories, its criteria unchanged but one: a page's visit recorded with its project, a finding of RET-004, is already done, the library's `project_uses` recording each search and page read with its project since EN-106 ([sources](../../tooling/sources.md), Uses). From the card C5 and the rows I36, I72, I82, I83, I88, I91, I92, I110 and I111 (RET-004), and its finding that `epic-check` is blind to the CI. A tooling feature, validated by the agent (ADR-0072).

## Acceptance criteria

1. The status line shows no call of an agent that has ended, and `metrics` counts the command after `|| exit 9`, cuts its findings to the period asked and reads a Sonnet subagent's tokens (EN-120).
2. `epic-check` closes a `dropped` story and reports the last CI of the pull request into `main`; `session-report` rounds to the minute it shows; `val-prepare` writes no line for a struck criterion; `context-size` counts a file's tokens (EN-139).

## Stories

In delivery order.

| ID | Type | Title | Status |
| --- | --- | --- | --- |
| [EN-120](../stories/E17-F32-EN-120-the-status-line-and-metrics.md) | EN | The status line and metrics say what ran | ready |
| [EN-139](../stories/E17-F32-EN-139-the-checks-and-reports-of-a-review.md) | EN | The reports of a review say what happened | ready |
| [AUD-034](../stories/E17-F32-AUD-034-audit-f32.md) | AUD | Audit F32 | ready |
| [VAL-033](../stories/E17-F32-VAL-033-validate-f32.md) | VAL | Validate F32 | ready |

## Feature plan

| Acceptance criterion | Realized by | Verified by |
| --- | --- | --- |
| 1 | EN-120 | AUD-034, VAL-033 |
| 2 | EN-139 | AUD-034, VAL-033 |
