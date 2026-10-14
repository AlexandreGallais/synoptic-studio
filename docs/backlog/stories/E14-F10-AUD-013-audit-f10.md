---
id: AUD-013
epic: E14
feature: F10
title: Audit F10
status: ready
points: 1
---

# E14 · F10 · AUD-013 — Audit F10

Independent audit of F10 (ADR-0020): criteria traced, formulas re-derived, expected values recomputed, sources re-read, duplicates and leftovers looked for.

## Acceptance criteria

- Given F10's criteria, when the audit runs, then each is traced to code and tests.
- Given its findings, when the audit ends, then each correctness finding is fixed and the rest recorded.

## Tasks

One task = one commit, referenced as `AUD-013.Tn`.

- [ ] T1 — Run the auditor and fix its findings (2 h)
