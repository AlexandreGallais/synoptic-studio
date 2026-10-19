---
id: F31
epic: E17
title: The guard and the agent's commands
status: in-progress
---

# E17 · F31 — The guard and the agent's commands

**Benefit hypothesis**: the agent's calls in a worktree pass or fail for what they do, never for how they are written, and no command waits silently or rewrites a line it should keep, so a story spends no call on a refusal or a time-out.

Cut on 2026-10-18 by the Product Owner from E17's tool incidents (SP-024): EN-119 from F23, its id kept, with the cards of `replace` without an argument and of `npm run fix` on a checklist barrel (REV-006). From the card C2, the rows I12, I22, I40, I87, I93 and I103 (RET-004) and AUD-023's table of the guard's cases ([tool incidents](../../process/tool-incidents.md)). EN-119's criterion on a feature branch pushed with lease after its rebase is dropped: a feature enters `develop` by a merge commit (ADR-0086, decision 3), so no feature is rebased. A tooling feature, validated by the agent (ADR-0072).

## Acceptance criteria

1. In a worktree, the guard reads a command by its words: a read-only chain, a multi-line commit message and a URL go through; `mkdir …heavy.lock`, a `python3` writing a file and a worker's write into the main checkout are refused; `git-sequences state` gives a worktree's state in one call (EN-119).
2. `replace` called alone with a terminal for input, with `--help` or with an argument prints its usage and ends at once; called alone with a closed input, as the agent's Bash tool gives it, it ends at once with `no edit block read`; and `npm run fix` keeps a type-only re-export (`export type * from …`) as it is (EN-138; worded so by AUD-033, F6).

## Stories

In delivery order.

| ID | Type | Title | Status |
| --- | --- | --- | --- |
| [EN-119](../stories/E17-F31-EN-119-the-guard-reads-by-words.md) | EN | The guard reads a command by its words | done |
| [EN-138](../stories/E17-F31-EN-138-replace-and-fix-do-what-they-say.md) | EN | replace and npm run fix do what they say | done |
| [AUD-033](../stories/E17-F31-AUD-033-audit-f31.md) | AUD | Audit F31 | done |
| [VAL-032](../stories/E17-F31-VAL-032-validate-f31.md) | VAL | Validate F31 | ready |

## Feature plan

| Acceptance criterion | Realized by | Verified by |
| --- | --- | --- |
| 1 | EN-119 | AUD-033, VAL-032 |
| 2 | EN-138 | AUD-033, VAL-032 |
