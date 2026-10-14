---
id: EN-029
epic: E14
feature: F13
title: "Pass 1: name the calculations of the model, io, render and playground"
status: done
points: 2
---

# E14 · F13 · EN-029 — Pass 1: name the calculations of the model, io, render and playground

The same pass on `src/model`, `src/io`, `src/render` and the playground: an inline calculation with a mathematical name calls the function of that name.

## Acceptance criteria

- Given those folders, when the story ends, then each named calculation calls its function, or is left inline with its reason.
- Given every test, when the story ends, then no expected value was edited and `check:all` is green.

## Tasks

One task = one commit, referenced as `EN-029.Tn`.

- [x] T1 — The calls in the model, io, render and playground (2 h)

## Review

Auditor, before the last commit: every replacement re-derived; fixed — the conversion buttons' test of a flat shape kept as it was (`!hasArea` differs when a fractional product underflows), the TSDoc of `hasArea` and `clamp`, the split point's mirrored handle written as `mirroredHandle`, the folders table. Left inline with its reason: turning by an angle, already two named steps.
