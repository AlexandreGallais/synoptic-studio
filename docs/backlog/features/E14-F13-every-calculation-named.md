---
id: F13
epic: E14
title: Every calculation named
status: done
---

# E14 · F13 — Every calculation named

**Benefit hypothesis**: a reader of any function sees, in the names it calls, which mathematical step each line is — "a rotation, then a projection, then a bisection" — instead of bare multiplications; the code reads as the theory it applies, and each step has its source.

**Product Owner's request (2026-10-14, night after VAL-011's first session)**: once F12 is done, so that no feature code changes at the same time, go over the whole code base twice. In each pass, every calculation that mathematics names — an operation, or several steps in a row that form one known construction — becomes a function of that name, with its source; then a second pass changes what the first one missed. No new feature. The approximations and business rules already decided stay as they are.

## Plan, to refine at its start

- Pass 1: list, folder by folder (`math`, `geometry`, `model`, `io`, `render`, `playground`), each inline formula and each sequence of steps with a mathematical name; turn them into named functions with a `REF-*` or a `DERIV-*` step; the math-expert agent checks each name against its source.
- Pass 2: the same, on the result, and the duplicates the audits left: `NO_HANDLES` and `NO_CORNER` in model and geometry, `midpoint` beside `pointBetween`, `cubicVelocity` beside `derivativeCoefficients`; the shape kinds told apart three ways (`isFreeShape`, `"anchor" in model`, `"kind" in model`, AUD-013); the per-axis product written in `stretchedToEllipse` and `stretchedHandles`; the test helper `isDisabled` in two playground tests.
- Every drawing unchanged, bit for bit (the property tests of F11 and F12).

## Refinement (2026-10-14)

The Product Owner asked for no further questions: F13 is done after F10, and both are presented together. Started from F10's validation branch, so that F10's code is renamed too; it reaches `develop` after F10.

## Acceptance criteria

1. Every calculation in `src/` that mathematics names is a call to a function of that name, with its `@see` source; a list of those left inline, each with its reason, is kept in the feature's audit.
2. The duplicates found by the audits are merged into one function each, unless merging would change a floating-point result (then both are kept and say why).
3. Every drawing is identical bit for bit: the tests' exact values unchanged, no expected value edited.
4. No new feature, no business rule changed, the approximations kept.

## Stories

| ID | Type | Title | Status |
| --- | --- | --- | --- |
| [SP-014](../stories/E14-F13-SP-014-research-f13.md) | Spike | Research for F13: the named calculations and their sources | done |
| [EN-028](../stories/E14-F13-EN-028-name-math-and-geometry.md) | Enabler | Pass 1: name the calculations of `math` and `geometry` | done |
| [EN-029](../stories/E14-F13-EN-029-name-model-and-playground.md) | Enabler | Pass 1: name the calculations of the model, io, render and playground | done |
| [CHK-013](../stories/E14-F13-CHK-013-checkpoint-f13.md) | Checkpoint | Checkpoint between the two passes | done |
| [EN-030](../stories/E14-F13-EN-030-second-pass.md) | Enabler | Pass 2: what pass 1 missed, and the duplicates | done |
| [AUD-014](../stories/E14-F13-AUD-014-audit-f13.md) | Audit | Audit F13 | done |
| [VAL-013](../stories/E14-F13-VAL-013-validate-f13.md) | Validation | Validate F13 | done |

Set ready by the run (ADR-0045).

## Feature plan

| Acceptance criterion | Realized by | Verified by |
| --- | --- | --- |
| F13.AC1 | SP-014, EN-028, EN-029, EN-030 | AUD-014 |
| F13.AC2 | EN-030 | AUD-014 |
| F13.AC3 | every story | `npm run check:all` |
| F13.AC4 | every story | AUD-014 |
