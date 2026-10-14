---
id: EN-030
epic: E14
feature: F13
title: "Pass 2: what pass 1 missed, and the duplicates"
status: ready
points: 2
---

# E14 · F13 · EN-030 — Pass 2: what pass 1 missed, and the duplicates

A second reading of the whole code base, on the result of pass 1, by a fresh agent; and the duplicates the audits left: `NO_HANDLES` and the curve contour's `NO_CORNER`, `midpoint` and `pointBetween`, `cubicVelocity` and `derivativeCoefficients`, the shape kinds told apart three ways, the per-axis product, the playground test helper `isDisabled`.

## Acceptance criteria

- Given the second reading, when the story ends, then what it found is named or left with its reason.
- Given the duplicates, when the story ends, then each is one function, unless merging would change a floating-point result (then recorded).

## Tasks

One task = one commit, referenced as `EN-030.Tn`.

- [x] T1 — The findings of the second reading (2 h)
- [x] T2 — The duplicates merged (2 h)
