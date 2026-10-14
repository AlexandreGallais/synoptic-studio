---
id: AUD-011
epic: E14
feature: F12
title: Audit F12
status: done
points: 1
---

# E14 · F12 · AUD-011 — Audit F12

Independent audit of F12 before its validation: formulas re-derived, expected values recomputed, sources re-read, duplicates looked for (ADR-0020).

## Acceptance criteria

- Given each derivation of F12 (`DERIV-cubic-sides`, `DERIV-handle-angle`, the derivations of shares and of the tension (removed by EN-026), `DERIV-free-shape-stretch` step 5), when re-derived, then every check value holds.
- Given F12.AC1 to AC6, when audited, then each is proven by a tagged test.
- Given the decisions taken during the stories, when audited, then each is in the domain or an open question, and the domain agrees with the code.

## Result

Auditor across the feature, and the doc editor on its pages, 2026-10-13. Every criterion F12.AC1–AC6 is met and traced to tagged tests; every derivation re-derived and its checks recomputed; every source re-read; turned and stretched shapes probed across stories.

| Finding                                                                                                                             | Fix                                                                                                     |
| ----------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------- |
| the list of points and the radius field showed radii clamped without the curve rule (a point in handles mode shown 40, drawn sharp) | `filletableCorners` exported (folder `geometry/curve`) and used by both; test on the rectangle 120 × 80 |
| `interaction.md` and Q28 still said a deleted point's neighbours are joined by a straight edge, an added point always sharp         | both point to the F12 rules in `shapes.md`                                                              |
| US-026's decisions said the automatic handles are never stored                                                                      | reworded: stored as last computed                                                                       |
| stale TSDoc after F12 (midpoint, deletion, box, pieces, share way)                                                                  | updated                                                                                                 |
| a `cubic-bezier()` pasted on a side of length 0 gave zero handles silently                                                          | refused, as a share typed                                                                               |
| the paste's number pattern cited SVG for a stricter rule than its grammar                                                           | the comment says it is the project's choice; test                                                       |
| five copies of the zero vector, six of the −0 idiom                                                                                 | `ZERO_VECTOR` and `withoutNegativeZero` in `math/vector` (`REF-ECMA-262-NUMBER-ADD`)                    |
| the domain section on handles: long bullets, one mode under five names                                                              | rewritten as tables; the modes, sides and methods added to the word list                                |
| AUD-011 and VAL-011 without criteria                                                                                                | written                                                                                                 |

Left as they are: `PERCENT` and `X_AXIS` repeated in a few modules of `model/handles` (local constants, one meaning each); `roundedContour` beside `curveContour` (the box frame still uses it). For the Product Owner: ADR-0046 says handles are typed as integers — typed values are, but pasted and computed handles are stored with their decimals.
