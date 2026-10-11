---
id: AUD-006
epic: E01
feature: F09
title: Audit F09
status: ready
points: 2
---

# E01 · F09 · AUD-006 — Audit F09

Full check of F09 in depth and of the project in breadth, following [audit.md](../../conventions/audit.md) (ADR-0020), with the `/audit` skill.

## Acceptance criteria

- Given the functions of F09, when audited, then every formula is re-derived, every expected value recomputed by hand, and each function survives a mutation spot-check.
- Given the sources, when audited, then every `@see` of F09 is re-verified online and dated.
- Given the project, when audited, then no duplicate, no copied code and no inconsistency between code, domain, ADRs and docs remains unrecorded.

## Tasks

One task = one commit, referenced as `AUD-006.Tn`.

- [ ] T1 — A. Independent review by the `auditor` subagent; mathematics, mutation spot-checks, missing properties (2 h)
- [ ] T2 — B to H. Sources, provenance, duplicates, consistency, quality gates, next feature, light evolvability check; findings table (1.5 h)
