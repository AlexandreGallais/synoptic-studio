---
id: EN-004
epic: E01
feature: F01
title: Fillet setback
status: done
points: 2
---

# E01 · F01 · EN-004 — Fillet setback

Compute the setback `s = r · tan(|τ| / 2)` of a fillet at a vertex (`DERIV-local-radius-clamp` step 1), for the radius clamp.

## Acceptance criteria

- Given r = 10 and a quarter turn, when the setback is computed, then it is 10.
- Given r = 30 and τ = 0, when the setback is computed, then it is 0 (no arc).
- Given r = 10 and |τ| = π/3, when the setback is computed, then it is 10 / √3 (`DERIV-fillet-setback` check table).
- Given a counter-clockwise turn, when the setback is computed, then only |τ| matters.

## Tasks

One task = one commit, referenced as `EN-004.Tn`.

- [x] T1 — Verify `REF-GG-FILLET` online and mark it `[verified]`, or request research (1 h) — not found online; replaced by `DERIV-fillet-setback` (Euclid, MathWorld, verified)
- [x] T2 — Write the tests of the cases above (0.5 h)
- [x] T3 — Implement the fillet setback (`geometry`) (1 h) — `filletSetback`
