# Shapes

## 1. Shape model

Processing chain: typed numbers → integer model → computed geometry → `<path>` (ADR-0005).

- A shape = one or more **closed contours** (the first is the outside, the next ones are holes, produced by booleans).
- A contour = ordered list of **vertices**: integer coordinates when they are the model (a rectangle), fractional when they are derived from it (a regular polygon, later booleans), never written back (ADR-0003).
- Each vertex carries an integer **corner radius** (0 = sharp corner).
- The shape is rendered as a single `<path>` (ADR-0002).
- Geometric primitives: segments and circular arcs only (ADR-0001).

Consequence: every shape is described by numbers. The drawing is **determined**, not drawn by hand.

### Contour orientation (Q11)

- Contours are listed **clockwise on screen** (SVG frame, y pointing down), **starting at the top-left vertex**.
- Where no vertex is both topmost and leftmost (a triangle pointing up), the contour starts at the topmost vertex, the leftmost on a tie (Q18): the start is a natural one, not a strict rule; only the clockwise order matters to the geometry.
- A contour is **cyclic**: the last vertex joins the first; every per-vertex computation (corner radius, clamping) wraps around, so the first vertex sees the last edge and the last vertex sees the first edge.
- Once its corners are rounded, the top-left vertex is no longer on the drawn outline: the written path starts where the first corner's arc ends and closes with that arc, keeping the clockwise order (`DERIV-fillet-arc` step 7).

### Size (Q20)

- No upper limit chosen in the model nor in the calculations: a size is any safe integer ≥ 0 (up to 2⁵³ − 1, the largest integer a double holds exactly); coordinates lose precision long before that bound.
- Good practice: one user unit is one screen pixel. Draw a view at the size of its screen — even a wall of 8K screens stays far below 100 000 pixels — and let the SVG scale for a screen seen from far.
- The interface caps a width, height or radius typed above 100 000 at 100 000 (US-008).
- `EPSILON` stays absolute. Rounding errors are about 1e-16 × the size: 1e-11 at 100 000, 1e-10 at a million, so under `EPSILON` = 1e-9 up to a few million units. Beyond, tiny segments of rounding may remain, and near the largest safe integer an output coordinate can be off by 0.5 (AUD-002): a relative tolerance is not needed for real screens.

### Precision (Q10)

- `SVG_DECIMALS = 5`: derived coordinates are written with at most 5 decimals, in their shortest form: no trailing zeros (`2.5`, not `2.50000`), never a negative zero (`-0.000001` is written `0`).
- `EPSILON = 1e-9`: tolerance of floating-point comparisons.
- The user only ever enters integers; decimals only appear in derived geometry and output.

### Rotation of shapes

- In the Symbol Editor, a shape may be rotated by **any integer angle in degrees** (e.g. a rectangle tilted by 45°). The angle is stored; the rotated vertices are derived (ADR-0003).
- Instances in the View Editor only rotate by quarter turns (ADR-0008).

## 2. Corner radius

Reference behavior: Figma.

- Radius entered **numerically**, never with the mouse.
- Radius **per vertex**: select one or more vertices → type a value.
- Global radius: applied to every vertex of the shape.
- Geometry: arc tangent to both edges; setback `d = r / tan(θ/2)` (`DERIV-fillet-setback`).

### Radius clamping (ADR-0007)

Rule: **local proportional reduction per edge** (`DERIV-local-radius-clamp`).

- Two corners with the same radius on a too-short edge → each stops at the middle.
- A single corner pushed to the maximum → the arc covers the whole edge, and goes no further.
- Never an "anti-corner": the arcs of one edge never overlap.
- No error message: the effective value simply caps.
- Only the vertices adjacent to a conflicting edge are reduced.
- A spike — a vertex where the contour turns back on itself — is consumed by its fillet: rounding may eat length (Q16, settled).
- Where no corner can be rounded (a size of 0, an aligned vertex), the radius stays on the vertex and the interface shows it "as requested": it applies as soon as the shape grows (Q17, settled).
- Known limit: on a concave contour, an arc might touch a non-adjacent edge; the clamp only checks adjacent edges. To be detected, not corrected, when concave shapes arrive (F03, node editing); unreachable with rectangles.

Storing the value (Q8, settled):

- The model keeps the **requested** radius (e.g. 1000).
- The **effective** radius is derived at every evaluation (e.g. 100 on a 100 square), never stored (ADR-0003).
- Enlarging the shape makes the rounding grow up to the requested value.
- Interface: show the requested value and signal the effective value when they differ.

### Useful special case

- Square of side `c` + radius `c/2` on all 4 corners = **circle**. The circle is therefore not a primitive: it is a rounded square.
- Same for a regular polygon with maximal radius: every fillet stops at the middle of its edges, so the polygon becomes exactly its inscribed circle.

## 3. Creation tools

### Rectangle

- Parameters: `width`, `height` (integers ≥ 0; negative forbidden, 0 allowed — Q15) and a global corner `radius` (integer ≥ 0; 0 = sharp corners). A radius larger than the rectangle is valid: it is clamped, not refused (ADR-0007); the requested value is kept (Q8).
- Result: contour of 4 vertices.

### Regular polygon

Functional reference: Inkscape's Star/Polygon tool.

- Parameters: number of corners, an integer `3 ≤ n ≤ 12` (Q19); `width` and `height` of the box, integers ≥ 0 (negative forbidden, 0 allowed — Q15); a global corner `radius`, integer ≥ 0 (0 = sharp corners), clamped rather than refused (ADR-0007), the requested value kept (Q8).
- Examples: `n=3` equilateral triangle, `n=4` square, `n=5` pentagon, `n=6` hexagon.
- Rule: the shape fills as much of the `width × height` box as possible **without exceeding it**.
- Derived vertices (cos/sin): not integers. They are **computed**, not stored (ADR-0003). `n`, `width`, `height`, the requested `radius` and the `anchor` are stored (`RegularPolygon`).

**Chosen mode: uniform** (`DERIV-regular-polygon-fit`).

- The shape stays regular.
- It touches the width, the height, or both, depending on `n` and the ratio `w/h`.
- Default orientation: **flat base** (horizontal bottom edge). `n=4` gives a square, not a diamond.
- Fitted on the sharp-cornered polygon; the corner radius then rounds inwards, clamped exactly as for the rectangle (same geometry, no rule of its own). At the maximal radius the shape is its inscribed circle and may no longer touch the box (a hexagon in 100 × 100 becomes a circle of diameter 86.6): accepted by the Product Owner (2026-10-09) — removing the radius makes it touch again; a shape filling its box whatever its rounding would be a separate "fill the parent" option.
- Number of corners: integer `3 ≤ n ≤ 12` (Q19): process and electrical symbols use triangles, squares, diamonds, hexagons and one octagon (note 0004); beyond 12 sides a polygon is hardly told from a circle, and the circle is already a rounded square.
- Other orientations (a diamond is a square turned by 45°, the IEC hexagon stands on a vertex: 30°) come from rotation (F04), applied to the flat-based polygon.

**Anchor** (F07, Product Owner 2026-10-10 and 2026-10-11):

- The polygon is placed in the room its box leaves by an **anchor**, one of 9 places on a 3 × 3 grid: a horizontal alignment (left, center, right) and a vertical one (top, center, bottom), stored as `min`, `mid`, `max` (SVG's `preserveAspectRatio`, `DERIV-regular-polygon-fit` step 5). Default: center on both axes, F02's placement.
- `min` puts the polygon's smallest coordinate on the box's (left or top), `max` its largest (right or bottom): a triangle in 100 × 100 anchored at the bottom has its base written at y = 100, its apex at 13.39746. In floating point the vertices of an anchored edge are on it within a rounding error, and written as the integer edge (Q10).
- The anchor is an **intention, always stored**: on the axis the polygon fills, the three alignments give the same drawing, and the chosen one is still kept and shown.
- The anchor places the **sharp-cornered** polygon; the corner radius then rounds inwards without moving it: a flat face on the anchored side keeps touching it, a rounded vertex leaves it (a triangle anchored at the top with radius 10 has its top at y = 10). No "always touch" option (Product Owner, 2026-10-11).
- Rotation (F04) comes after: the box turns with its content.

### Text

- Exception to "everything is a path": native `<text>` (ADR-0002).
- Parameters: content, font, integer size, anchor (start / middle / end), color.
- Reason: converting text to paths requires reading font files, hence a library or a heavy home-made parser.

### Layout box (Q21)

Settled with the Product Owner (2026-10-10, refining F07): a box that places the elements inside it, « un peu comme dans Figma » (a frame with a layout), made simple for synoptic views rather than following CSS. Delivered by epic E13, with the shape tree; « layout » is a working name, kept only if no better term is found.

- The box is a **rectangle only**, with a corner radius at most: its vertices cannot be edited, otherwise placing its children becomes too complex.
- Its children are placed **by anchor** (the 3 × 3 grid of F07, top-left … bottom-right), or **stacked** in a row or a column with gaps; the box may have padding. Both modes exist.
- **Any child may override the anchor** and still be laid out by the box: « au lieu d'aller en ancre en haut à droite, moi, je choisis d'aller en ancre en bas à gauche ». This is what Figma lacks: its child in absolute position no longer follows the parent's layout.
- The box does **not resize to its children** (unlike Figma's hug): its size is the one the designer typed.
- Children: shapes, text, later symbols. The polygon in its own box (F07) is its single-child case.

- A child given **its own anchor** stays in the box (same group) but **leaves the stack completely**: the stack follows the box's default layout, the child its own. Overlaps are the designer's business: « l'option est bien, mais après, c'est à l'utilisateur de l'utiliser au mieux ».
- **Stacks are a drawing aid, fixed at run time**: they live in the Symbol Editor (still there when the symbol is edited again); once the symbol is built, every position is static. An element hidden by an animation leaves its place empty: the stack never closes the gap (« c'est juste cet élément à cet endroit-là qui est caché »).

### Out of scope (symbols)

- **Pen** and **freeform drawing** in the Symbol Editor: contrary to the "logical" principle.
- **Bézier curves**, Figma's **Bend** tool, handle **mirroring** in symbols (ADR-0001).
- **Corner smoothing** (Figma squircle smoothing): produces Béziers.

Static drawings of the View Editor follow other rules (§8).

## 4. Node editing

See `interaction.md` §3.

## 5. Boolean operations

Functional reference: Inkscape's Path menu (`REF-INKSCAPE-BOOL`).

| Operation    | French            | Result                                          |
| ------------ | ----------------- | ----------------------------------------------- |
| Union        | Union             | area covered by at least one shape              |
| Difference   | Différence        | bottom shape minus top shape                    |
| Intersection | Intersection      | common area                                     |
| Exclusion    | Exclusion         | area covered by exactly one shape (XOR)         |
| Division     | Division          | bottom shape cut into pieces by the top outline |
| Cut path     | Découpe de chemin | outlines cut at intersections, without fill     |

Reference algorithms: `REF-MARTINEZ-2009` (sweep line, O((n+k) log n), holes and multiple shapes), `REF-GREINER-HORMANN` (simpler, tricky degenerate cases). An extension to circular arcs is needed.

**Chosen mode: non-destructive** (ADR-0006).

- An operation creates a boolean node `{ operation, operands[] }`; the operands stay integer and editable.
- The result (fractional vertices) only exists in the derived geometry, never in the model.
- The operands' corner radius is applied before the operation.
- **Shape Builder** tool (reference: Inkscape 1.3, `REF-INKSCAPE-SHAPEBUILDER`): non-destructive version, a region is stored as "inside A, outside B…".
- No destructive flattening in v1.
- Corner radius on the new vertices of a result: not available in v1 (Q9).

## 6. Strokes

The stroke is **computed as a path**, not through the SVG `stroke` property (ADR-0002).

| Property            | Values                     | Note                                                          |
| ------------------- | -------------------------- | ------------------------------------------------------------- |
| Width               | integer ≥ 0                | —                                                             |
| Alignment           | `inner`, `center`, `outer` | model of the W3C `stroke-alignment` draft (`REF-SVG-STROKES`) |
| Join                | `miter`, `round`, `bevel`  | + miter limit, as in SVG (`REF-SVG2-PAINT`)                   |
| Cap (open contours) | `butt`, `round`, `square`  | —                                                             |
| Dashes              | pattern of integers        | exact arc length: `r × θ`                                     |

Rules:

- Open contour: alignment ignored (no inside), as in design tools (`REF-SVGWG-957`).
- Advantage of the segments + arcs scope: **the offset of a circular arc is a concentric circular arc**. The offset is therefore exact, without approximation.
- Geometry: offset of the contour at ±width, then the ring between both contours.

## 7. Shape tree

See `interaction.md` §6.

## 8. Static drawings

- Made by business users in the View Editor, with **more freedom** than symbols but few options.
- A drawing has no parameter and no animation. Selecting a group and saving it creates a drawing.
- Drawings go into a **drawing library shared between projects**, so that the same drawings are reused from one synoptic view to another.
- Curves are allowed **only** in drawings (Q12, ADR-0018): a Figma-like pen places points, and dragging a point pulls Bézier handles. Symbols keep ADR-0001 (segments and arcs only).
