---
id: SP-017
epic: E15
feature: F16
title: "Research for F16: colors in hexadecimal, HSL and HSB, and their picker"
status: in-progress
points: 2
---

# E15 · F16 · SP-017 — Research for F16: colors in hexadecimal, HSL and HSB, and their picker

Find, read and record the sources of every story of F16 before its implementation, then write its stories (ADR-0020, research protocol §8). F05's refinement, rounds 1 and 2, settled the business rules of colors: a hexadecimal value and an opacity from 0 to 100 %, both always shown; a picker on a click, with a hue slider, an opacity slider and a plane in HSL or HSB, its hexadecimal translation shown; no RGB fields, no CSS color names. What they leave open is decided here, provisional, to confirm at the feature's validation.

## Acceptance criteria

- Given each story of F16, when the spike ends, then every formula, algorithm and behavior it needs has a verified `REF-*`, a `DERIV-*` or an open research request.
- Given the sources found, when the spike ends, then the stories' criteria and tasks cite them, and research note 0019 records what was found and what remains open.
- Given F16's third criterion, when the spike ends, then `DERIV-color-spaces` says which representation keeps the round trip from hexadecimal exact, checked on every one of the 16 777 216 colors.

## Questions

| Question                                                                                                                              | Story       |
| ------------------------------------------------------------------------------------------------------------------------------------- | ----------- |
| Which hexadecimal forms does a designer paste (3, 4, 6 or 8 digits, upper or lower case, with or without `#`), and what do they mean? | EN-037      |
| How does an 8-bit alpha map to an opacity from 0 to 100 %, and back, without loss?                                                    | EN-037      |
| How are HSL and HSB defined from red, green and blue, and back (CSS Color 4 sample code, HSV formulas)?                               | EN-038      |
| What hue has a grey, whose hue is undefined?                                                                                          | EN-038      |
| Which representation keeps hexadecimal → HSL or HSB → hexadecimal exact: integer degrees and percents, decimals, or none?             | EN-038      |
| Which axes does the plane of a picker show, in HSL and in HSB?                                                                        | US-035      |
| How is a picker's slider used with the keyboard (WAI-ARIA slider pattern)?                                                            | US-035      |
| How do Figma and Inkscape show a color's hexadecimal value, its opacity and its HSL or HSB values?                                    | US-035      |
| What must every shape and text of E01, E14, F05 and F14 keep?                                                                         | every story |

## Tasks

One task = one commit, referenced as `SP-017.Tn`.

- [x] T1 — Inventory the formulas and behaviors of the stories (1 h)
- [x] T2 — Find, read and record the sources; write `DERIV-color-spaces` (3 h)
- [x] T3 — Write research note 0019, the domain's provisional rules and F16's stories (2 h)
