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

- **Drawing**: a native `<text>`, the exception to "everything is a path" (ADR-0002), with one positioned `<tspan>` per line, so that every SVG reader shows the same lines. The library measures the text and breaks its lines itself. Text converted to paths is not wanted. An HTML layer for browsers comes on top (F15), off by default — the agent's reading, provisional, VAL-017 (ADR-0053, below).
- **Lines drawn** (US-031, `DERIV-text-width` steps 5 and 6):
  - each line's `x` places it in the block as its alignment says: aligned left, at the block's left edge; centered, half of the room it leaves before it; aligned right, all of it. "Hello" over "World" in Arial 16, centered: "Hello" starts 2.66016 right of the block's edge, "World" at it;
  - left, center and right are read as the physical sides for both directions, as CSS's `left` and `right` are: a right-to-left text aligned left starts each line on the block's left edge (agent, 2026-10-15, from the words of the refinement; to be confirmed by the Product Owner at VAL-014). A right-to-left line is written at its right edge, where SVG draws its start, with `direction="rtl"`;
  - each line's `y` is its baseline; every space typed is drawn, as it is measured (`white-space="pre"` and `xml:space="preserve"`, ADR-0054);
  - the font is written as itself, then its fallback, then `sans-serif`, each name once: `"Inter", sans-serif` (ADR-0054);
  - the opacity is written as a number from 0 to 1 (`fill-opacity`, 40 % gives 0.4);
  - an underline or a strike-through is a rectangle as long as the line, placed and as thick as the font says, in the text's color — each in a color and an opacity of its own once US-041 is built (Product Owner, 2026-10-15); the underline stays at the font's standard position (`post`), lower than the Product Owner first expected, kept as built; an underline that skips descenders is not wanted (Product Owner, 2026-10-15); a line 0 wide or less (empty, or tightened past nothing) has none, and neither has a stroke the font gives no thickness (0 or negative) (2026-10-15, provisional, to confirm at VAL-014);
  - a line tightened past nothing by a negative spacing keeps its negative width and is placed by the same rule, so it may fall outside the block, which stays at least 0 wide: lines 10 and −8 wide, centered, start 0 and 9 from the block's left edge; a single line −6.89 wide, aligned right, starts 6.89 right of it (2026-10-15, provisional, to confirm at VAL-014);
- **Measuring**: from the font's metrics, read once from the font file and kept; measuring a text is then a sum of stored numbers (`DERIV-text-width`). In a running view a text may change a hundred times a second on a thousand texts: moving, turning, showing, hiding and recoloring stay cheap transforms.
- **Font metrics** (EN-031): the library reads a TrueType or OpenType file (`REF-OT-OTFF`) into plain numbers, in font units:
  - the units per em (`head`), and the ascender, descender and line gap of `hhea`;
  - the advance of each character asked that the font maps to a glyph, read from its Unicode `cmap` subtable of format 12, else 4 (`REF-OT-CMAP`), and from `hmtx`, where a glyph past the last record takes the last record's advance (`REF-OT-HMTX`);
  - the advance of glyph 0, `.notdef`, which a character the font does not map takes;
  - the kerning between those characters: the lookups of the `GPOS` table's `kern` feature, whatever its script, in lookup list order, the first subtable of a lookup that matches a pair giving its value and the lookups adding up (`REF-OT-LAYOUT-COMMON`, `REF-OT-GPOS`); when `GPOS` has no `kern` feature, the horizontal format 0 subtables of the `kern` table, in order, each adding its values to those before it, or replacing them when its override bit is set (`REF-OT-KERN`, as browsers do, `REF-CSS-FONTS-4`). Only the first glyph's `xAdvance` is read, and every pair is adjacent: the lookup flags that skip marks are not applied. The kerning is kept by classes of characters that kern alike, so that a pair's kerning is one look-up;
  - the underline's position and thickness (`post`), the strike-out's (`OS/2`), and the embedding permissions `fsType` (`OS/2`, `REF-OT-OS2`);
  - the cap height, `sCapHeight` of `OS/2` from version 2 of the table; a table older than version 2, too short to hold it, or giving 0 or less (a font without `H`) gives 0.66 em, the fallback of CSS (`REF-CSS-INLINE-3`, A.2): Arial's is 1467 units, its fallback would be 1351.68 for its 2048 units per em; Inter's is 1490 (EN-035);
  - a file that is not a TrueType or OpenType font (a collection `.ttc` among them), or lacks `cmap`, `head`, `hhea`, `hmtx`, `post` or `OS/2`, gives no metrics;
  - every read is checked against its table: a table too short for what is read in it, or a part of `cmap`, `GPOS` or `kern` pointing past its table's end, gives no metrics, never an error; a `GPOS` or `kern` table too short for its own header is left out (2026-10-14, EN-031 review). Ranges of glyphs are looked up, never spread out, so a range of all 65 536 glyphs costs no more than another;
  - a font whose units per em lie outside 16 to 16 384 (`REF-OT-HEAD`) or whose `hhea` announces no `hmtx` record, a `kern` subtable shorter than its own header or more `kern` subtables than the table can hold, gives no metrics; a `GPOS` class pair subtable whose records hold no value gives each pair of classes no kerning, without listing them (agent, 2026-10-15, AUD-015).
  - the characters kept are those of the scripts measured (ADR-0052): the Unicode blocks Basic Latin, Latin-1 Supplement, Latin Extended-A and B, Latin Extended Additional, Combining Diacritical Marks (added 2026-10-15, EN-032 review), Greek and Coptic, Cyrillic, Hebrew, General Punctuation, and the symbols industrial labels use: Currency Symbols, Letterlike Symbols, Arrows and Mathematical Operators (€ − → ≤ ™, and ș ț of Romanian), 2080 code points (`REF-UNICODE-BLOCKS`). "Latin" is read as the Latin script and the symbols labels use (agent, 2026-10-14, EN-031 review), to be confirmed by the Product Owner at VAL-014;
  - the default font's metrics ship with the library for Inter 4.1's nine weights, upright and italic; Inter has no Hebrew glyph, so a Hebrew character is measured with Inter's `.notdef` advance until a font with Hebrew is read.
- **Characters**: a text holds:
  - its content, with the line breaks the designer typed, a line break between two lines: a line feed (LF), a carriage return (CR) or both (CR LF), as Unicode's hard line breaks (`REF-UAX14`, LB5; 2026-10-15, provisional, to confirm at VAL-014); any content is allowed, the empty one included, which draws nothing;
  - a font named with a fallback (`Inter`, else `sans-serif`), any font allowed; its name is stored trimmed, with no white space at either end, holds at least one character, and no control character such as a line break or a tab (2026-10-15, provisional, to confirm at VAL-014: `" Inter"` and `"Inter⏎Bold"` are refused, the playground trims what is typed); the fallback's name follows the same rule (US-032, from CHK-014); the default font is Inter, whose metrics ship with the library;
  - a size, an integer from 1 to 100 000, in the one unit used everywhere (no millimetres, no points); a size outside that range or not an integer is refused;
  - a weight, an integer from 100 (thin) to 900 (black), with a quick bold button and an italic button; the bold button gives back the weight held before it was pressed (Product Owner, 2026-10-15, VAL-014's decision 3): from Thin (100), Bold gives 700 and Bold again gives 100, not 400; from 800, Bold gives 700 and Bold again 800. It shows pressed at 700 and while it holds a weight; a weight chosen in the list, a family chosen that changes the weight, or a guided step opened, forgets the weight held, and Bold at 700 with no weight held gives 400 (agent's reading, to confirm at VAL-014; US-037);
  - a color in hexadecimal, `#` and six lower-case digits (typed in another form or in upper case, it is turned to that form first, and a color written in upper case before F16 is turned to lower case when read, `interaction.md` §5; provisional, VAL-016), with an opacity, an integer from 0 to 100 %, picked or typed as `interaction.md` §5 says (F16, ADR-0056);
  - letter spacing and word spacing, integers from −100 000 to 100 000 user units, the size's bound, negative to tighten, to fit text in small frames (bounds of 2026-10-15, provisional, to confirm at VAL-014);
  - underline and strike-through;
  - a case transform, written by the library (Q36, below);
  - a direction, left to right or right to left; a case, an alignment or a direction outside its values, read from a file, is refused;
  - an alignment of its lines (left, center, right), a position (the top-left of its block before it turns: integers when typed, finite and possibly fractional once a turned text has grown, ADR-0057) and an angle.
- **Case transforms** (Q36, the agent's proposal of 2026-10-15; kept by the Product Owner on 2026-10-15, sentence case removed as of no use, US-037): each line is written in the case on its own, its line breaks kept; a text whose case is not one of these, `sentence` among them, is refused. For "pump P-101 is running":

  | Case     | Rule                                                                                                                                                                  | Gives                   |
  | -------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------- |
  | none     | as typed                                                                                                                                                              | `pump P-101 is running` |
  | upper    | every letter in upper case                                                                                                                                            | `PUMP P-101 IS RUNNING` |
  | lower    | every letter in lower case                                                                                                                                            | `pump p-101 is running` |
  | title    | the first letter of each run of characters between white space (space, no-break space, tab) in upper case, the spaces and the rest kept (`pump  on` gives `Pump  On`) | `Pump P-101 Is Running` |
  | camel    | the words, cut at spaces, hyphens and underscores, in lower case, each but the first capitalized, joined                                                              | `pumpP101IsRunning`     |
  | Pascal   | the same words, each capitalized, joined                                                                                                                              | `PumpP101IsRunning`     |
  | kebab    | the same words in lower case, joined by hyphens                                                                                                                       | `pump-p-101-is-running` |
  | snake    | the same words in lower case, joined by underscores                                                                                                                   | `pump_p_101_is_running` |
  | constant | the same words in upper case, joined by underscores                                                                                                                   | `PUMP_P_101_IS_RUNNING` |

  A run of separators cuts once, so a line of separators only gives an empty line in the programmers' cases.

- **Font face** (EN-032): a text is measured with the face of its font family that CSS would choose (`REF-CSS-FONTS-4`, §5.2): the faces of its style first (italic or upright), the other style when the family has none; then its weight if a face has it, otherwise, for a weight from 400 to 500, the heavier weights up to 500 in ascending order, then the lighter ones in descending order, then those past 500 in ascending order; for a weight under 400, the lighter ones in descending order, then the heavier ones in ascending order; for a weight over 500, the heavier ones in ascending order, then the lighter ones in descending order. Inter 450 is measured with Inter 500; a family of 300 and 700 only measures 400 with 300 and 600 with 700. A family is found by its name without regard to case, as CSS matches family names (`REF-CSS-FONTS-4`, §5.1): "arial" names Arial. A font without metrics is measured with its fallback family's face, when that family's metrics were read, else with the default font's (ADR-0055; 2026-10-15, provisional, to confirm at VAL-014).
- **Measured block** (EN-032, `DERIV-text-width`): the content, written in its case, is cut into lines at each line break; each line is composed into Normalization Form C (e and a combining acute measured as é) and its invisible formatting characters (Unicode category Cf: zero-width space, zero-width joiner, soft hyphen) are measured as if absent; it is measured from the face's advances and kerning, plus the letter spacing between adjacent typographic character units — a character with the nonspacing marks (category Mn, such as the Hebrew points) after it, so that no spacing comes before a mark — and the word spacing at each space and no-break space (2026-10-15, provisional, to confirm at VAL-014); the block is as wide as the widest line and as tall as the number of lines times the line height, less what a trimmed first line gives up above its cap height (EN-035). An empty content has no line, a block 0 wide and 0 tall; a block whose lines are all narrower than nothing, tightened by a negative spacing, is 0 wide (2026-10-15, provisional, to confirm at VAL-014); a content ending with a line break ends with an empty line, which counts in the height. A text in a box is measured and drawn with the lines its box shows, their ellipses included (EN-034).
- **Box** (F14, `DERIV-text-box`; decided at F05's refinement, rounds 1 and 2; the rules marked "provisional" were read from those decisions by the agent at SP-016, 2026-10-15, to confirm at VAL-015):
  - a text **fits its content**, as in F05, or **sits in a box** of a given width and height, integers from 0 to 100 000 (provisional, the bounds of a size and Q15);
  - in a box, each paragraph (the text between two line breaks typed) wraps **at spaces only**: after the last space of a run, never at a no-break space, never at a hyphen; the spaces where a line wraps are neither measured nor drawn; lines are filled as much as they can be (greedy). "Hello World" in Arial 16 in a box 50 wide gives "Hello" over "World". The spaces that open a paragraph stay with its first word, so that no line is made of them alone, although UAX #14 (LB18) would allow a break after them (agent, 2026-10-15, EN-033 audit, a design choice, provisional, VAL-015);
  - a **word too long** for the box, as the designer chooses: **broken at a character** — it starts its own line, then breaks after as many characters as fit, no hyphen ("running", 53.4 wide, in a box 50 wide gives "runnin" over "g") — or **moved whole** — it stays whole on its own line, cut with an ellipsis when the ellipsis is on ("runn…"), drawn whole and overflowing the box when it is off ("running", 53.4 wide) (provisional reading of both options, VAL-015). Broken, a word's parts each keep at least one character, a character never parted from its accents, the spaces after the word stay with its last part, and the next words may join that part ("Hello is World" in a box 30 wide gives "Hell", "o is", "Wor", "ld"); a part ends at the first character that does not fit, which is the longest run that fits unless a negative spacing narrows a longer run again (agent, 2026-10-15, EN-033, provisional, VAL-015). A word is never broken right after a no-break space, in either option: the no-break space goes with the character after it, so that no part ends with one — "Hello", a no-break space and "World" in a box 45 wide give "Hello", then the no-break space and "Worl", then "d"; a no-break space ending a word, with no character after it, is measured as a character of its own (2026-10-15, EN-033 audit, provisional, VAL-015); only U+00A0 is held so: the narrow no-break space U+202F, the figure space U+2007 and the word joiner U+2060 may end a part (2026-10-15, AUD-016, provisional, VAL-015);
  - the lines shown are the most that fit in the box's height, at most the **maximum number of lines** when one is set (an integer, 0 for none, up to 100 000), and **at least one**, even in a box shorter than a line (provisional, VAL-015), but never more than the lines the text wraps into: "Hello" over "World" in a box 60 tall shows 2 lines, though 3 would fit; an empty text shows none and draws nothing;
  - the **ellipsis** is an option, on unless the designer turns it off (F05's refinement: a text too long "can end with an ellipsis"; on by default, provisional, VAL-015). Off, a cut text simply stops at the last line shown: "Hello World" in a box 60 wide with at most 1 line shows "Hello". On, a text **cut** ends its last line shown with an ellipsis "…" (U+2026), and a line wider than the box ends with one too, a word moved whole among them, when at least one character can be removed: a line of one character wider than the box keeps no ellipsis, the first character of a line being "clipped rather than ellipsed" in CSS ("Hello" broken at a character in a box 5 wide shows "H", "e", "l", "l", "o") (2026-10-15, EN-034 audit, provisional, VAL-015); empty lines left out, as after a content ending with a line break, add no ellipsis ("Hello" and a line break with at most 1 line shows "Hello") (2026-10-15, EN-034 audit, provisional, VAL-015); characters are removed from the end of the line, with the spaces before them, until the line and its ellipsis fit, the first character other than a space always kept, and never a cut right after a no-break space, which stays with the character after it ("Hello", a no-break space and "World" in a box 70 wide shows "Hello…") (2026-10-15, EN-034 audit, provisional, VAL-015); a font without "…" gets three full stops. "Hello World" in a box 60 wide with at most 1 line shows "Hello…" (52.46 wide), in a box 50 wide "Hell…"; "Hello World Hello World" in a box 60 wide with at most 2 lines shows "Hello" over "World…" (57.78). The ellipsis removes characters rather than whole words (provisional, VAL-015). When even the first character and the ellipsis do not fit, both are drawn, past the box's edge: "Hello World" in a box 10 wide with at most 1 line shows "H…", 27.6 wide. CSS would clip the ellipsis itself, but clipping needs a primitive the project does not allow (ADR-0052), so this departs from CSS (provisional, VAL-015; the alternatives: draw only what fits, or nothing);
  - the lines are aligned **across** the box's width (left, center, right, as in F05) and the block of lines **down** the box: top, middle or bottom, as a line is placed across ("Hello" over "World" in a box 60 tall, middle: 11.6 of room above);
  - **line height**: automatic, the font's (F05), or an integer from 1 to 100 000 in user units, 0 meaning automatic (provisional); a chosen line height spreads the room it leaves around the letters half above, half below (CSS's half-leading), while the automatic one keeps F05's placement, its line gap below the line (provisional, so that every F05 text draws as before);
  - **trim**: the block starts at the cap height of its first line and ends at the baseline of its last, so that texts of different sizes line up on where their capitals start; the cap height is the font's `sCapHeight`, or 0.66 of the size when the font gives none;
  - **wrap style**: automatic (greedy) or **balanced**, which keeps the number of lines and evens their lengths, the sum of the squares of the room each line leaves made the least ("Pump P-101 is running" in a box 110 wide: "Pump P-101" over "is running" rather than "Pump P-101 is" over "running"): the least among cuts whose lines grow while they fit; the full minimum when widths grow with each token (provisional, VAL-015); a paragraph holding a word too long, of more than 10 automatic lines or of more than 500 words, and a text cut by its box or its maximum number of lines, the ellipsis on or off, keep the automatic lines (provisional, VAL-015; CSS balances the lines left after the cut instead); on equal evenness the later break is kept ("Hello World Hello" in a box 90: "Hello World" over "Hello"); the "pretty" style named at the refinement is not built in F14, balance only (provisional, VAL-015: keep it for later, or drop it);
  - wrapping, the maximum number of lines, the ellipsis option, the vertical alignment and the wrap style apply in a box only; line height, trim and background apply to both kinds of text (provisional, VAL-015);
  - **defaults** (EN-033): a text fits its content, its line height is automatic, it is not trimmed, it has no background and its ellipsis is on, so that every text of F05 draws as before; the options read only in a box start as a box 0 × 0, a word too long broken at a character, no maximum number of lines, the block at the top, the automatic wrap; a background turned on starts yellow (`#ffff00`) and opaque, as a highlighter (agent, 2026-10-15, provisional, to confirm at VAL-015). Every option is checked whichever kind the text is, so that it stays valid when it changes kind;
  - a text in a box turns about the middle of its box (a box 60 × 40 at (0, 0) turns about (30, 20)), and its selection shows the box (Product Owner, 2026-10-15, ADR-0057); a box resized while turned keeps its turned top-left corner in place, as a shape does;
  - **background**: a color in hexadecimal with an opacity, as the text's (F16, the agent's reading, provisional, VAL-016), drawn behind each line as a highlighter, from the font's ascender to its descender and as long as the line with its ellipsis, all lines in one `<path>` drawn under the letters; an empty line has none. A line height chosen or a trim leaves each band from ascender to descender: the bands leave a gap under a line height larger than the content area and overlap under a smaller one, the overlap filled once since the bands are one path filled by the nonzero rule (CSS paints each line's background on its own), and a trimmed text's first band rises above the top of its block, or of its box (provisional, VAL-015);
  - scrolling a long text inside its box on a touch screen comes with the synoptic views (E08), through the HTML layer (F15).
- **Scripts**: the library measures Latin, Greek and Cyrillic, and Hebrew written right to left. Arabic and Indic scripts need shaping: they are left to the browser's HTML layer (F15).
- **Turning**: like a shape, by an integer angle kept from 0 to 359, clockwise, about the middle of its block: of the measured text when it fits its content, of its box otherwise (F14). When shapes get rotation anchors, text gets them too.
  - The middle is the text's position plus half its measured block (`DERIV-turned-shape` step 4): "Hello" over "World" in Arial 16 without kerning, at (0, 0), has a block 41.78125 × 36.796875 and turns about (20.890625, 18.3984375) (US-032).
  - **A turned text that grows or shrinks keeps its turned top-left corner in place** on the drawing (Product Owner, 2026-10-15, ADR-0057; EN-042): whatever changes its block's size — the content typed, a letter or word spacing, the size, more lines — its width grows along the text's direction and its height with its lines, and the stored position is computed again so that the corner does not move, as a width typed on a turned shape does (`DERIV-turned-shape` step 5). A block 100 × 20 at (47, 61) turned by 23 has its turned top-left corner at (54.88207, 42.25840); grown to 120 wide, its position becomes (46.20505, 64.90731) and the corner stays at (54.88207, 42.25840), where turning about the new middle alone would move it to (55.67702, 38.35108). The position may then be fractional, as a turned shape's is (ADR-0035). X and Y show it at 5 decimals, the exact position kept behind them; a position typed is an integer (ADR-0003, ADR-0057), a fraction typed refused, the value shown accepted as it is; a position typed, or an angle changed, is drawn where it is (US-038).
  - The turn is written as `transform="rotate(α cx cy)"` (`REF-SVG11-TRANSFORM`, §7.6) on the `<g>` that holds the text's `<text>` and its underline and strike-through paths, so that the strokes turn with the letters; the criterion's "a `transform` on its `<text>`" is met through that group (agent, 2026-10-15, provisional, to confirm at VAL-014). A text at 0° carries no `transform`.
  - A selected text shows its measured block, or its box when it sits in one (US-033), as a shape's straight box is shown: dashed by thin filled rectangles, turned with the text about the same point (US-032); an empty text, whose block has no area, and a box 0 wide or 0 tall show none.
  - A turned text also shows its **straight box**, as a shape does: the smallest upright rectangle around its turned block or box, with its x, y, width and height, as SVG's `getBBox` gives for its group (Product Owner, 2026-10-15; US-039). Around the block or the box, not the letters' outlines, which the library does not read (agent's reading, to confirm at VAL-015): a block 40 × 20 at (0, 0) turned by 45 about its middle (20, 10) has a straight box at (−1.21320, −11.21320), 42.42641 × 42.42641. Its x, y, width and height are written under the position fields, which keep the top-left of the block or the box before it turns (US-032); a text at 0° shows its frame only, its straight box being the frame itself, and an empty text or a box 0 wide or 0 tall shows none (agent's reading, to confirm at VAL-015).
- **Empty text**: the library draws an empty text as nothing; a text created by a click and left empty disappears when the designer changes tool or clicks elsewhere (an editor behavior, E02).
- **Fonts from files** (F15, decided at F05's refinement, round 2; the rules marked "provisional" were read from it by the agent at SP-018, 2026-10-15, to confirm at VAL-017; research note 0020):
  - the designer imports a TrueType or OpenType file (`.ttf`, `.otf`), or, in Chrome and Edge, chooses a font installed on the computer after a click and the browser's permission (`REF-WICG-LOCAL-FONT-ACCESS`); both are read by the same reader, from the file's bytes, once, into the metrics F05 reads, and kept with the file (provisional: kept for the session; across sessions with the editor's files, E02);
  - the editor's font field is a **list of the families it can measure**, not a name typed: the default Inter, the fonts imported and the installed fonts read with the browser's permission; the weight list shows the faces of the family chosen only; the size of a text is computed with the metrics of the font selected, never with Inter's alone (Product Owner, 2026-10-15, US-036). A font typed by name without metrics, such as `monospace`, was measured with Inter while the browser drew another font, so its block and its trim were wrong;
  - a face is named by its file: its family is the typographic family name (name ID 16), else the family name (name ID 1), read from the Unicode strings of the file — Windows encodings 1 and 10 and the Unicode platform in UTF-16BE, Macintosh Roman when its bytes are ASCII; Windows code pages (encodings 3, 4, 5) are not read —, English first: Windows English (United States), then Macintosh English, then a Unicode platform name, then any other language (`REF-MS-LCID`, `REF-APPLE-NAME`); its weight is `usWeightClass`, brought into 100 to 900 (50 gives 100, 950 gives 900, 450 stays 450); it is italic when `fsSelection` bit 0 is set (`REF-OT-NAME`, `REF-OT-OS2`; provisional). A face read again, same family (without regard to case), weight and style, replaces the one kept, its family keeping the name it was first listed under; a family read for the first time is listed after those kept (provisional). A text in `arial` 700 is then measured with the face of Arial 700 read from its file (EN-039);
  - each face read keeps its file, the version its file names (name ID 5, such as `Version 5.01x`; empty when it names none) and whether its `fsType` permits embedding it (`DERIV-font-embedding`, step 3); a font file that names no family, or whose `OS/2` table is too short to give its style, is refused as a file that is not a font (EN-039);
  - a file that is not a TrueType or OpenType font, a collection or a WOFF among them, is refused, as in F05.
  - **Weights of the family chosen** (US-036): the weight list offers the weights of the chosen family's faces, from the lightest, each with its common name when it has one (`700 Bold`, `450` alone); a family chosen keeps the weight held when it has a face of it (Arial at 700 to Inter stays 700), else the weight of the face the Font face rule matches is chosen and shown (Inter 300 to a family of 400 and 700 gives 400, Inter 600 gives 700). The bold button asks for 700, or gives back the weight held (Characters, US-037), among those faces, the nearest face chosen the same way; it holds the weight before only when the face it reaches differs from it: on a family of 300 and 600, from 300 Bold gives 600, pressed, and Bold again 300; on a family of 400 alone, Bold keeps 400, released, and holds nothing (the agent's reading, provisional, VAL-017);
  - **Installed fonts** (US-036, the agent's reading, provisional, VAL-017): a click lists the installed families after the browser's `local-fonts` permission (`REF-WICG-LOCAL-FONT-ACCESS`), after the families kept in the font list, without reading their files (`Installed families listed: 312. Choose one in the font list.`); a family chosen has the files of its faces read then, once, as imported ones are, and is chosen under the name its files give (`Arial: 4 faces read.`), each face read on its own, a face whose file the browser cannot give or that is not a font, a collection among them, refused and counted, the others kept (`Arial: 3 faces read, 1 refused (…)`). While a family chosen is read, the list shows it chosen and the text keeps the family chosen before it, never Inter by default; a family none of whose faces is read gives the list back to that family, said why (Product Owner's decision at the US-036 audit, 2026-10-15). Where the browser cannot list them, Firefox and Safari, the page says so and points to **Import a font**; a permission refused is said too;
  - **Import a font** (US-036, the agent's reading, provisional, VAL-017): the designer chooses one or more `.ttf` or `.otf` files; each face read is said (`Arial 700 imported.`), and a file refused is named with its reason (`notes.ttf is not a TrueType or OpenType font file (.ttf, .otf): refused.`), each file on its own, a file the browser cannot read said so and the others kept; the family of the last face read is chosen in the font list, so that the text is measured and drawn with it. The browser is given the face's file too, so that a font imported but not installed is drawn with its own outlines;
- **Embedded fonts** (F15, ADR-0053; provisional): an option of a symbol, off unless turned on; once in a synoptic view for all its symbols comes with the views (E08). Each face its texts are measured with is written once, as an `@font-face` rule with the whole file as a base64 `data:` URL in one `<style>` of the `<defs>`: a symbol of 10 texts in Arial regular and bold writes 2 rules (`DERIV-font-embedding`). A face is embedded only when its `fsType` permits it: not when only bitmaps may be embedded, nor when restricted without the preview-and-print or editable bit (Arial, 8, and Inter, 0, are embeddable; 2 is not); a face refused, or whose file the editor does not hold (Inter, whose metrics ship without its files, until its file is imported), is left out and the editor says so: the library lists each face left out with its reason — no file, not permitted, or, below, family incomplete, generic name, not named. The face embedded is the one a text is measured with — of its font, else of its fallback; a face of the default font is embedded only when a text names it — under its family's name as read, escaped as the text's `font-family` is (`My "Font"` gives `"My \"Font\""`); two texts share a face when the faces they are measured with are the same (Arial 600 and 700 are both measured with Arial's 700 face when it holds 400 and 700). The rules come in the order of each face's first use; each is `@font-face { font-family: "Arial"; font-style: italic; font-weight: 700; src: url(data:font/ttf;base64,…) format("truetype"); }`, `normal` for an upright face, `font/otf` and `"opentype"` for a file with CFF outlines (`OTTO`), never `local()`. With the option off, or no face to embed, the document has no `<style>`. Provisional (EN-040 audit, to confirm at VAL-017): a family is embedded whole or not at all — when one face of it that the texts use may not be embedded, every used face of it is left out (family incomplete), since an `@font-face` family hides the installed family of that name (`REF-CSS-FONTS-4`, §5.2, §10.2): Inter 400 held and 700 not embeds no Inter; a family named like a generic family (`Monospace`) is never embedded (generic name); only a face of a family a text names, as its font or its fallback, is embedded — a face reached only as the default font, Inter for a text in Helvetica, is left out (not named). The editor never measures nor edits with a font embedded in a file.
  - **In the playground** (US-036, the agent's reading, provisional, VAL-017): the drawing stands for a document, its options **Embed fonts** and **HTML layer** applying to all its texts, both off as the page opens; each face left out is listed with its reason (`Inter 400: not embedded, its file not held.`; the others: its licence (fsType) does not permit it, another face of its family used may not be embedded, its family is named like a generic family, reached only as the default font); **Export SVG** saves the drawing as `drawing.svg`, its `<defs>` and its text without the selection's dashed frame.
- **HTML layer** (F15, ADR-0053; provisional): an option of a symbol, off unless turned on. Each text is then written as a `switch`: first a `foreignObject` with `requiredExtensions="http://www.w3.org/1999/xhtml"`, which browsers draw, holding the text's content in XHTML with the CSS of its options, laid out by the browser with its own fonts (Arabic and Indic scripts among them); then the text's group as F05 and F14 write it, without its `transform`, which resvg, Batik and Inkscape draw; an outer group carries the turn and holds the `switch`. With the layer off, the text is written exactly as before. The layer's line height is the library's, and the layer is placed so that its first baseline meets the library's: for an automatic line height, half the font's line gap higher (Arial 16: 0.26171875; trimmed, 3.28515625; `DERIV-html-layer`). This holds when the browser uses the same ascender and descender as the library: on macOS, and elsewhere when `USE_TYPO_METRICS` is unset with `win` equal to `hhea`, or set with the typographic values equal to `hhea` — Arial 5.01 and Inter 4.1 both do. Its lines may still differ from the library's where the browser's font or wrapping differs; the library's block stays the text's block.
  - **Written** (EN-041): the `foreignObject` carries `overflow="visible"`, so that lines past it are drawn as the library draws them, and holds a `div` as large as it, which in a box places the block down the box by its vertical alignment (a flex column packed at its start, or `unsafe` at its center or end, so that a block taller than the box overflows both edges in the middle and the top at the bottom, as the library's does; the content's `div` never shrinks, `flex-shrink: 0`), holding the `div` of the content, one `span` per paragraph with a line feed between two; a content ending with a line break ends with a paragraph holding a zero-width space (U+200B, which the library measures as absent), since a browser lays out no line after a final line feed (`"Hello\n"`: 2 lines in both); a layer 0 wide or 0 tall (a box 80 × 0 or 0 × 40, an empty text) is not written, SVG not rendering it, and the text's group stands alone as with the layer off; the layer's numbers are written at 5 decimals, as every SVG number (Q10): Arial 16's line height 18.3984375 is written `18.39844px` (the agent's reading, provisional, VAL-017).
  - **The content's CSS** (EN-041): the font list as the `<text>`'s, its size in pixels (a user unit), weight and style; its color with its opacity as one eight-digit color (`#00000066` for black at 40 %), so that a background's opacity is not multiplied by the text's; the library's line height; letter and word spacing when not 0; `direction: rtl` right to left; `text-align` as the alignment; underline and strike-through as `text-decoration-line` with `text-decoration-thickness: from-font`; `white-space: pre`, or in a box `pre-wrap` with `overflow-wrap: anywhere` for a word broken at a character and `normal` for one moved whole, and `text-wrap: balance` for the balanced wrap; with a background, each paragraph's `span` has its color with its opacity (provisional, VAL-017).
  - **Cut** (EN-041, the agent's reading, provisional, VAL-017): in a box with the ellipsis on, `overflow: hidden` and `text-overflow: ellipsis` for a line wider than the box, and, when the box shows at most `n` lines — its height's and its maximum's, as the library counts them, at least one — `-webkit-line-clamp: n` with `display: -webkit-box` and `-webkit-box-orient: vertical`: a box 50 × 40 in Arial 16 shows at most 2; with the ellipsis off, the block is cut down only at `n` line heights (`max-height`, `overflow-y: clip`), left visible across (`overflow-x: visible`), so that the browser shows no more lines than the library and a word moved whole overflows sideways, as the library draws it; with the ellipsis on, `overflow: hidden` clips both ways, as `text-overflow` needs. A text that fits its content is never cut, and nothing of the layer is clipped by its `foreignObject`.
  - **A trimmed text in a box** (EN-041, `DERIV-html-layer`, in general): the browser trims nothing, so its untrimmed block is placed down the box by the same alignment and the layer moved until both first baselines meet: at the top, raised by 3.28515625 in Arial 16; at the bottom, lowered by 3.65234375, both last baselines then on the box's bottom (provisional, VAL-017).
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
