# 0005 — Sources of F07: polygon aligned in its box

Spike SP-003, 2026-10-11. Claude Code's own research; every source below was read.

## Placement (EN-009, US-009)

- SVG 2 already solves the problem: `preserveAspectRatio` with `meet` scales a box uniformly into a viewport (the smaller of the two ratios) and places it by one of nine alignments, `xMin`/`xMid`/`xMax` × `YMin`/`YMid`/`YMax` (`REF-SVG2-COORDS`, §8.2 steps 7 and 11–14, §8.7). F02's fit is `xMidYMid meet`; F07 adds the other eight.
- The translation adds nothing (min), half the room (mid) or all of it (max) on each axis. `DERIV-regular-polygon-fit` step 5 now states it, with check rows for the triangle, the square and the hexagon.
- Edges: on the anchored side the vertices land on 0 or the box size within a rounding error relative to the box; the two vertices of one edge may differ in their last bit (their unit coordinates come from `cos` and `sin`), and the output writes them as the integer edge (5 decimals, Q10). Found by the review of SP-003: an "exact in floating point" claim was dropped. The code follows SVG's formula as is: one factor 0, ½ or 1 of the room.

## Names (US-009)

- SVG names the alignments `Min`, `Mid`, `Max` per axis; Figma's plugin API names them `MIN`, `CENTER`, `MAX` (`REF-FIGMA-ALIGN`). Both are the same on both axes and refer to the smallest and largest coordinate, which in the SVG frame (y downwards) are the left and the top.
- Chosen for the model: one type `Alignment = "min" | "mid" | "max"` (SVG's words, the spec the derivation follows), and an `Anchor = { horizontal, vertical }` stored as the polygon's `anchor` (functions take three parameters at most: the anchor travels as one object). The interface shows the Product Owner's words: left, center, right; top, center, bottom. Glossary: anchor (ancre) = the pair of alignments; distinct from the stroke alignment (inner, center, outer) and from the Align command (`interaction.md` §5).

## Rounding (US-009)

Nothing new: the derivation's note and step 6 already state that the fit is computed on the sharp-cornered polygon, that a flat edge keeps touching at any radius (its straight part stays on the edge, `DERIV-fillet-setback`; at the maximum the incircle is tangent to every side) and that a rounded vertex leaves it. F07.AC4 is a consequence, tested with the anchored cases.

## Playground (US-010)

- Functional reference: Figma's alignment box in the right panel, set by clicking or with the arrow keys (`REF-FIGMA-AUTO-LAYOUT`); the 3 × 3 grid itself is the Product Owner's choice (F07).
- Keyboard: nine native radio buttons sharing one name follow the W3C radio group pattern (`REF-WAI-APG-RADIO`): Tab enters the group, the arrows move to the next or previous cell in reading order, wrapping. Figma's arrows move in two dimensions (down goes to the cell below); the radio pattern does not. Kept as the standard behavior, with no code of our own; a 2-D keyboard is a possible later improvement.

## Open

Nothing blocks the stories. The "always touch" option was left out by the Product Owner (F07, 2026-10-11).
