---
id: EN-149
epic: E02
feature: F42
title: The helps drawn within the area shown and its padding
status: done
points: 2
---

# E02 · F42 · EN-149 — The helps drawn within the area shown and its padding

The helps of the layer above the drawing are drawn only within the area shown and its padding, so that a large rounded shape's band stays fast (the Product Owner, 2026-10-23, SP-028's answer 8). A band about a contour (`contourBand`) is computed only for the parts of its pieces within a box, the area shown widened by half the canvas on each side (SP-030's answer 7), kept until a pan leaves it, the view's scale or the drawing changes: the chords of each piece cut by halves of their run, a half left out before its samples are computed when its middle sample lies farther from the box than the half's length bound, so that nothing is sampled out of sight ([note 0031](../../research/notes/0031-f42-zoom-and-pan.md), §5; `DERIV-band-within-box`, which says why the samples' indices are halved rather than an arc's angles and a cubic's parameter). Ready since SP-030's answers of 2026-10-23.

## Acceptance criteria

The examples use a canvas of 1 200 × 800 CSS pixels, the opening view at 100 %, origin (−40, −40), the area shown from (−40, −40) to (1 160, 760), the helps computed within it widened by half the canvas on each side, from (−640, −440) to (1 760, 1 160) (SP-030's answer 7); R, a rounded rectangle 99 999 × 100 000 at (0, 0) of radius 100 000, its corners quarter circles of radius 49 999.5.

- Given R selected at 100 %, then its band has no subpath, none of its contour coming within 18 647.0 units of the helps' box, where the band of its four corners alone is 157 084 subpaths today, 156 to 200 ms and 15 513 064 characters of path data (measured on 2026-10-23, note 0031, §5); 60 of its redraws take under a second, 16 ms each, measured in a test.
- Given R fitted to the canvas, at a scale of 0.01 (0.0072 asked, under the bound; AUD-044), then its band follows each corner by 393 chords of 199.84 units, 1.998 screen pixels, all within the helps' box and shown: the whole band.
- Given any contour, a half width, a step and a box, then the band within the box is the whole band's subpaths (`contourBand`) in their order, less those painting no point of the box, so that it paints the box as the whole band does, its chords still at most the step; none of its subpaths lies wholly outside the box widened by the half width, a chord meeting it drawn whole (`DERIV-band-within-box`): a property test on random segments, arcs, cubics and boxes checks both.
- Given a pan that keeps the area shown within the helps' box, at most 600 pixels across and 400 down from where the box was computed, then the bands are not computed again, the `viewBox` alone changed: a circle 2 000 across at (0, 0) selected, its band cut at x = 1 761, panned 100 pixels right, keeps the band of (−640, −440) to (1 760, 1 160); panned 700 pixels, its band is computed again within (60, −440) to (2 460, 1 160); a change of the drawing or of the scale computes the bands again.
- Given the shape under the pointer, then it is still found on its whole contour (`shapeAtPoint`), the band alone cut: R viewed from (49 000, 49 000) at 100 % is under the pointer at (49 100, 49 100), its outline empty there.

## Tasks

- [x] T1 — `DERIV-band-within-box` written for the `math-expert`'s check (the subpaths kept, the runs of chords left out by halves, the coverage the cut band keeps, the helps' box); the band within a box (`src/geometry/contour-band/`), `@kind geometry`, its TSDoc and the tests of the criteria, hand-computed bands, degenerate cases (a box inside a band's width, an arc tangent to the box), the property test; failing (1 h)
- [x] T2 — The band within the box, the hover's and the selection's outlines computed within the area shown and its padding; R measured; Q48 and `interaction.md` §2 updated; the tests of T1 passing (2 h)

Files: `src/geometry/contour-band/`, `src/model/view/` (`shownBox`), `playground/helps-frame/` (the box kept and the bands reused), `playground/hover/` (the outlines within the helps' box), `docs/derivations/` (`DERIV-band-within-box`). `@see` `DERIV-band-within-box`, `REF-SVG2-PAINT`, `REF-POMAX-BEZIER`, `REF-WIKI-TRIANGLE-INEQUALITY`, `docs/domain/interaction.md#_2-selector`.

One task = one commit, referenced as `EN-149.T1` in the commit footer.

## For the main thread

**The cut differs from the plan of note 0031, §5** (agent's choice, EN-149): rather than an arc cut by the angles where it meets the box and a cubic by de Casteljau halves left out by their control points' box, the runs of each piece's chords are halved by their samples' indices, a run left out when its middle sample lies farther from the box than the run's length bound (`DERIV-band-within-box`, steps 4 and 5). Halves at `t = ½` end between two samples when the chords are odd in number, so the band would no longer be the whole band's chords; the indices keep them exactly for every kind of piece, with one bound. For the `math-expert`'s check of the derivation, with `REF-WIKI-TRIANGLE-INEQUALITY`, read and added.

**Rule-docs**: `interaction.md` §2 (Computed where it is seen), Q48 and the guide page of F42 are changed; VAL-043 is dropped, so no decision to confirm is written there.
