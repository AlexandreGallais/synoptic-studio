---
id: EN-005
epic: E01
feature: F01
title: Local proportional radius clamp
status: done
points: 3
---

# E01 · F01 · EN-005 — Local proportional radius clamp

Compute the effective radius of every vertex from the requested radii (`DERIV-local-radius-clamp` steps 2 and 3, ADR-0007).

## Acceptance criteria

Sources: `DERIV-local-radius-clamp` (check table), principle of `REF-CSS-BR` §4.5, edge length by `REF-MATHWORLD-VECTOR-NORM` (SP-001).

- Given a 100 × 100 square with radius 1000 on one corner and 0 elsewhere, when clamped, then the effective radius of that corner is 100.
- Given an edge of length 25 between two right corners with radius 100, when clamped, then both effective radii are 12.5 within `EPSILON` (the float result is `12.500000000000004`).
- Given a 100 × 100 square with radius 50 on every corner, when clamped, then every effective radius is 50.
- Given aligned vertices with radius 30, when clamped, then no setback is produced (`s = 0`).
- Given a rectangle of size 0 × 50 (Q15), when clamped, then no `NaN` appears and no setback is produced: `S ≤ L` is tested before dividing (`L = S = 0` gives `f = 1`).

Float results are compared within `EPSILON = 1e-9` (Q10).

## Tasks

One task = one commit, referenced as `EN-005.Tn`.

- [x] T1 — `EPSILON` (Q10) and edge length `norm` (`math`, `REF-MATHWORLD-VECTOR-NORM`) with their tests (0.5 h)
- [x] T2 — Generalize `cyclicVertex` into `cyclicItem`, used for vertices and for per-vertex numbers (`geometry`) (0.5 h)
- [x] T3 — `Corner` (vertex and requested radius), setbacks, edge lengths, factor per edge (`geometry`, `DERIV-local-radius-clamp` steps 1–2) with the check table (1 h)
- [x] T4 — Factor per vertex and effective radii (`geometry`, step 3); properties: 0 ≤ effective ≤ requested, setbacks of an edge ≤ its length (1.5 h)
- [x] T5 — Audit findings: `turningAngle` gave π on a zero-length edge after a negative incoming vector (signed zeros), now 0 explicitly; factor ranges `[0, 1]`; independent setbacks in the overlap property (1 h)
