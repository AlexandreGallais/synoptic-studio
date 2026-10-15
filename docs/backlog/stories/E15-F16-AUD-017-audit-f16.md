---
id: AUD-017
epic: E15
feature: F16
title: Audit F16
status: ready
points: 2
---

# E15 · F16 · AUD-017 — Audit F16

The feature in depth, the project in breadth, following [audit.md](../../conventions/audit.md) (ADR-0020), with the `/audit` skill.

## Acceptance criteria

- Given the functions of F16, when audited, then every formula of `DERIV-color-spaces` is re-derived, every expected value recomputed by hand, the round trip re-run over every color, and each function survives a mutation spot-check.
- Given the sources, when audited, then every `@see` of F16 is re-verified and dated.
- Given the project, when audited, then no duplicate, no copied code, no unnamed calculation (ADR-0049) and no inconsistency between code, domain, ADRs and docs remains unrecorded.

## Tasks

One task = one commit, referenced as `AUD-017.Tn`.

- [ ] T1 — The audit by independent reviewers and its findings (2 h)

## Findings

| Check | Result | Evidence | Action |
| ----- | ------ | -------- | ------ |
