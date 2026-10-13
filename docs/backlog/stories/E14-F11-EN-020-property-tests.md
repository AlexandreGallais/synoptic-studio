---
id: EN-020
epic: E14
feature: F11
title: Property tests for the nine functions without one
status: done
points: 3
---

# E14 · F11 · EN-020 — Property tests for the nine functions without one

Give a property test (ADR-0022) to each function listed in `tests/fitness.test.ts`: `alignmentFactor`, `boxAspect`, `cornerSetbacks`, `edgeFactors`, `hasLength`, `isSoundStretch`, `mirrorAcrossAxis`, `stretchedRightHalf`, `stretchedSideDirections`.

## Acceptance criteria

- Given each listed function, when its test file runs, then at least one `test.prop` states an invariant that holds for every valid input (a bound, a symmetry, a round trip), checked by the `verifier` agent.
- Given the fitness test, when the story ends, then its list is gone and it checks each function of `src/math/` and `src/geometry/` in its **own** test file: `stretchScale` and `turningAngles`, which passed only because another file's property named them, get one too (found at the start of the story).

## Tasks

One task = one commit, referenced as `EN-020.Tn`.

- [x] T1 — Properties of the five box, alignment and corner functions (2 h)
- [x] T2 — Properties of the stretched-polygon functions, `stretchScale` and `turningAngles`; the fitness test checks each function's own test file (3 h)
