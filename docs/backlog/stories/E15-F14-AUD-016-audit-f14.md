---
id: AUD-016
epic: E15
feature: F14
title: Audit F14
status: ready
points: 2
---

# E15 · F14 · AUD-016 — Audit F14

The feature in depth, the project in breadth, following [audit.md](../../conventions/audit.md) (ADR-0020), with the `/audit` skill.

## Acceptance criteria

- Given the functions of F14, when audited, then every formula of `DERIV-text-box` is re-derived, every expected value recomputed by hand, the balanced wrap checked against a search of every way to break small texts, and each function survives a mutation spot-check.
- Given the sources, when audited, then every `@see` of F14 is re-verified and dated.
- Given the project, when audited, then no duplicate, no copied code, no unnamed calculation (ADR-0049) and no inconsistency between code, domain, ADRs and docs remains unrecorded.

## Tasks

One task = one commit, referenced as `AUD-016.Tn`.

- [ ] T1 — The audit by independent reviewers and its findings (2 h)

## Findings

| Check | Result | Evidence | Action |
| ----- | ------ | -------- | ------ |
