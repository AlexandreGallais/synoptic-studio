# Shapes

Business rules of the shapes of a symbol: model, rotation, corner radius, creation tools, booleans, strokes and static drawings.

## 1. Shape model

Processing chain: typed numbers → integer model → computed geometry → `<path>` (ADR-0005).

- A shape = one or more **closed contours** (the first is the outside, the next ones are holes, produced by booleans).
- A contour = ordered list of **vertices**: coordinates stored by a free shape (ADR-0032, ADR-0033: integers when typed, fractional as a conversion leaves them), fractional when they are derived from it (a regular polygon, later booleans), never written back (ADR-0003).
- Each vertex carries an integer **corner radius** (0 = sharp corner); a free shape keeps it with its point, as a corner (ADR-0033), a regular polygon in its list `radii` (ADR-0029); their number is the number of vertices, otherwise the shape is refused.
- The shape is rendered as a single `<path>` (ADR-0002).
- Geometric primitives: segments and circular arcs only (ADR-0001).

Consequence: every shape is described by numbers. The drawing is **determined**, not drawn by hand.

### Contour orientation (Q11)

- Contours are listed **clockwise on screen** (SVG frame, y pointing down), **starting at the top-left vertex**; a regular polygon starts at the right end of its flat base, then its left end, so that the base keeps points 1 and 2 when the number of corners changes (Q18 amended, F08).
- For other shapes, where no vertex is both topmost and leftmost, the contour starts at the topmost vertex, the leftmost on a tie (Q18): the start is a natural one, not a strict rule; only the clockwise order matters to the geometry.
- **Point numbers never change** once a shape exists: point 1 is the first point of the path's `d`, the others follow in its order, wherever its points move or however it turns (F08, F04): points are never renumbered. Only adding or deleting a point shifts the numbers after it by one (`interaction.md` §3). The top-left, clockwise start holds at creation only.
- A contour is **cyclic**: the last vertex joins the first; every per-vertex computation (corner radius, clamping) wraps around, so the first vertex sees the last edge and the last vertex sees the first edge.
- Once its corners are rounded, the top-left vertex is no longer on the drawn outline: the written path starts where the first corner's arc ends and closes with that arc, keeping the clockwise order (`DERIV-fillet-arc` step 7).

### Size (Q20)

- No upper limit chosen in the model nor in the calculations: a size is any safe integer ≥ 0 (up to 2⁵³ − 1, the largest integer a double holds exactly); coordinates lose precision long before that bound.
- Good practice: one user unit is one screen pixel. Draw a view at the size of its screen — even a wall of 8K screens stays far below 100 000 pixels — and let the SVG scale for a screen seen from far.
- The interface caps a width, height or radius typed above 100 000 at 100 000 (US-008).
- `EPSILON` stays absolute. Rounding errors are about 1e-16 × the size: 1e-11 at 100 000, 1e-10 at a million, so under `EPSILON` = 1e-9 up to a few million units. Beyond, tiny segments of rounding may remain, and near the largest safe integer an output coordinate can be off by a few units (0.5 measured in AUD-002, 2 for a square of side 2⁵³ − 1 in AUD-003): a relative tolerance is not needed for real screens.

### Precision (Q10)

- `SVG_DECIMALS = 5`: derived coordinates are written with at most 5 decimals, in their shortest form: no trailing zeros (`2.5`, not `2.50000`), never a negative zero (`-0.000001` is written `0`).
- `EPSILON = 1e-9`: tolerance of floating-point comparisons.
- The user only ever enters integers; decimals only appear in derived geometry and output.

### Rotation of shapes

- In the Symbol Editor, a shape may be rotated by **any integer angle in degrees** (e.g. a rectangle tilted by 45°). The angle is stored; the rotated vertices are derived (ADR-0003).
- A shape turns about the **center of its box**; only shapes turn, never a single point (Product Owner, 2026-10-11, F04).
- A positive angle turns **clockwise** on screen; the angle is kept **from 0 to 359**: 370 becomes 10, −90 becomes 270 (Product Owner, 2026-10-11, F04).
- Turning keeps the shape, as in Figma (Product Owner's rule): its width and height stay those before it turned, its corner radii, anchor (the box turns with its content) and point numbers do not change; back to 0 the shape is straight again. In Points mode, x and y are those of the turned point in the SVG (Q27). A turned polygon converted to a free shape keeps its angle (F04).
- Two boxes (Product Owner, 2026-10-11, F04): the **turned box**, the shape's box turned with it, shown as its selection (a turned square does not seem to grow); the **straight box**, the smallest upright rectangle around the drawn shape, rounded corners included, as a browser's `getBBox` (`DERIV-arc-bounding-box`), which grows at 45°; it is shown discreetly, with its x, y, width and height. The playground's view stays put, the origin at its top-left: a turned shape turns about its center on screen and may leave the canvas (Product Owner, VAL-007: a shape leaving the view is acceptable).
- **Position**: every shape has one, shown and typed as the x and y of the top-left of its straight box, as in Inkscape; the model stores the top-left of its box before it turns, so that changing a radius does not move a turned shape (ADR-0035). Turning does not move the center of the box, so a square turned by 45° has smaller x and y. A width typed on a turned shape keeps its turned top-left corner in place, as Figma; a point typed leaves the others where they are (F04, US-019); so do a point added in the middle of an edge and a deletion, and a turned polygon converted keeps its drawing.
- Instances in the View Editor only rotate by quarter turns (ADR-0008).

## 2. Corner radius

Reference behavior: Figma.

- Radius entered **numerically**, never with the mouse.
- Radius **per vertex**: select one or more vertices (node editing) → type a value (F03).
- Global radius: in shape mode, the radius field applies to every vertex of the shape. When the vertices have different radii it shows **Mixed** and is empty; typing a value gives it to every vertex (Product Owner, 2026-10-11, as in Figma).
- Changing the number of corners of a regular polygon keeps the radii **by vertex number**: vertex 1 keeps vertex 1's radius; removed vertices take their radius away; new vertices have radius 0 (Product Owner, 2026-10-11).
- No "independent corners" switch: the radius of one corner is set in node editing only.
- Geometry: arc tangent to both edges; setback `d = r / tan(θ/2)` (`DERIV-fillet-setback`).

### Points with handles (F12)

A point of a free shape can bend its two sides with two handles, as in Figma, without losing its corner radius (Product Owner decision, 2026-10-13). Drawing rules: ADR-0046; formulas: the derivations named in the tables.

**Radius or handles.** Each point has a switch with two states, always visible; its final layout is part of the UX pass at the end of E14. On a regular polygon, the switch first converts it to a free shape, unmoved, as deleting a point does (Product Owner, VAL-011, 2026-10-13).

| State         | The point is drawn                | Its corner radius                                                          |
| ------------- | --------------------------------- | -------------------------------------------------------------------------- |
| Corner radius | filleted (ADR-0007)               | drawn                                                                      |
| Handles       | its two sides bent by its handles | kept, not drawn; back as it was when switched off (30 stays 30, 0 stays 0) |

**Handle modes.** Typing one handle sets the other by the point's mode, chosen per point:

| Mode (playground)                 | Stored             | The other handle                                                                                           |
| --------------------------------- | ------------------ | ---------------------------------------------------------------------------------------------------------- |
| Mirror angle and length (default) | `angle-and-length` | opposite, same length: one handle at (130, 0) around a point at (100, 0) puts the other at (70, 0) (Figma) |
| Mirror angle                      | `angle`            | opposite direction, its own length kept; unchanged when the typed handle has length 0 (Figma)              |
| No mirroring                      | `none`             | unchanged (Figma)                                                                                          |

**Sides and box:**

- A side is a cubic curve when a handle is on at either of its ends and not of length 0 (ADR-0046); a point with its handles on but of length 0 leaves its sides straight. A point in handles mode is drawn sharp, its radius kept; the radius shown as effective is then 0. A corner in radius mode beside a curved side keeps its rounding: an arc tangent to the side and to the curve, the curve cut there (ADR-0047, `DERIV-fillet-beside-curve`; Product Owner, VAL-011, 2026-10-13). A radius too large is reduced, never drawn sharp: it may use the whole curve, whose other end is sharp; it becomes the largest circle tangent to both sides that crosses neither (a rectangle 120 × 80 with handles of 40 at point 3: a radius 60 at point 2 becomes 47.7).
- Handles stretch with the shape, like its points.
- A free shape whose points lie on one line is refused even when its sides bulge (a lens): its points span no area (Q33, Product Owner, VAL-011, 2026-10-14).
- The shape's box is the box of its curve, as in Figma: a square 100 × 100 whose top bulges by 20 shows about 100 × 115.

**Points added or deleted on a curve:**

| Edit                                         | Effect                                                                                                                                                                                                                                                                                                                                                  |
| -------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| A point added in the middle of a curved side | the curve is split at its middle and the drawing does not move — unless a neighbour's fillet reached past the middle: it is then rebuilt on the half left and may shrink (Q34, settled at VAL-011); the new point is in handles mode, Mirror angle and length; a neighbour in Mirror angle and length goes to Mirror angle (`DERIV-cubic-sides` step 4) |
| A point deleted                              | its two sides become one, keeping the outer handles                                                                                                                                                                                                                                                                                                     |

**Typing a handle.** Two methods, kept by the Product Owner at VAL-011 out of the seven tried (Q32, research note 0013, 2026-10-13). The method changes only how the stored vector is shown and typed.

| Method                      | Typed                                                                           | Rules                                                                                                                                                                                            | Derivation           |
| --------------------------- | ------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | -------------------- |
| x, y on the page            | the handle's end, as in Figma                                                   | —                                                                                                                                                                                                | —                    |
| angle, length from the side | degrees clockwise on screen from the side the handle leaves along, and a length | angle brought into 0 to 359 as a shape's (370 becomes 10, −90 becomes 270); a negative length refused, and a side of length 0; an angle typed while the length is 0 kept until a length is typed | `DERIV-handle-angle` |

With the same values at both ends of a side, the angle from the side gives a symmetric arch, the same at the four corners of a rectangle: outwards for angles from 180° to 359° on a clockwise contour (330° bends a side out by 30°), inwards for the others (Product Owner decision, 2026-10-13).

### Radius clamping (ADR-0007)

Rule: **local proportional reduction per edge** (`DERIV-local-radius-clamp`).

- Two corners with the same radius on a too-short edge → each stops at the middle.
- A single corner pushed to the maximum → the arc covers the whole edge, and goes no further.
- Never an "anti-corner": the arcs of one edge never overlap.
- No error message: the effective value simply caps.
- Only the vertices adjacent to a conflicting edge are reduced.
- A spike — a vertex where the contour turns back on itself — is consumed by its fillet: rounding may eat length (Q16, settled).
- Where no corner can be rounded (a size of 0, an aligned vertex), the radius stays on the vertex and the interface shows it "as requested": it applies as soon as the shape grows (Q17, settled).
- Known limit: on a concave contour, an arc might touch a non-adjacent edge; the clamp only checks adjacent edges. Concave contours exist since F08 (node editing); the case is not detected yet.

Storing the value (Q8, settled):

- The model keeps the **requested** radius (e.g. 1000).
- The **effective** radius is derived at every evaluation (e.g. 100 on a 100 square), never stored (ADR-0003).
- Enlarging the shape makes the rounding grow up to the requested value.
- Interface: show the requested value and signal the effective value when they differ.

### Useful special case

- Square of side `c` + radius `c/2` on all 4 corners = **circle**. The circle is therefore not a primitive: it is a rounded square. The round shape (F10) draws it without a radius field: a circle in a square box, an ellipse otherwise ("Round shape" below). A capsule is a rectangle whose radius is half its smaller side.
- Same for a regular polygon with maximal radius: every fillet stops at the middle of its edges, so the polygon becomes exactly its inscribed circle.

## 3. Creation tools

### Rectangle

- The rectangle is a **free shape** of 4 points made by the rectangle tool (Product Owner, 2026-10-11, F09): underneath, a rectangle is always a free shape. Width and height of any free shape stretch its points, the top-left of its box fixed; a rectangle stays a rectangle; stretching is without loss (0 then back gives the same shape). The eight handles of the box, the opposite side fixed as in Inkscape, come with E02.

- Model (ADR-0032, ADR-0033) — a **free shape** stores:
  - its **corners**, each a point in the shape's own box, its radius (integer ≥ 0) and its **handles** (F12, ADR-0046, "Points with handles" above); at least 3 corners, spanning a box of positive width and height (contour order Q11);
    - handles: the switch `isOn`, the `mode`, the vectors `before` and `after` from the point (in the frame of the stored points, stretched with them; exact, fractional after an angle or a stretch); a new point or a converted shape has them off, in `angle-and-length`, (0, 0);
  - its `width`, `height`, `angle` and position `x`, `y` (ADR-0035).
- Numbers: finite; integers when typed; fractional after a conversion (Q27) or an edit (ADR-0034); on a turned shape, integers in the drawing only, the point being stored un-turned (ADR-0035).
- Drawing: the stored points stretched from the top-left of their curve's box (`DERIV-free-shape-stretch`, ADR-0046), moved to the position, then turned by the angle (`DERIV-turned-shape`). The rectangle tool stores the unit square.
- A point typed in the SVG's coordinates: the corners are kept as drawn, that point replaced, the box fitted around them; on a turned shape the box is placed so that the other points stay where they are drawn (`DERIV-turned-shape` step 5).
- Rectangle tool parameters: `width`, `height` (integers ≥ 0; negative forbidden, 0 allowed — Q15), `angle` (0 to 359), `x`, `y` (integers, ADR-0003), and `radii`, the requested corner radius of each of its 4 vertices in contour order (integers ≥ 0; 0 = sharp corner; ADR-0029). They are creation values: the free shape the tool makes is what is edited afterwards, and its position may become fractional (ADR-0035). A radius larger than the rectangle is clamped, not refused (ADR-0007); the requested value is kept (Q8).
- Result: contour of 4 vertices.

### Regular polygon

Functional reference: Inkscape's Star/Polygon tool.

- Parameters: number of corners, an integer `3 ≤ n ≤ 12` (Q19); `width` and `height` of the box, integers ≥ 0 (negative forbidden, 0 allowed — Q15); `radii`, the requested corner radius of each of its `n` vertices in contour order, integers ≥ 0 (0 = sharp corner, ADR-0029), clamped rather than refused (ADR-0007), the requested values kept (Q8).
- Examples: `n=3` equilateral triangle, `n=4` square, `n=5` pentagon, `n=6` hexagon.
- Rule: the shape fills as much of the `width × height` box as possible **without exceeding it**.
- Derived vertices (cos/sin): not integers. They are **computed**, not stored (ADR-0003). `n`, `width`, `height`, the requested `radii`, the `anchor`, the `stretched` option (ADR-0037), the `angle` and the position `x`, `y` of its box (ADR-0035) are stored (`RegularPolygon`).

**Chosen mode: uniform** (`DERIV-regular-polygon-fit`).

- The shape stays regular.
- It touches the width, the height, or both, depending on `n` and the ratio `w/h`.
- Default orientation: **flat base** (horizontal bottom edge). `n=4` gives a square, not a diamond.
- Fitted on the sharp-cornered polygon; the corner radius then rounds inwards, clamped exactly as for the rectangle (same geometry, no rule of its own). At the maximal radius the shape is its inscribed circle and may no longer touch the box (a hexagon in 100 × 100 becomes a circle of diameter 86.6): accepted by the Product Owner (2026-10-09) — removing the radius makes it touch again; a shape filling its box whatever its rounding would be a separate "fill the parent" option.
- Number of corners: integer `3 ≤ n ≤ 12` (Q19): process and electrical symbols use triangles, squares, diamonds, hexagons and one octagon (note 0004); beyond 12 sides a polygon is hardly told from a circle, and the circle is already a rounded square.
- Other orientations (a diamond is a square turned by 45°, the IEC hexagon stands on a vertex: 30°) come from rotation (F04), applied to the flat-based polygon.

**Stretched option** (F06, Product Owner 2026-10-12):

- A "stretched" option of the regular polygon, off by default: the polygon then touches the **four sides** of its box, its base flat at the bottom, symmetric about the vertical axis of the box. Stretching makes some sides longer than others; the growth is spread over all the sides so that their lengths stay **as even as possible**. The rule (Product Owner, 2026-10-12, from research note 0011, rule C): all sides equal as long as every corner turns at least half of its regular turn (`π/n`); past that, that limit polygon is stretched on each axis to the box. The triangle stays isosceles and the square a rectangle: they have no freedom.
- In the code: `stretchedPolygon` (`DERIV-stretched-polygon`); the scale of rule C is found by bisection (`stretchScale`).
- It stays a polygon: its number of corners, radii (one per corner, the effective ones may differ since its corners no longer share one angle), angle and position work as for any polygon. Its anchor has no effect: it fills its box. Converted to a free shape, it does not move.
- Figma's polygon does not fill its box (the Product Owner tried it): no functional reference.

**Anchor** (F07, Product Owner 2026-10-10 and 2026-10-11):

- The polygon is placed in the room its box leaves by an **anchor**, one of 9 places on a 3 × 3 grid: a horizontal alignment (left, center, right) and a vertical one (top, center, bottom), stored as `min`, `mid`, `max` (SVG's `preserveAspectRatio`, `DERIV-regular-polygon-fit` step 5). Default: center on both axes, F02's placement.
- `min` puts the polygon's smallest coordinate on the box's (left or top), `max` its largest (right or bottom): a triangle in 100 × 100 anchored at the bottom has its base written at y = 100, its apex at 13.39746. In floating point the vertices of an anchored edge are on it within a rounding error, and written as the integer edge (Q10).
- The anchor is an **intention, always stored**: on the axis the polygon fills, the three alignments give the same drawing, and the chosen one is still kept and shown.
- The anchor places the **sharp-cornered** polygon; the corner radius then rounds inwards without moving it: a flat face on the anchored side keeps touching it, a rounded vertex leaves it (a triangle anchored at the top with radius 10 has its top at y = 10). No "always touch" option, which would push a rounded polygon until it touches its box (Product Owner, 2026-10-11, confirmed at `VAL-003`): a polygon is used to attach things to the face that touches its box, and a rounded polygon pushed off its sharp place would put that face, and the measures taken from it, at meaningless positions.
- The grid keeps its **nine places** (Product Owner, 2026-10-11, left to the agent at `VAL-003`). In a given box only one axis has room, so only three places give different drawings. A corner place is not a duplicate: it holds both alignments, and the one that counts follows the proportions of the box. A triangle anchored bottom left sits at the bottom of a 100 × 100 box; a square anchored bottom left sits on the left of a 100 × 50 box. The layout box (Q21) uses the same grid for elements with room on both axes.
- Rotation (F04) comes after: the box turns with its content.

### Round shape (F10)

- A **round shape** fills its box: a circle in a square box, an ellipse otherwise, touching its four sides (Product Owner, 2026-10-12). A capsule stays a rectangle rounded by half its smaller side.
- Model (`RoundShape`, ADR-0048): `width`, `height` (integers ≥ 0, Q15), `angle` (0 to 359), position `x`, `y` (ADR-0035), and `kind: "round"`, which tells it from the shapes that have corners. No radius, no anchor, no point to edit.
- Drawn as four cubic sides through its top, right, bottom and left points, point 1 at the top then clockwise; each handle `4(√2 − 1)/3` of the half axis, in Mirror angle and length (`DERIV-round-shape-curve`). The curve is within about 0.00027 of the larger half axis of the true ellipse: 0.014 on a circle of diameter 100. A box of width or height 0 draws the line it is.
- One conversion to a free shape: **Convert to free shape** gives its 4 points with their handles, the drawing unchanged (Product Owner, 2026-10-12). A box of width or height 0 is not converted, as a polygon (F08). No conversion to a rounded rectangle: rounding its box turns an ellipse into a capsule, which loses the drawing (Product Owner, 2026-10-14, at VAL-012).
- Refined by the agent from the Product Owner's decisions and accepted at VAL-012 (2026-10-14): the cubic sides, point 1 at the top, the name of the conversion.

### Text

Decided by the Product Owner at F05's refinement, rounds 1 and 2 (2026-10-14); drawn and measured as ADR-0052 says.

- **Drawing**: a native `<text>`, the exception to "everything is a path" (ADR-0002), with one positioned `<tspan>` per line, so that every SVG reader shows the same lines. The library measures the text and breaks its lines itself. Text converted to paths is not wanted. An HTML layer for browsers comes on top later (F15).
- **Lines drawn** (US-031, `DERIV-text-width` steps 5 and 6):
  - each line's `x` places it in the block as its alignment says: aligned left, at the block's left edge; centered, half of the room it leaves before it; aligned right, all of it. "Hello" over "World" in Arial 16, centered: "Hello" starts 2.66016 right of the block's edge, "World" at it;
  - left, center and right are read as the physical sides for both directions, as CSS's `left` and `right` are: a right-to-left text aligned left starts each line on the block's left edge (agent, 2026-10-15, from the words of the refinement; to be confirmed by the Product Owner at VAL-014). A right-to-left line is written at its right edge, where SVG draws its start, with `direction="rtl"`;
  - each line's `y` is its baseline; every space typed is drawn, as it is measured (`white-space="pre"` and `xml:space="preserve"`, ADR-0054);
  - the font is written as itself, then its fallback, then `sans-serif`, each name once: `"Inter", sans-serif` (ADR-0054);
  - the opacity is written as a number from 0 to 1 (`fill-opacity`, 40 % gives 0.4);
  - an underline or a strike-through is a rectangle as long as the line, placed and as thick as the font says, in the text's color; a line 0 wide or less (empty, or tightened past nothing) has none, and neither has a stroke the font gives no thickness (0 or negative) (2026-10-15, provisional, to confirm at VAL-014);
  - a line tightened past nothing by a negative spacing keeps its negative width and is placed by the same rule, so it may fall outside the block, which stays at least 0 wide: lines 10 and −8 wide, centered, start 0 and 9 from the block's left edge; a single line −6.89 wide, aligned right, starts 6.89 right of it (2026-10-15, provisional, to confirm at VAL-014);
- **Measuring**: from the font's metrics, read once from the font file and kept; measuring a text is then a sum of stored numbers (`DERIV-text-width`). In a running view a text may change a hundred times a second on a thousand texts: moving, turning, showing, hiding and recoloring stay cheap transforms.
- **Font metrics** (EN-031): the library reads a TrueType or OpenType file (`REF-OT-OTFF`) into plain numbers, in font units:
  - the units per em (`head`), and the ascender, descender and line gap of `hhea`;
  - the advance of each character asked that the font maps to a glyph, read from its Unicode `cmap` subtable of format 12, else 4 (`REF-OT-CMAP`), and from `hmtx`, where a glyph past the last record takes the last record's advance (`REF-OT-HMTX`);
  - the advance of glyph 0, `.notdef`, which a character the font does not map takes;
  - the kerning between those characters: the lookups of the `GPOS` table's `kern` feature, whatever its script, in lookup list order, the first subtable of a lookup that matches a pair giving its value and the lookups adding up (`REF-OT-LAYOUT-COMMON`, `REF-OT-GPOS`); when `GPOS` has no `kern` feature, the horizontal format 0 subtables of the `kern` table, in order, each adding its values to those before it, or replacing them when its override bit is set (`REF-OT-KERN`, as browsers do, `REF-CSS-FONTS-4`). Only the first glyph's `xAdvance` is read, and every pair is adjacent: the lookup flags that skip marks are not applied. The kerning is kept by classes of characters that kern alike, so that a pair's kerning is one look-up;
  - the underline's position and thickness (`post`), the strike-out's (`OS/2`), and the embedding permissions `fsType` (`OS/2`, `REF-OT-OS2`);
  - a file that is not a TrueType or OpenType font (a collection `.ttc` among them), or lacks `cmap`, `head`, `hhea`, `hmtx`, `post` or `OS/2`, gives no metrics;
  - every read is checked against its table: a table too short for what is read in it, or a part of `cmap`, `GPOS` or `kern` pointing past its table's end, gives no metrics, never an error; a `GPOS` or `kern` table too short for its own header is left out (2026-10-14, EN-031 review). Ranges of glyphs are looked up, never spread out, so a range of all 65 536 glyphs costs no more than another.
  - the characters kept are those of the scripts measured (ADR-0052): the Unicode blocks Basic Latin, Latin-1 Supplement, Latin Extended-A and B, Latin Extended Additional, Combining Diacritical Marks (added 2026-10-15, EN-032 review), Greek and Coptic, Cyrillic, Hebrew, General Punctuation, and the symbols industrial labels use: Currency Symbols, Letterlike Symbols, Arrows and Mathematical Operators (€ − → ≤ ™, and ș ț of Romanian), 2080 code points (`REF-UNICODE-BLOCKS`). "Latin" is read as the Latin script and the symbols labels use (agent, 2026-10-14, EN-031 review), to be confirmed by the Product Owner at VAL-014;
  - the default font's metrics ship with the library for Inter 4.1's nine weights, upright and italic; Inter has no Hebrew glyph, so a Hebrew character is measured with Inter's `.notdef` advance until a font with Hebrew is read.
- **Characters**: a text holds:
  - its content, with the line breaks the designer typed, a line break between two lines: a line feed (LF), a carriage return (CR) or both (CR LF), as Unicode's hard line breaks (`REF-UAX14`, LB5; 2026-10-15, provisional, to confirm at VAL-014); any content is allowed, the empty one included, which draws nothing;
  - a font named with a fallback (`Inter`, else `sans-serif`), any font allowed; its name is stored trimmed, with no white space at either end, holds at least one character, and no control character such as a line break or a tab (2026-10-15, provisional, to confirm at VAL-014: `" Inter"` and `"Inter⏎Bold"` are refused, the playground trims what is typed); the default font is Inter, whose metrics ship with the library;
  - a size, an integer from 1 to 100 000, in the one unit used everywhere (no millimetres, no points); a size outside that range or not an integer is refused;
  - a weight, an integer from 100 (thin) to 900 (black), with a quick bold button and an italic button; the bold button is pressed at a weight of 700 or heavier, and a click sets 400 when it is pressed, 700 otherwise: from 800 it gives 400, from 600 it gives 700 (2026-10-15, provisional, to confirm at VAL-014);
  - a color in hexadecimal, `#` and six digits (upper or lower case), with an opacity, an integer from 0 to 100 % (F16 brings the picker);
  - letter spacing and word spacing, integers from −100 000 to 100 000 user units, the size's bound, negative to tighten, to fit text in small frames (bounds of 2026-10-15, provisional, to confirm at VAL-014);
  - underline and strike-through;
  - a case transform, written by the library (Q36, below);
  - a direction, left to right or right to left; a case, an alignment or a direction outside its values, read from a file, is refused;
  - an alignment of its lines (left, center, right), a position (integers, the top-left of its block before it turns) and an angle.
- **Case transforms** (Q36, a provisional decision of 2026-10-15, to be confirmed at VAL-014): each line is written in the case on its own, its line breaks kept. For "pump P-101 is running":

  | Case     | Rule                                                                                                                                                                  | Gives                   |
  | -------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------- |
  | none     | as typed                                                                                                                                                              | `pump P-101 is running` |
  | upper    | every letter in upper case                                                                                                                                            | `PUMP P-101 IS RUNNING` |
  | lower    | every letter in lower case                                                                                                                                            | `pump p-101 is running` |
  | sentence | the line's first letter in upper case, past what is not a letter ("␣pump" gives "␣Pump", "(pump) on" gives "(Pump) on"), the rest kept                                | `Pump P-101 is running` |
  | title    | the first letter of each run of characters between white space (space, no-break space, tab) in upper case, the spaces and the rest kept (`pump  on` gives `Pump  On`) | `Pump P-101 Is Running` |
  | camel    | the words, cut at spaces, hyphens and underscores, in lower case, each but the first capitalized, joined                                                              | `pumpP101IsRunning`     |
  | Pascal   | the same words, each capitalized, joined                                                                                                                              | `PumpP101IsRunning`     |
  | kebab    | the same words in lower case, joined by hyphens                                                                                                                       | `pump-p-101-is-running` |
  | snake    | the same words in lower case, joined by underscores                                                                                                                   | `pump_p_101_is_running` |
  | constant | the same words in upper case, joined by underscores                                                                                                                   | `PUMP_P_101_IS_RUNNING` |

  A run of separators cuts once, so a line of separators only gives an empty line in the programmers' cases.

- **Font face** (EN-032): a text is measured with the face of its font family that CSS would choose (`REF-CSS-FONTS-4`, §5.2): the faces of its style first (italic or upright), the other style when the family has none; then its weight if a face has it, otherwise, for a weight from 400 to 500, the heavier weights up to 500 in ascending order, then the lighter ones in descending order, then those past 500 in ascending order; for a weight under 400, the lighter ones in descending order, then the heavier ones in ascending order; for a weight over 500, the heavier ones in ascending order, then the lighter ones in descending order. Inter 450 is measured with Inter 500; a family of 300 and 700 only measures 400 with 300 and 600 with 700. A family is found by its name without regard to case, as CSS matches family names (`REF-CSS-FONTS-4`, §5.1): "arial" names Arial. A font without metrics is measured with its fallback family's face, when that family's metrics were read, else with the default font's (2026-10-15, provisional, to confirm at VAL-014).
- **Measured block** (EN-032, `DERIV-text-width`): the content, written in its case, is cut into lines at each line break; each line is composed into Normalization Form C (e and a combining acute measured as é) and its invisible formatting characters (Unicode category Cf: zero-width space, zero-width joiner, soft hyphen) are measured as if absent; it is measured from the face's advances and kerning, plus the letter spacing between adjacent typographic character units — a character with the nonspacing marks (category Mn, such as the Hebrew points) after it, so that no spacing comes before a mark — and the word spacing at each space and no-break space (2026-10-15, provisional, to confirm at VAL-014); the block is as wide as the widest line and as tall as the number of lines times the line height. An empty content has no line, a block 0 wide and 0 tall; a block whose lines are all narrower than nothing, tightened by a negative spacing, is 0 wide (2026-10-15, provisional, to confirm at VAL-014); a content ending with a line break ends with an empty line, which counts in the height.
- **Scripts**: the library measures Latin, Greek and Cyrillic, and Hebrew written right to left. Arabic and Indian scripts need shaping: they are left to the browser's HTML layer (F15).
- **Turning**: like a shape, by an integer angle kept from 0 to 359, clockwise, about the middle of its block: of the measured text when it fits its content, of its box otherwise (F14). When shapes get rotation anchors, text gets them too.
  - The middle is the text's position plus half its measured block (`DERIV-turned-shape` step 4): "Hello" over "World" in Arial 16 without kerning, at (0, 0), has a block 41.78125 × 36.796875 and turns about (20.890625, 18.3984375) (US-032).
  - The turn is written as `transform="rotate(α cx cy)"` (`REF-SVG11-TRANSFORM`, §7.6) on the `<g>` that holds the text's `<text>` and its underline and strike-through paths, so that the strokes turn with the letters; the criterion's "a `transform` on its `<text>`" is met through that group (agent, 2026-10-15, provisional, to confirm at VAL-014). A text at 0° carries no `transform`.
  - A selected text shows its measured block as a shape's straight box is shown: dashed by thin filled rectangles, turned with the text about the same middle (US-032); an empty text, whose block has no area, shows none.
- **Empty text**: the library draws an empty text as nothing; a text created by a click and left empty disappears when the designer changes tool or clicks elsewhere (an editor behavior, E02).
- **Later in E15**: a box of text of given width and height, its lines wrapped, a word too long broken at a character or moved whole to the next line, no hyphenation, an ellipsis and a maximum number of lines, a vertical alignment, a line height, a wrap style (balance, pretty), a background color drawn as a path, a vertical trim from the cap height to the baseline (F14); fonts read from files or from the computer, fonts embedded once per view, the HTML layer (F15); colors and their pickers (F16).
- **Not wanted**: lists (a dash typed by hand is enough), highlight shapes beyond a background color, CSS color names, RGB fields.
- **Text styles**: named styles ("title", "body", "description") that predefine a text's options; in the free edition, templates kept locally that forbid nothing (`editions.md` for the pro edition); with the libraries' epic.

### Layout box (Q21)

Settled with the Product Owner (2026-10-10, refining F07): a box that places the elements inside it, as a Figma frame with a layout does, made simple for synoptic views rather than following CSS. Delivered by epic E13, with the shape tree; "layout" is a working name, kept only if no better term is found.

- The box is a **rectangle only**, with a corner radius at most: its vertices cannot be edited, otherwise placing its children becomes too complex.
- Its children are placed **by anchor** (the 3 × 3 grid of F07, top-left … bottom-right), or **stacked** in a row or a column with gaps; the box may have padding. Both modes exist.
- **Any child may override the anchor** and still be laid out by the box: a child may choose the bottom-left anchor where the box places the others top right. This is what Figma lacks: its child in absolute position no longer follows the parent's layout.
- The box does **not resize to its children** (unlike Figma's hug): its size is the one the designer typed.
- Children: shapes, text, later symbols. The polygon in its own box (F07) is its single-child case.

- A child given **its own anchor** stays in the box (same group) but **leaves the stack completely**: the stack follows the box's default layout, the child its own. Overlaps are the designer's business: the option exists, using it well is up to them.
- **Stacks are a drawing aid, fixed at run time**: they live in the Symbol Editor (still there when the symbol is edited again); once the symbol is built, every position is static. An element hidden by an animation leaves its place empty: the stack never closes the gap: only that element, at that place, is hidden.

### Out of scope (symbols)

- **Pen** and **freeform drawing** in the Symbol Editor: contrary to the "logical" principle.
- Bézier curves other than the cubic sides from handles (ADR-0046); Figma's **Bend** tool dragged on the drawing (E02).
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
- Curves drawn freely with a pen are allowed in drawings only (Q12, ADR-0018): a Figma-like pen places points, and dragging a point pulls Bézier handles. Symbols have cubic sides from typed handles only, or the round shape's derived ones (ADR-0046, ADR-0048).
