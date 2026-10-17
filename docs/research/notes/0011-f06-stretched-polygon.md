# 0011 — Sources of F06: polygon stretched to its box

Spike SP-009, 2026-10-12. Answer to [research request 0011](#research-request), run by the Product Owner in claude.ai (Research mode); summed up here in the agent's words, every source marked verified re-read by the agent the same day.

## What was asked

A regular polygon of `n` corners (3 to 12), its base flat at the bottom, symmetric about the vertical axis of its box, touching the four sides of a `w × h` box, its `n` corners convex and in the same order, its sides as even as possible.

## Findings

### No published construction

No book, paper or tool documents such a polygon. The rule has to be a derivation, each step citing a read source.

- Drawing tools stretch by a plain scaling, which makes the sides uneven: an affine map keeps lines and ratios on a line, not lengths nor angles (`REF-MATHWORLD-AFFINE`). Illustrator's live polygon scales non-uniformly and offers "Make Sides Equal", which returns to the regular polygon (`REF-ADOBE-LIVE-SHAPES`). Inkscape's polygon has no box-filling option (`REF-INKSCAPE-POLYGON`). Figma's polygon does not fill its box (Product Owner's test).
- PowerPoint's preset hexagon and octagon fill their box with insets measured from the shorter side, so their cut corners keep fixed angles and the straight sides absorb the stretch: a third rule, read only second-hand (`REF-OOXML-ISSUE-465`); the standard itself (`REF-ECMA-376`) is not read.
- An equilateral quadrilateral is a rhombus; a convex equilateral pentagon is fixed by two consecutive angles (`REF-WIKI-EQUILATERAL-POLYGON`).

### Even sides alone are not enough

- The usual measure of evenness is the **edge ratio**, the longest side over the shortest, 1 when all are equal (`REF-VTKM-EDGE-RATIO`, from the Verdict library, `REF-VERDICT-2007`, not read). Other measures (variance, least squares) agree when equal sides exist.
- Minimizing it alone loses corners: a stretched square becomes a trapezoid; in a 300 × 50 box the pentagon's apex flattens; in 50 × 300 the hexagon's side corners vanish. Even when equal sides exist, corners can become almost flat: the equal-sided octagon of the λ family (below) in 100 × 50 has corners turning 13.6°, against 45° when regular.
- So a rule must also protect the corners, measured by their **turning angle**, the angle between the directions of the two sides meeting there (`REF-CARROLL-2013`); a regular polygon turns `2π/n` at each corner.
- A closed polygon is fixed, up to its place, by its side lengths and turning angles (`REF-CRANE-DISCRETE-CURVES`): the shape can be built from directions of sides of length 1.
- Equal sides are unique only for `n = 5` and 6; from 7 on, a family of equal-sided polygons fills the box, and a choice is needed. The research answer counted the octagon as unique too, assuming a central symmetry; the review of SP-009 found other equal-sided octagons filling a square (side directions 42°, 101.24°, 123.24°), so it is a family as well.

### The λ family

- Every side has length 1, and takes the direction of the matching side of the regular polygon after a vertical scaling by `λ`: `φⱼ = atan2(λ sin θⱼ, cos θⱼ)`, `θⱼ = 2πj/n`. For an odd `n`, the side reaching the apex takes the direction that closes the polygon. `λ = 1` is the regular polygon; `λ > 1` makes it taller.
- It gives the unique equal-sided polygon for `n = 5` and 6, and one member of the family for the others (for the octagon, the one with a central symmetry).
- `λ` is the value for which the polygon's height over its width equals `h / w`, found by bisection on `ln λ` (`REF-MATHWORLD-BISECTION`: after `k` steps the error is at most `2^−k` times the interval); closed forms for the hexagon and the octagon (`φ₁ = atan r + asin(r / (2√(1 + r²)))` for the hexagon, `r = h/w`).
- Bisection needs the aspect to grow with `λ`: checked on 601 values for every `n` from 5 to 12 by the research and on 6 001 values by the agent's tests, **not proved**.

### Where equal sides stop being sound

With every corner kept turning at least half its regular turn (`π/n`), equal sides are possible for these ratios `h / w`:

| `n` | Regular | Equal sides, corners kept |
| --- | --- | --- |
| 5 | 0.95106 | 0.63167 – 1.67401 |
| 6 | 0.86603 | 0.36603 – 1.27293 |
| 7 | 0.97493 | 0.62434 – 1.55632 |
| 8 | 1 | 0.61991 – 1.61313 |
| 9 | 0.98481 | 0.67233 – 1.59293 |
| 10 | 0.95106 | 0.54837 – 1.49813 |
| 11 | 0.98982 | 0.60000 – 1.60040 |
| 12 | 1 | 0.61699 – 1.62078 |

The triangle and the square have no freedom: plain stretch, an isosceles triangle and a rectangle.

## The three rules

| Rule | What it does | Hexagon 300 × 50 (edge ratio, smallest corner turn) |
| --- | --- | --- |
| **A** — plain stretch | the regular polygon scaled on each axis to the box, as drawing tools do; simple, sides uneven | 1.897, 18.4° |
| **B** — equal sides at any cost | equal sides whenever they exist; corners may almost vanish, impossible past a ratio | 1, 14.2° |
| **C** — equal sides, then plain stretch | equal sides (λ family) while every corner turns at least `π/n`; past that limit, that limit polygon is scaled on each axis to the box | 1.117, 14.7° |

Rule C, the answer's recommendation: always touches the four sides, symmetric, base flat, `n` convex corners in order, unique and continuous as `w` and `h` change, depends only on `h / w`, closed forms or one bisection. Where the box is within the range above, its sides are exactly equal.

## Test values (rule C, SVG frame, 5 decimals)

Drawing order of F02: clockwise on screen from the right end of the base.

| Shape, box | Vertices | Sides |
| --- | --- | --- |
| triangle 100 × 100 | (100, 100), (0, 100), (50, 0) | 100, 111.80340, 111.80340 |
| square 300 × 50 | (300, 50), (0, 50), (0, 0), (300, 0) | the rectangle |
| pentagon 100 × 100 | (81.75623, 100), (18.24377, 100), (0, 39.16418), (50, 0), (100, 39.16418) | all 63.51246 |
| pentagon 300 × 50 | (228.85967, 50), (71.14033, 50), (0, 12.85950), (150, 0), (300, 12.85950) | 157.71933, 80.25188, 150.55021 |
| hexagon 100 × 100 | (77.42919, 100), (22.57081, 100), (0, 50), (22.57081, 0), (77.42919, 0), (100, 50) | all 54.85838 |
| hexagon 300 × 50 | (204.90381, 50), (95.09619, 50), (0, 25), (95.09619, 0), (204.90381, 0), (300, 25) | 109.80762, 98.32744 |
| hexagon 50 × 300 | (41.47297, 300), (8.52703, 300), (0, 150), (8.52703, 0), (41.47297, 0), (50, 150) | 32.94593, 150.24217 |
| octagon 100 × 100 | the regular octagon | all 41.42136 |

Hand check, hexagon 100 × 100: `φ₁ = 45° + asin(1/(2√2)) = 65.7048°`; the unit half-width `1/2 + cos φ₁ = 0.91144`; scale `50 / 0.91144 = 54.85838`, the side. The agent's own value before the research was 54.858.

## Decided by the Product Owner (2026-10-12)

1. **Rule C**: equal sides while every corner keeps enough turn, then a plain stretch of that limit polygon.
2. **The corner guard is half of the regular turn** (`π/n`, 30° for the hexagon): a design value, not a sourced one; one third and two thirds were offered.
3. **The square stays a rectangle**, and the triangle isosceles: neither has freedom (not the trapezoid that the edge ratio alone prefers).

## Open, for the derivation (settled in `DERIV-stretched-polygon`, SP-009)

- Prove that the aspect of the λ family grows with `λ` for each `n`, and that the sound range contains `λ = 1` and is one interval; or test it exhaustively.
- A source for the identity `a sin φ − b cos φ = c ⇒ φ = atan(b/a) + asin(c / √(a² + b²))` (closed forms of the hexagon and octagon).
- Compute the limits `λ_lo`, `λ_hi` at run time or tabulate them with their derivation.
- Read ECMA-376 before citing the PowerPoint presets.

## Research request

Request `0011-stretched-polygon-even-sides.md` in `research/requests/`; the answer file was folded into this note and removed.
