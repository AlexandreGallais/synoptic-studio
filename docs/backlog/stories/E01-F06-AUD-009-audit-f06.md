---
id: AUD-009
epic: E01
feature: F06
title: "Audit F06: polygon stretched to fill its box"
status: ready
points: 2
---

# E01 · F06 · AUD-009 — Audit F06: polygon stretched to fill its box

Full check of F06 in depth and of the project in breadth, following [audit.md](../../conventions/audit.md) (ADR-0020), with the `/audit` skill.

## Acceptance criteria

- Given the functions of F06, when audited, then every formula is re-derived, every expected value recomputed by hand, and each function survives a mutation spot-check.
- Given the sources, when audited, then every `@see` of F06 is re-verified online and dated.
- Given the project, when audited, then no duplicate, no copied code and no inconsistency between code, domain, ADRs and docs remains unrecorded.

## Tasks

One task = one commit, referenced as `AUD-009.Tn`.

- [ ] T1 — A. Independent review by the `auditor` subagent; mathematics, mutation spot-checks, missing properties (2 h)
- [ ] T2 — B to H. Sources, provenance, duplicates, consistency, quality gates, research list for the next feature, light evolvability check; findings table (1.5 h)
