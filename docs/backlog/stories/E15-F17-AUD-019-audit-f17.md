---
id: AUD-019
epic: E15
feature: F17
title: Audit F17
status: ready
points: 3
---

# E15 · F17 · AUD-019 — Audit F17

Full check of F17 in depth and of the project in breadth, following [audit.md](../../conventions/audit.md) (ADR-0020), with the `/audit` skill.

## Acceptance criteria

- Given the functions of F17, when audited, then every formula is re-derived, every expected value recomputed by hand, and each function survives a mutation spot-check.
- Given the sources, when audited, then every `@see` of F17 is re-verified online and dated.
- Given the project, when audited, then no code, test, doc or step of the removed options (the balanced wrap, the word broken at a character, the HTML layer) is left, and no duplicate, copied code nor inconsistency remains unrecorded.

## Tasks

One task = one commit, referenced as `AUD-019.Tn`.

- [ ] T1 — A. Independent review by the `auditor` subagent; mathematics, mutation spot-checks, missing properties (3 h)
- [ ] T2 — B. Sources re-verified (1 h)
- [ ] T3 — C, D, E. Provenance, duplicates, consistency, removed options left nowhere (2 h)
- [ ] T4 — F, G. Quality gates and research list for the next feature; findings table (1 h)

## Findings

| Check | Result | Evidence | Action |
| ----- | ------ | -------- | ------ |
