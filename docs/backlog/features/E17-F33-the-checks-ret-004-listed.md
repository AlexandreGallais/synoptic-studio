---
id: F33
epic: E17
title: The checks RET-004 listed
status: in-progress
---

# E17 · F33 — The checks RET-004 listed

**Benefit hypothesis**: what E17's retrospective found held only by a sentence is held by a check that fails, so a regression of a floor, a version, a path, an anchor or an agent page is caught at the push rather than at a review.

Cut on 2026-10-18 by the Product Owner from E17's tool incidents (SP-024): EN-121 from F23, its id kept, holding RET-004's list of checks, its builds and lanes gone to F29, and a `bench` binary for the stories' measures. From RET-004's themes "Checks to add" (I37, I54, I55, I56, I58, I60, I81, I99) and "Builds and lanes" (P8, story 4). Last in the order of the tooling features. A tooling feature, validated by the agent (ADR-0072).

## Acceptance criteria

1. The Rust coverage floor is 95 % of lines and functions, `typeCheck` is set in `.oxlintrc.json`, the cargo tools' versions are checked as the other dependencies are, and the editor's TypeScript path is the installed TypeScript 7, each held by a test (EN-121).
2. A heading's anchor is the same on GitHub and on the site, `move-rows` names the notes it leaves, `backlog:index --check` reaches the index tool, and no agent page names a binary through `npm run`, each held by a test (EN-140).
3. A `bench` binary measures a command's median duration over runs, for the stories that set a limit from a measure (EN-141).

## Stories

In delivery order.

| ID | Type | Title | Status |
| --- | --- | --- | --- |
| [EN-121](../stories/E17-F33-EN-121-floors-and-versions-checked.md) | EN | The floors, versions and paths checked | done |
| [EN-140](../stories/E17-F33-EN-140-anchors-rows-and-agent-pages-checked.md) | EN | The anchors, the rows and the agent pages checked | done |
| [EN-141](../stories/E17-F33-EN-141-a-bench-binary.md) | EN | A bench binary for the stories' measures | done |
| [AUD-035](../stories/E17-F33-AUD-035-audit-f33.md) | AUD | Audit F33 | ready |
| [VAL-034](../stories/E17-F33-VAL-034-validate-f33.md) | VAL | Validate F33 | ready |

## Feature plan

| Acceptance criterion | Realized by | Verified by |
| --- | --- | --- |
| 1 | EN-121 | AUD-035, VAL-034 |
| 2 | EN-140 | AUD-035, VAL-034 |
| 3 | EN-141 | AUD-035, VAL-034 |
