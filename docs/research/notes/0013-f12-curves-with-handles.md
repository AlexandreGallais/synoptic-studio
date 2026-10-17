# 0013 — Sources of F12: points on a curve with handles

Spike SP-011, 2026-10-13. How tools let a user type the handles of a curve, how a point holds both a corner radius and handles, and what cubic curves change in the library. Sources in [references](../../references.md#curves-and-handles).

## Ways to type a handle (Q32)

Running example: a side from point A (100, 0) to point B (200, 0); A's handle leaving towards B, 30 long, 30° above the side (the page's y grows downwards, as in SVG): its vector is (30 cos 30°, −30 sin 30°) ≈ (25.98, −15), rounded to the nearest whole number below.

| Method | Used by, source | A's handle typed as | Integers typed |
| --- | --- | --- | --- |
| 1. **x, y of the handle's end, on the page** | SVG's absolute `C` (`REF-SVG2-PATHS`); asked for in Inkscape (`REF-INKSCAPE-HANDLE-COORDS`); the Product Owner's choice | x = 126, y = −15 | yes |
| 2. **dx, dy from its point** | how Figma stores a handle, a tangent vector from its vertex (`REF-FIGMA-VECTOR-NETWORK`); SVG's relative `c`; the other option of the Inkscape request | dx = 26, dy = −15 | yes |
| 3. **angle and length, from the page's x axis** | AutoCAD's relative polar input, `@30<30` there, its angles counterclockwise (`REF-AUTOCAD-POLAR`) | angle = 330°, length = 30 (clockwise from 0 to 359, as a shape's angle, F04) | yes |
| 4. **angle and length, from its side** | none found in a tool: the Product Owner's idea, the angle measured from the direction of the side the handle leaves along, so that the same values bend any side the same way | angle = 330° from the side, length = 30 | yes |
| 5. **share of the side** | CSS `cubic-bezier(x1, y1, x2, y2)` places both control points as fractions of a unit square between the ends (`REF-CSS-EASING`); here a share along the side and a share across it, the second handle measured from B | 26 % along, 15 % across | yes, as whole percentages |
| 6. **paste curve data** | a segment of SVG path data `C x1 y1 x2 y2 x y` (`REF-SVG2-PATHS`), or a CSS `cubic-bezier()` value (`REF-CSS-EASING`), whose diagonal from (0, 0) to (1, 1) is laid on the side | `C 126 -15 …` | the text pasted |
| 7. **a tension, the handles computed** | Inkscape's auto-smooth nodes, whose handles follow the neighbours (`REF-INKSCAPE-NODE-TYPES`); the cardinal spline's tangent `(1 − c)(next − previous) / 2`, c = 0 the Catmull–Rom spline, as handles `m / 3` (`REF-WIKI-HERMITE`) | tension 0 (Catmull–Rom), from the points before and after A | yes, as a whole percentage |

Findings:

- **Typing handles is rare**: Inkscape's toolbar shows the node, not the handle, and a request asks for either absolute or relative coordinates (`REF-INKSCAPE-HANDLE-COORDS`); Figma's help on vector editing describes handles dragged on the canvas and no field for them (`REF-FIGMA-EDIT-VECTOR`, an inference from what the page leaves out). Building the seven methods and trying them, as the Product Owner wants, is the way to learn which one is simplest.
- Methods 1 to 3 are the same vector written three ways; methods 4, 5 and 7 depend on the shape, so the **same values give the same bend anywhere**: on any side whatever its direction (4), its length too (5), or from the neighbouring points (7) — what the Product Owner asked for. With the same values at both ends of a side, both handles leave towards the outside of the shape, a symmetric arch (Product Owner, 2026-10-13): the handle before B is measured from the side's reverse direction, mirrored. Methods 4 and 5 refuse a side of length 0. Each needs a derivation (the rotation matrix, `REF-MATHWORLD-ROTATION-MATRIX`).
- Method 6 lays CSS's diagonal from (0, 0) to (1, 1) on the side, so that `cubic-bezier(0.42, 0, 0.58, 1)` keeps its S shape: along = (x + y) / 2, across = (y − x) / 2, "across" counted to the left of the travel from A to B, above a side going right (a derivation); x values outside 0 to 1 are refused, as CSS does.
- Method 6 reads text: a parser of one `C` segment, its end on B, and of `cubic-bezier()`, refusing anything else with a line saying why.

## The model of a point

| Rule | Source or decision |
| --- | --- |
| A handle is a vector from its point | Figma's `tangentStart` / `tangentEnd` (`REF-FIGMA-VECTOR-NETWORK`): moving or turning the point carries its handles |
| A zero handle is a straight side | "A line segment is just a cubic spline segment where both handles have a tangent of `{x: 0, y: 0}`" (`REF-FIGMA-VECTOR-NETWORK`) |
| Three modes per point | `NONE`, `ANGLE`, `ANGLE_AND_LENGTH` (`REF-FIGMA-HANDLE-MIRRORING`, `REF-FIGMA-EDIT-VECTOR`); default angle and length (Product Owner, F12) |
| Radius and handles both stored | Figma keeps `cornerRadius` and `handleMirroring` on the same vertex (`REF-FIGMA-VECTOR-NETWORK`); the Product Owner: the radius comes back when the handles go |
| Stored exactly | the handles of a free shape are stored as typed vectors, exact like its points (ADR-0031, ADR-0034); each method converts to and from them |

Proposed: a free shape's corner gets `handles: { isOn, mode, before, after }` beside its radius — `isOn` is the switch between the radius mode and the handles mode, `before` and `after` the vectors towards the previous and the next side. In handles mode the radius is kept but not drawn. Typing one handle sets the other by the mode: angle and length gives `before = −after`; angle keeps `before`'s length on the opposite direction (unchanged when the handle typed has length 0); none leaves it.

Handles live in the frame of the stored points, like them: stretching the shape stretches its handles, as resizing does in Figma. **The shape's box is the box of its curve**, not of its points (Product Owner, 2026-10-13, as Figma): a square 100 × 100 whose top bulges by 20 shows about 100 × 115. A point added in the middle of a curved side splits the curve in two at its middle, the outline kept (de Casteljau, `REF-WIKI-BEZIER`); deleting a point joins its two sides into one, keeping the outer handles.

**Limit kept for later**: a corner in radius mode next to a curved side cannot be filleted by a circular arc tangent to both sides (ADR-0007 works on straight sides); such a corner stays sharp until a rule is chosen.

## What curves change in the library

| Change | Source |
| --- | --- |
| A cubic piece in contours | SVG's `C x1 y1 x2 y2 x y` (`REF-SVG2-PATHS`); a side with a handle at either end becomes a cubic from A to B through A + A.after and B + B.before |
| Path writer | one more command, `C`, in `contourPiecesToPathData` |
| Straight box (bounding) | the curve's extremes where the derivative is zero for t from 0 to 1, plus its ends (`REF-POMAX-BEZIER`, `REF-WIKI-BEZIER`), the roots taken without cancellation (`REF-WIKI-QUADRATIC`); a derivation with hand-computed examples |
| Fit, stretch, turn | affine maps applied to the control points (`REF-WIKI-BEZIER`); a handle takes only the linear part; the box is computed again after a turn, never turned |
| Strokes | "you cannot offset a Bézier curve perfectly with another Bézier curve" (`REF-POMAX-BEZIER`): approximated by splitting, with a tolerance, decided at E04 (strokes), not in F12 |
| Closing side | a curved side from the last point to the first is written as an explicit `C` before `Z`; today the writer leaves the last straight side to `Z` |
| Circles for F10 | a quarter circle as one cubic is off by about 0.00027 of its radius (`REF-POMAX-BEZIER`), which advises circular arcs for circles; F10 decides, elliptical arcs included |

## Proposal

- **ADR-0046**: cubic Bézier sides allowed in symbols, from points in handles mode; arcs stay fillets only; strokes on curves left to E04.
- **Stories of F12**, in delivery order: the point's model (EN-024); the cubic in contours, path data, the curve's box, a point added or deleted on a curve (EN-025); the switch and method 1 in Points mode (US-022); method 2 (US-023); checkpoint; methods 3 and 4 (US-024); methods 5 and 6 (US-025); method 7 (US-026); audit; validation with one guided step per method.

Reviewed by the auditor and the math expert before the end of the spike: the `cubic-bezier()` mapping, the radius next to one curved side, the closing side, the frame of handles, points added or deleted on a curve and the constants of the circle were corrected from their findings.
