---
id: SP-003
epic: E01
feature: F07
title: Research for F07
status: done
points: 1
---

# E01 · F07 · SP-003 — Research for F07

Find, read and record the sources of every story of F07 before its implementation (ADR-0020, research protocol §8).

## Acceptance criteria

- Given each story of F07, when the spike ends, then every formula and behavior it needs has a verified `REF-*` or a `DERIV-*`.
- Given the sources found, when the spike ends, then the stories' criteria and tasks cite them, and a research note records what was found and what remains open.

## Questions

- Placement of a uniformly scaled box in a larger one with nine alignments: a normative source (SVG's `preserveAspectRatio` and its viewBox transform), and how `DERIV-regular-polygon-fit` step 5 changes.
- Edges: whether the anchored vertices land exactly on `x = 0`, `x = w`, `y = 0`, `y = h` in floating point (answer: within rounding, written exactly; note 0005).
- Names of the two settings and of their values (SVG, CSS box alignment, design tools), for the model and the glossary.
- A flat face stays on the anchored side at any radius, a rounded vertex leaves it: what the derivation already proves (step 6, incircle tangent to every side).
- The 3 × 3 anchor picker of design tools, as a functional reference for the playground.

## Inventory

| Story   | Needs                                                                       |
| ------- | --------------------------------------------------------------------------- |
| EN-009  | placement per alignment, anchored edges; `DERIV-regular-polygon-fit` step 5 |
| US-009  | names of the settings; default; validity; F02 unchanged at center           |
| US-010  | 3 × 3 picker behavior, keyboard; functional reference                       |
| AUD-003 | the sources above, re-verified                                              |
| VAL-003 | the check table of `DERIV-regular-polygon-fit`, anchored rows               |

## Tasks

One task = one commit, referenced as `SP-003.Tn`.

- [x] T1 — Find, read and record the sources; update the derivation (2 h)
- [x] T2 — Write the research note and update the stories (1 h)
