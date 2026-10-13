---
id: AUD-012
epic: E14
feature: F12
title: Audit the changes asked at the validation
status: draft
points: 1
---

# E14 · F12 · AUD-012 — Audit the changes asked at the validation

Independent audit of EN-026, SP-012 and US-027 before VAL-011 resumes: formulas re-derived, expected values recomputed, sources re-read, nothing left of what was removed (ADR-0020).

## Acceptance criteria

- Given the derivation of SP-012, when re-derived, then every check value holds.
- Given the code, the playground and the docs, when audited, then nothing serves only a removed method or mode, and every rule of `docs/domain/` agrees with the code.
