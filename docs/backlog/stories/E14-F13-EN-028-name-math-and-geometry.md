---
id: EN-028
epic: E14
feature: F13
title: "Pass 1: name the calculations of math and geometry"
status: done
points: 2
---

# E14 · F13 · EN-028 — Pass 1: name the calculations of math and geometry

The math-expert agent lists, file by file, each inline calculation of `src/math` and `src/geometry` that mathematics names, with its source; each becomes a named function or a call to an existing one.

## Acceptance criteria

- Given the list, when the story ends, then each item is a named function with its `@see`, or left inline with its reason (a rewrite that would change the order of floating-point operations).
- Given every test, when the story ends, then no expected value was edited and `check:all` is green.

## Tasks

One task = one commit, referenced as `EN-028.Tn`.

- [x] T1 — The named functions of `math` and `geometry`, each with its property test (3 h)

## Review

Math expert, before the last commit: every replacement re-derived bit for bit, ±0 included, and checked by machine where the golden file may not reach (38 416 rotations with signed zeros, subnormals and infinities; 2 000 scales per stretched polygon; π/n for n up to 99 999). Fixed: tests that a swap of cosine and sine or of two Bernstein weights would pass; two test titles; one constant for half a turn; comments where a constant or a sum is borrowed. Left for pass 2: `unitStretchedPolygon`'s reflection, the uniform scale of `fitInBox`, the direction `unit(subtract(b, a))` of `filletArc`.
