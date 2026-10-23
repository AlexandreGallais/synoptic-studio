---
id: EN-149
epic: E02
feature: F42
title: The helps drawn within the area shown and its padding
status: ready
points: 2
---

# E02 · F42 · EN-149 — The helps drawn within the area shown and its padding

The helps of the layer above the drawing are drawn only within the area shown and its padding, so that a large rounded shape's band stays fast (the Product Owner, 2026-10-23, SP-028's answer 8). A band about a contour (`contourBand`) is computed only for the parts of its pieces within a box, the area shown widened by the padding SP-030's answer 7 sets: an arc by the angles where it lies within the box, a cubic by halves whose control points' box meets it, before sampling, so that nothing is sampled out of sight ([note 0031](../../research/notes/0031-f42-zoom-and-pan.md), §5). Draft until SP-030's answers 1 and 7.

## Acceptance criteria

The examples use a canvas of 1 200 × 800 CSS pixels, the opening view at 100 %, origin (−40, −40), the area shown from (−40, −40) to (1 160, 760); R, a rounded rectangle 99 999 × 100 000 at (0, 0) of radius 100 000, its corners quarter circles of radius 49 999.5.

- Given R selected at 100 %, then its band has no subpath, none of its contour coming within 19 353.4 units of the area, where the band of its four corners alone is 157 084 subpaths today, 156 to 200 ms and 15 513 064 characters of path data (measured on 2026-10-23, note 0031, §5); its redraw takes under 16 ms, measured in a test.
- Given R fitted to the canvas, at a scale of 0.0072, then its band follows each corner by about 283 chords of at most 2 screen pixels, all within the area and shown.
- Given any contour, a half width, a step and a box, then the band within the box covers every point of the box nearer to the contour than the half width less the chords' deviation from it, which the derivation bounds, its chords still at most the step, and none of its subpaths lies wholly outside the box widened by the half width, a segment meeting the box drawn whole (`DERIV-band-within-box`): a property test on random contours and boxes checks both, at sample points of the box.
- Given a pan that keeps the area shown within the padded box, then the helps are not computed again, the `viewBox` alone changed, if SP-030's answer 7 pads by more than the band's half width.
- Given the shape under the pointer, then it is still found on its whole contour (`shapeAtPoint`), the band alone cut.

## Tasks

- [ ] T1 — `DERIV-band-within-box` written for the `math-expert`'s check (an arc's angles within a box; a cubic's halves by de Casteljau, left out when their control points' box misses it); the band within a box (`src/geometry/contour-band/`), `@kind geometry`, its TSDoc and the tests of the criteria, hand-computed bands, degenerate cases (a box inside a band's width, an arc tangent to the box), the property test; failing (1 h)
- [ ] T2 — The band within the box, the hover's and the selection's outlines computed within the area shown and its padding; R measured; Q48 and `interaction.md` §2 updated; the tests of T1 passing (2 h)

Files: `src/geometry/contour-band/`, `playground/hover/` (the outlines within the area shown), `docs/derivations/` (`DERIV-band-within-box`). `@see` `DERIV-band-within-box`, `REF-SVG2-PAINT`, `REF-WIKI-BEZIER`, `REF-POMAX-BEZIER`, `REF-MDN-RAF`, `docs/domain/interaction.md#_2-selector`.

One task = one commit, referenced as `EN-149.T1` in the commit footer.
