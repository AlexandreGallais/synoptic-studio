---
id: AUD-000
epic: E00
feature: F00
title: Audit F00
status: draft
points: 3
---

# E00 · F00 · AUD-000 — Audit F00

Full check of F00 in depth and of the project in breadth, following [audit.md](../../conventions/audit.md) (ADR-0020), with the `/audit` skill.

## Acceptance criteria

- Given the functions of F00, when audited, then every formula is re-derived, every expected value recomputed by hand, and each function survives a mutation spot-check.
- Given the sources, when audited, then every `@see` of F00 is re-verified online and dated.
- Given the project, when audited, then no duplicate, no copied code and no inconsistency between code, domain, ADRs and docs remains unrecorded.

## Tasks

One task = one commit, referenced as `AUD-000.Tn`.

- [ ] T1 — A. Independent review by the `auditor` subagent; mathematics, mutation spot-checks, missing properties (3 h)
- [ ] T2 — B. Sources re-verified (1 h)
- [ ] T3 — C, D, E. Provenance, duplicates, consistency (2 h)
- [ ] T4 — F, G. Quality gates and research list for the next feature; findings table (1 h)

## Findings

| Check | Result | Evidence | Action |
| ----- | ------ | -------- | ------ |
