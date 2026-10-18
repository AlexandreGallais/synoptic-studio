---
id: F29
epic: E17
title: The local checks stable and fast
status: ready
---

# E17 · F29 — The local checks stable and fast

**Benefit hypothesis**: a push fails only on a real fault: no timing test fails under load, `check:all` spends nothing on the Rust lane when no Rust changed, and the main checkout's tools are never older than its sources after a pull.

Decided by the Product Owner on 2026-10-18, first of the tooling features cut by SP-024: about one push in three fails on a timing test (`src/model/text-wrap/wrapped-paragraph.test.ts`, "wraps 16 000 words in well under a second", 1.14 s against its 1 s that day; `[F18.AC19]` at 12.3 s against 5 s, load average near 60). From EN-121's builds and lanes (RET-004: C8, C9; I51, I97, I106, I112). A tooling feature, validated by the agent (ADR-0072); sources in [research note 0026](../../research/notes/0026-the-tooling-features-of-2026-10-18.md).

## Acceptance criteria

1. Every timing test of `src/` and `tests/` has a limit at least four times its duration alone, measured and written beside it, and passes in a loaded `check:all` (`.claude/rules/tooling.md`) (EN-129).
2. `check:all` skips the Rust lanes, saying so, when the branch changed no Rust input against its base, as it skips the docs lane, and runs the Rust coverage as a lane of its own (EN-130).
3. After a pull or a merge in a checkout, the release binaries whose sources changed are rebuilt before the next tool runs (EN-131).

## Stories

In delivery order.

| ID | Type | Title | Status |
| --- | --- | --- | --- |
| [EN-129](../stories/E17-F29-EN-129-timing-tests-that-hold-under-load.md) | EN | Timing tests that hold under load | done |
| [EN-130](../stories/E17-F29-EN-130-the-rust-lane-only-when-rust-changed.md) | EN | The Rust lane only when Rust changed, its coverage a lane | done |
| [EN-131](../stories/E17-F29-EN-131-binaries-rebuilt-after-a-pull.md) | EN | The binaries rebuilt after a pull | ready |
| [AUD-031](../stories/E17-F29-AUD-031-audit-f29.md) | AUD | Audit F29 | ready |
| [VAL-030](../stories/E17-F29-VAL-030-validate-f29.md) | VAL | Validate F29 | ready |

## Feature plan

| Acceptance criterion | Realized by | Verified by |
| --- | --- | --- |
| 1 | EN-129 | AUD-031, VAL-030 |
| 2 | EN-130 | AUD-031, VAL-030 |
| 3 | EN-131 | AUD-031, VAL-030 |
