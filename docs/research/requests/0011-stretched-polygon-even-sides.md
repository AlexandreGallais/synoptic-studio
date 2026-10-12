# Research request 0011 — A polygon stretched to its box with sides as even as possible

**Status**: done (→ `notes/0011-f06-stretched-polygon.md`) — run by the Product Owner in claude.ai on 2026-10-12

## Precise question

I draw schematic SVG symbols from integer inputs, with segments and circular arcs only. A "regular polygon" tool takes a number of corners `n` (3 ≤ n ≤ 12) and a box `w × h` (integers ≥ 1). Today the polygon stays regular, its base flat at the bottom, and is scaled uniformly to fit the box, so it touches only two of its sides on one axis. I want a "stretched" option where the polygon touches **all four sides** of its box, stays **symmetric about the vertical axis** of the box with its **base flat on the bottom side**, keeps its `n` corners convex and in the same order, and has side lengths **as even as possible** (stretching makes some sides longer; the growth should be spread over all the sides so that the shape stays homogeneous).

1. Is there a known construction, in geometry, CAD or graphics literature, for an equilateral (or "most nearly equilateral") convex polygon inscribed in a rectangle, touching its four sides, with a given number of sides and an axis of symmetry? Give primary sources (books, papers, documentation pages) that I can read and cite.
2. Which measure of "as even as possible" is standard or best justified (for example: minimize the ratio of the longest to the shortest side, minimize the variance of side lengths, keep the vertices on the ellipse inscribed in the box with equal chords), and does it give a unique answer?
3. For each `n` from 3 to 12: which vertices touch which side of the box, how many degrees of freedom remain once symmetry and contact are imposed, and is there a closed form (for example the hexagon gives a quadratic) or does it need an iterative method (which one, with which convergence guarantee and tolerance)?
4. What should happen when equal sides are impossible: a triangle or a rectangle have no freedom left; a pentagon in a 300 × 50 box or a hexagon in a 50 × 300 box cannot have equal sides while touching the four sides.
5. Do drawing tools (Figma, Inkscape, Illustrator, Affinity, CAD tools) offer such a stretched polygon, and with which rule? (Figma's polygon was checked by hand: it does not fill its box.)

## Why it blocks

F06 (polygon stretched to fill its box), stories SP-009 and EN-017: the function deriving the stretched corners needs a rule the Product Owner can validate and a source or a derivation for each step (`CLAUDE.md`: no geometry function without a verified source). The Product Owner chose the goal (four sides touched, sides as even as possible) and left the exact rule to the research.

## Already consulted

| Source                                     | What it says                                                                                                                                                        | Why insufficient                                                |
| ------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------- |
| Wikipedia, "Equilateral polygon"           | definitions; an equilateral quadrilateral is a rhombus; a cyclic equilateral polygon is regular                                                                     | no construction inside a rectangle                              |
| Figma help, shape tools                    | the polygon tool, triangle by default                                                                                                                               | nothing on filling a box; the Product Owner's test: it does not |
| `DERIV-regular-polygon-fit` (this project) | the uniform fit of a regular polygon in its box                                                                                                                     | the regular case only                                           |
| Agent's own computation                    | hexagon in 100 × 100: top and bottom sides 54.858, the four others 54.858 (equal); pentagon in 100 × 100: equal sides of about 63.5, side corners at about y = 39.2 | no source; the impossible cases and the measure are open        |

## Expected deliverable

- [ ] formula or algorithm with its primary source, for each `n` from 3 to 12
- [ ] complete reference (authors, title, year, DOI or URL)
- [ ] known edge cases (no freedom, equal sides impossible, `w` or `h` small)
- [ ] verifiable test values: triangle, square, pentagon and hexagon in 100 × 100, 300 × 50 and 50 × 300
