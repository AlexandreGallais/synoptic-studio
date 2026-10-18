---
id: F23
epic: E02
title: E17's tool incidents fixed
status: draft
---

# E02 · F23 — E17's tool incidents fixed

**Benefit hypothesis**: with the tools that failed during E17 fixed together, before E02's first library feature, a run of E02 meets none of E17's nine incidents again, and the method of ADR-0084 and ADR-0085 is held by the tools rather than by sentences.

Decided by the Product Owner on 2026-10-18 at E17's retrospective ([RET-004](../stories/E17-RET-004-retrospective-e17.md), P8 and P9): E02's first feature, a tooling feature validated by the agent (ADR-0072), four stories. Its sources are the tool incident cards C1 to C9 ([tool incidents](../../process/tool-incidents.md)) and the improvement rows the record assigns to it (§2); its refinement is drafted here, to be completed when E02 starts.

## Acceptance criteria

1. A story attached to `develop` rather than a feature gets its worktree, its pull request and its log rows by the same tools as any story; `tool-incidents.md` is a log of the shared files, `move-rows` moves a worker's `### Incidents`, and a test checks the cards' form (EN-118).
2. In a worktree, the reads a worker chains are answered in one call, the guard reads a command by its words, and a feature branch is pushed with lease only by its rebase onto `develop` (ADR-0084) (EN-119).
3. The status line, `metrics`, `epic-check`, `session-report`, `val-prepare` and the library report what happened: no ended agent shown running, the command after `|| exit 9` named, a `dropped` story closed, the CI of the pull request into `main` read, a page's visits counted per project (EN-120).
4. The main checkout's binaries are rebuilt after a pull, `check:rust` runs only when Rust changed, the tests reading Inter hold under load, and the checks RET-004 listed are added (EN-121).

## Stories

In delivery order.

| ID | Type | Title | Status |
| --- | --- | --- | --- |
| [EN-118](../stories/E02-F23-EN-118-stories-outside-a-feature-and-the-cards.md) | EN | Stories outside a feature, and the cards as a log | draft |
| [EN-119](../stories/E02-F23-EN-119-the-guard-in-a-worktree.md) | EN | The guard in a worktree, and the feature rebased with lease | draft |
| [EN-120](../stories/E02-F23-EN-120-reports-that-say-what-happened.md) | EN | Reports that say what happened | draft |
| [EN-121](../stories/E02-F23-EN-121-builds-lanes-and-checks.md) | EN | Builds, lanes and the checks RET-004 listed | draft |
| [AUD-025](../stories/E02-F23-AUD-025-audit-f23.md) | AUD | Audit F23 | draft |
| [VAL-024](../stories/E02-F23-VAL-024-validate-f23.md) | VAL | Validate F23 | draft |

## Feature plan

| Acceptance criterion | Realized by | Verified by |
| --- | --- | --- |
| 1 | EN-118 | AUD-025, VAL-024 |
| 2 | EN-119 | AUD-025, VAL-024 |
| 3 | EN-120 | AUD-025, VAL-024 |
| 4 | EN-121 | AUD-025, VAL-024 |
