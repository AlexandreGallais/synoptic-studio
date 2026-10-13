---
id: F13
epic: E14
title: Every calculation named
status: draft
---

# E14 · F13 — Every calculation named

**Benefit hypothesis**: a reader of any function sees, in the names it calls, which mathematical step each line is — "a rotation, then a projection, then a bisection" — instead of bare multiplications; the code reads as the theory it applies, and each step has its source.

**Product Owner's request (2026-10-14, night after VAL-011's first session)**: once F12 is done, so that no feature code changes at the same time, go over the whole code base twice. In each pass, every calculation that mathematics names — an operation, or several steps in a row that form one known construction — becomes a function of that name, with its source; then a second pass changes what the first one missed. No new feature. The approximations and business rules already decided stay as they are.

## Plan, to refine at its start

- Pass 1: list, folder by folder (`math`, `geometry`, `model`, `io`, `render`, `playground`), each inline formula and each sequence of steps with a mathematical name; turn them into named functions with a `REF-*` or a `DERIV-*` step; the math-expert agent checks each name against its source.
- Pass 2: the same, on the result, and the duplicates the audits left: `NO_HANDLES` and `NO_CORNER` in model and geometry, `midpoint` beside `pointBetween`, `cubicVelocity` beside `derivativeCoefficients`.
- Every drawing unchanged, bit for bit (the property tests of F11 and F12).

## Stories

To write at its refinement.
