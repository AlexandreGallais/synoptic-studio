---
id: AUD-012
epic: E14
feature: F12
title: Audit the changes asked at the validation
status: done
points: 1
---

# E14 · F12 · AUD-012 — Audit the changes asked at the validation

Independent audit of EN-026, SP-012 and US-027 before VAL-011 resumes: formulas re-derived, expected values recomputed, sources re-read, nothing left of what was removed (ADR-0020).

## Acceptance criteria

- Given the derivation of SP-012, when re-derived, then every check value holds.
- Given the code, the playground and the docs, when audited, then nothing serves only a removed method or mode, and every rule of `docs/domain/` agrees with the code.

## Tasks

- [x] T1 — Audit across EN-026, SP-012, US-027 and US-028; findings fixed (1 h)

## Result

Auditor across the four stories, 2026-10-14. F12.AC1–AC8 traced to tagged tests; the derivation re-derived and its checks recomputed, 47.74907 and 46.86292 by brute force; nothing left of the methods removed; domain, ADR-0047 and code agree. Fixed: a side of length 0 beside a curve drew an arc off its circle (its direction was rounding noise); the equilateral check and both parameters between two curves now tested; the derivative's two forms tested to agree. Left for the refactor pass: `NO_HANDLES` and `NO_CORNER` written in model and geometry, `midpoint` beside `pointBetween`.
