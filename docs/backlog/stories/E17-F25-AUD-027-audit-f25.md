---
id: AUD-027
epic: E17
feature: F25
title: Audit F25
status: done
points: 1
---

# E17 · F25 · AUD-027 — Audit F25

A one-line change of `ci.yml`, audited by the main thread against the failing log of run 39138216016 and #370's next run (ADR-0086: a specialist only where its independence pays).

## Acceptance criteria

- Given F25's criterion, when the audit ends, then it is traced to a test titled `[F25.AC1] …`, broken once on purpose and seen failing.

## Tasks

- [x] T1 — Criterion traced, the condition read against `REF-GH-CONTEXTS` (main thread)

One task = one commit, referenced as `AUD-027.T1`.

## Findings

Audited by the main thread (ADR-0086: a one-line condition, no specialist's independence to pay for).

| Id | Check | Result |
| --- | --- | --- |
| 1 | `[F25.AC1]`'s two tests traced, seen failing before the fix (EN-126.T1) | passed after T2 |
| 2 | `matrix.scripts` in a step's `if`: available there (`REF-GH-CONTEXTS`, Context availability) | no finding |
| 3 | The test lane's `scripts` value is `test` in `ci.yml`'s matrix, the one the conditions name | no finding |
| 4 | Every other lane running `cargo deny`: none but `check:rust` (`rust:deny` in its scripts) and the tests' `[F19.AC9]` | no finding |
| 5 | The cause missed at F24: AUD-026 checked that each lane runs once, not what each lane's tests need from the other lanes | a row for the next review |

## For the main thread

### Improvements

| 2026-10-18 | AUD-027 | A lane split (F24) checked each lane ran once but not what a lane's tests need from another lane's setup: `[F19.AC9]` in the test lane needed the Rust lane's `cargo-deny` and database (#370, exit 101); a test that each lane's scripts find their tools would have caught it | tool |
