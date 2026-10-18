---
id: F26
epic: E17
title: The mutation runner stops a mutation on Linux
status: done
---

# E17 · F26 — The mutation runner stops a mutation on Linux

**Benefit hypothesis**: the `Rust` check of #370 ends, its test `stops_a_mutation_that_never_ends` (`tools/mutate`) no longer waiting 46 min on GitHub's runner for a process group that was never stopped.

Found at #370's run 39138216016 (2026-10-18): the test hung from 15:03 until the run was cancelled at 15:50. `stop_with_children` runs `kill -KILL -<pid>`; on macOS that stops the group, but Linux's `kill` reads an argument starting with `-` as a signal before a `--`, so no process is signalled and the wait never ends. A tooling fix, validated by the agent (ADR-0072), before 0.3.0.

## Acceptance criteria

1. A test command over its limit is stopped with its process group on Linux as on macOS: the group is named after `--`, and the command itself is killed too (EN-127).

## Stories

In delivery order.

| ID | Type | Title | Status |
| --- | --- | --- | --- |
| [EN-127](../stories/E17-F26-EN-127-mutate-stops-on-linux.md) | EN | The mutation runner stops a mutation on Linux | done |
| [AUD-028](../stories/E17-F26-AUD-028-audit-f26.md) | AUD | Audit F26 | done |
| [VAL-027](../stories/E17-F26-VAL-027-validate-f26.md) | VAL | Validate F26 | done |

## Feature plan

| Acceptance criterion | Realized by | Verified by |
| --- | --- | --- |
| 1 | EN-127 | AUD-028, VAL-027 |
