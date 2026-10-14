---
id: EN-028
epic: E14
feature: F13
title: "Pass 1: name the calculations of math and geometry"
status: ready
points: 2
---

# E14 · F13 · EN-028 — Pass 1: name the calculations of math and geometry

The math-expert agent lists, file by file, each inline calculation of `src/math` and `src/geometry` that mathematics names, with its source; each becomes a named function or a call to an existing one.

## Acceptance criteria

- Given the list, when the story ends, then each item is a named function with its `@see`, or left inline with its reason (a rewrite that would change the order of floating-point operations).
- Given every test, when the story ends, then no expected value was edited and `check:all` is green.

## Tasks

One task = one commit, referenced as `EN-028.Tn`.

- [ ] T1 — The named functions of `math` and `geometry`, each with its property test (3 h)
