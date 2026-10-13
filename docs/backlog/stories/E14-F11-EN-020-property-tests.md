---
id: EN-020
epic: E14
feature: F11
title: Property tests for the nine functions without one
status: ready
points: 3
---

# E14 · F11 · EN-020 — Property tests for the nine functions without one

Give a property test (ADR-0022) to each function listed in `tests/fitness.test.ts`: `alignmentFactor`, `boxAspect`, `cornerSetbacks`, `edgeFactors`, `hasLength`, `isSoundStretch`, `mirrorAcrossAxis`, `stretchedRightHalf`, `stretchedSideDirections`.

## Acceptance criteria

- Given each listed function, when its test file runs, then at least one `test.prop` states an invariant that holds for every valid input (a bound, a symmetry, a round trip), checked by the `verifier` agent.
- Given the fitness test, when the story ends, then its list is empty and the check holds for every function of `src/math/` and `src/geometry/`.

## Tasks

One task = one commit, referenced as `EN-020.Tn`.

- [ ] T1 — Properties of the five box and alignment functions (2 h)
- [ ] T2 — Properties of the four stretched-polygon functions; the list emptied (3 h)
