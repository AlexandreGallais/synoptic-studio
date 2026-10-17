---
id: EN-042
epic: E15
feature: F05
title: A turned text keeps its turned top-left corner when its block changes size
status: done
points: 2
---

# E15 · F05 · EN-042 — A turned text keeps its turned top-left corner when its block changes size

The library computes again the position of a turned text whose block, or box, changes size, so that its turned top-left corner keeps its place on the drawing; the text still turns about its middle (ADR-0057, adding to ADR-0055's decision 3; `DERIV-turned-shape`, step 5, as a width typed on a turned shape).

Product Owner decision of 2026-10-15, from a try of the playground before VAL-014. Built on the latest branch of the chain, `feature/f15-fonts-html`, after US-036.

## Acceptance criteria

- Given a text whose block is 100 × 20 at (47, 61), turned by 23, whose turned top-left corner is (54.88207, 42.25840), when its block grows to 120 × 20 (a letter spacing raised), then its position becomes (46.20505, 64.90731) and its turned top-left corner stays (54.88207, 42.25840), within 1e-9; without the new position, it would move to (55.67702, 38.35108).
- Given the same text, when its block grows to 120 × 40 (a second line), then its position becomes (42.29774, 64.11236) and the corner stays.
- Given a text in a box 60 × 40 at (0, 0), turned by 90, when its letter spacing changes, then its position does not change: the box keeps its size; when the box is resized to 80 × 40, then its turned top-left corner stays in place.
- Given a text at 0°, when its block grows, then its position does not change and nothing is computed.
- Given a text read with a fractional, finite position, then it is accepted; a position that is not finite is refused (`isValidText`, ADR-0057, ADR-0035).

## Tasks

One task = one commit, referenced as `EN-042.Tn`.

- [x] T1 — The position of a turned text after its block or its box changes size, from the corner kept (`positionAfterResize`, or its shared core, `@see DERIV-turned-shape` step 5, ADR-0057); tests first, the hand-computed cases above, a property test: the corner does not move (2 h)
- [x] T2 — `isValidText` accepts a finite fractional position; the domain and F05's fifth criterion brought up to date; tests first (1 h)
