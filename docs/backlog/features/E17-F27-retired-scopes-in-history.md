---
id: F27
epic: E17
title: The history check reads the scopes a commit was written with
status: done
---

# E17 · F27 — The history check reads the scopes a commit was written with

**Benefit hypothesis**: the `Commit messages and branch name` check of #370 passes: the six commits of F19 written with the scope `eslint`, valid then and removed from the list by EN-085, no longer refuse a range that holds them, while a new commit with `eslint` is still refused by the `commit-msg` hook.

Found at #370's run 39143909219 (2026-10-18): every other check green, this one refused six F19 commits (`chore(eslint): …`, `feat(eslint): …`) for `scope-enum`. `develop`'s history is not rewritten (no force push). A tooling fix, validated by the agent (ADR-0072), before 0.3.0.

## Acceptance criteria

1. The history check accepts a commit whose only error is a scope retired from the list (`eslint`, EN-085); the `commit-msg` hook still refuses it (EN-128).

## Stories

In delivery order.

| ID | Type | Title | Status |
| --- | --- | --- | --- |
| [EN-128](../stories/E17-F27-EN-128-retired-scopes-in-history.md) | EN | The history check reads the scopes a commit was written with | done |
| [AUD-029](../stories/E17-F27-AUD-029-audit-f27.md) | AUD | Audit F27 | done |
| [VAL-028](../stories/E17-F27-VAL-028-validate-f27.md) | VAL | Validate F27 | done |

## Feature plan

| Acceptance criterion | Realized by | Verified by |
| --- | --- | --- |
| 1 | EN-128 | AUD-029, VAL-028 |
