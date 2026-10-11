# 0005 — Sources of F07: polygon aligned in its box

Spike SP-003, 2026-10-11. Claude Code's own research; every source below was read.

## Placement (EN-009, US-009)

- SVG 2 already solves the problem: `preserveAspectRatio` with `meet` scales a box uniformly into a viewport (the smaller of the two ratios) and places it by one of nine alignments, `xMin`/`xMid`/`xMax` × `YMin`/`YMid`/`YMax` (`REF-SVG2-PRESERVE-ASPECT-RATIO`, §8.2 steps 7 and 11–14, §8.7). F02's fit is `xMidYMid meet`; F07 adds the other eight.
- The translation adds nothing (min), half the room (mid) or all of it (max) on each axis. `DERIV-regular-polygon-fit` step 5 now states it, with check rows for the triangle, the square and the hexagon.
- Exact edges: for `max`, writing `w − s · (max x − x)` instead of `Δx + s · (x − min x)` makes the extreme coordinate exactly `w` (the same algebra, step 5). For `min`, the coordinate is exactly 0. No new source is needed: the step is plain algebra on step 3's definition of the span.

## Names (US-009)

- SVG names the alignments `Min`, `Mid`, `Max` per axis; Figma's plugin API names them `MIN`, `CENTER`, `MAX` (`REF-FIGMA-ALIGN`). Both are the same on both axes and refer to the smallest and largest coordinate, which in the SVG frame (y downwards) are the left and the top.
- Chosen for the model: one type `Alignment = "min" | "mid" | "max"` (SVG's words, the spec the derivation follows), used for a `horizontalAlignment` and a `verticalAlignment`. The interface shows the Product Owner's words: left, center, right; top, center, bottom. Glossary: anchor (ancre) = the pair of alignments.

## Rounding (US-009)

Nothing new: the derivation's note and step 6 already state that the fit is computed on the sharp-cornered polygon, that a flat edge keeps touching at any radius (its straight part stays on the edge, `DERIV-fillet-setback`; at the maximum the incircle is tangent to every side) and that a rounded vertex leaves it. F07.AC4 is a consequence, tested with the anchored cases.

## Playground (US-010)

- Functional reference: Figma's alignment box in the right panel, a small grid set by clicking or with the arrow keys (`REF-FIGMA-AUTO-LAYOUT`).
- Nine native radio buttons sharing one name give the click and keyboard behavior without code of our own (the browser moves the choice with the arrow keys).

## Open

Nothing blocks the stories. The "always touch" option was left out by the Product Owner (F07, 2026-10-11).
