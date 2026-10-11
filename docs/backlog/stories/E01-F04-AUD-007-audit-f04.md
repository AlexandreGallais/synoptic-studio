---
id: AUD-007
epic: E01
feature: F04
title: "Audit F04: free rotation of shapes"
status: ready
points: 2
---

# E01 · F04 · AUD-007 — Audit F04: free rotation of shapes

Full check of F04 in depth and of the project in breadth, following [audit.md](../../conventions/audit.md) (ADR-0020), with the `/audit` skill.

## Acceptance criteria

- Given the functions of F04, when audited, then every formula is re-derived, every expected value recomputed by hand, and each function survives a mutation spot-check.
- Given the sources, when audited, then every `@see` of F04 is re-verified online and dated.
- Given the project, when audited, then no duplicate, no copied code and no inconsistency between code, domain, ADRs and docs remains unrecorded.

## Tasks

One task = one commit, referenced as `AUD-007.Tn`.

- [ ] T1 — A. Independent review by the `auditor` subagent; mathematics, mutation spot-checks, missing properties (2 h)
- [ ] T2 — B to H. Sources, provenance, duplicates, consistency, quality gates, research list for the next feature, light evolvability check; findings table (1.5 h)
