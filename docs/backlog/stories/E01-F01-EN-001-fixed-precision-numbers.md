---
id: EN-001
epic: E01
feature: F01
title: Write numbers with fixed precision
status: done
points: 2
---

# E01 · F01 · EN-001 — Write numbers with fixed precision

Write a derived number in SVG output with at most `SVG_DECIMALS = 5` decimals (Q10), for every path coordinate of F01.

## Acceptance criteria

- Given `1`, when written, then the output is `1` (integers stay integers).
- Given `1 / 3`, when written, then the output is `0.33333`.
- Given `86.602540378`, when written, then the output is `86.60254`.
- Given `2.5000000001`, when written, then the output is `2.5` (no trailing zeros, Product Owner 2026-10-08).
- Given `-0.000001`, when written, then the output is `0` (never `-0`, Product Owner 2026-10-08).

## Tasks

One task = one commit, referenced as `EN-001.Tn`.

- [x] T1 — Write the tests of the cases above, values computed by hand (1 h)
- [x] T2 — Add the `SVG_DECIMALS` constant module in `src/io/` (0.5 h) — `svg-decimals.ts`
- [x] T3 — Implement the number formatting function (`format`) with its TSDoc and `@see REF-SVG2-PATHS` (1.5 h) — `formatSvgNumber`
- [x] T4 — Record the precision rule in `docs/domain/shapes.md` §1 if anything changes (0.5 h)
