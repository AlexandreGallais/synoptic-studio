# Shapes

The shapes of a symbol: their model, rotation, corner radius, creation tools, text, booleans, strokes and static drawings.

## 1. Shape model

- **What is stored**: typed numbers make an integer model; the geometry is computed from it and written as a `<path>`; nothing derived is written back. A free shape whose points were typed (0, 0), (100, 0), (100, 50) draws its path from them.
- **Contours**: a shape is one or more closed contours, the first the outside, the next ones holes. A ring is a circle with a smaller circle as its hole.
- **Vertices**: a contour is an ordered list of vertices; a free shape stores them, integers when typed, fractional as a conversion leaves them; a regular polygon's are computed. A hexagon's vertex at (75, 93.30127) is computed, never stored.
- **One radius per vertex**: each vertex carries an integer corner radius, 0 for a sharp corner; a shape with more or fewer radii than vertices is refused. A triangle with 4 radii is refused.
- **One path**: a shape is drawn as a single `<path>`, made of segments, circular arcs and cubic sides from handles. A rounded rectangle is 4 segments and 4 arcs.
- **Determined**: every shape is described by numbers, never drawn by hand. A rectangle is its width, height, angle, position and radii.

### Contour orientation

- **Clockwise from the top-left**: a contour is listed clockwise on screen (y down), starting at its top-left vertex. A rectangle 100 × 50 at (0, 0) is (0, 0), (100, 0), (100, 50), (0, 50).
- **No single top-left**: the contour starts at the topmost vertex, the leftmost on a tie. A triangle pointing up starts at its apex.
- **A regular polygon** starts at the right end of its flat base, then its left end, so that the base keeps points 1 and 2 whatever its number of corners. A square of side 100 at (0, 0) starts at (100, 100), then (0, 100).
- **Numbers never change**: point 1 is the first point of the path, the others follow; moving or turning a shape never renumbers them, only adding or deleting a point shifts the numbers after it by one. Point 3 deleted from a pentagon makes old point 4 the new point 3.
- **Cyclic**: the last vertex joins the first, and every per-vertex computation wraps around. In a square, vertex 1 sees the edges to vertex 4 and to vertex 2.
- **A rounded path** starts where the first corner's arc ends and closes with that arc. A square of side 100 with radius 10 starts at (10, 0).

### Size

- **No upper limit** in the model nor the calculations: a size is any safe integer ≥ 0, up to 2⁵³ − 1. A width of 3 000 000 is valid.
- **Capped in the interface**: a width, height or radius typed above 100 000 is taken at 100 000. 250 000 gives 100 000.
- **One unit, one pixel**: a view is drawn at the size of its screen and the SVG scaled for a screen seen from far. Four 8K screens side by side are 30 720 units wide, far below 100 000.
- **Absolute tolerance**: rounding errors are about 1e-16 × the size, under `EPSILON` = 1e-9 up to a few million units. At 100 000 the error is about 1e-11.

### Precision

- **Five decimals**: a derived coordinate is written with at most 5 decimals, in its shortest form, never as a negative zero. 2.50000 is written `2.5`, −0.000001 is written `0`.
- **Tolerance**: floating-point comparisons use `EPSILON` = 1e-9. 0.1 + 0.2 equals 0.3 within it.
- **Integer input**: the user only ever types integers; decimals appear in derived geometry only. A width typed 100 is stored 100.

### Rotation of shapes

- **Any integer angle**: a shape of the Symbol Editor turns by any integer number of degrees; the angle is stored, the turned vertices computed. A rectangle tilted by 45° stores 45.
- **About its box's center**: a shape turns about the middle of its box; a single point never turns. A box 100 × 50 at (10, 20) turns about (60, 45).
- **Clockwise, 0 to 359**: a positive angle turns clockwise on screen, kept from 0 to 359. 370 gives 10, −90 gives 270.
- **The shape is kept**: turning keeps its width, height, radii, anchor and point numbers; back to 0 it is straight again; a turned polygon converted to a free shape keeps its angle. A rectangle 100 × 50 turned by 30° still reads W 100, H 50.
- **Points of a turned shape**: in Points mode a point's x and y are those of the turned point in the SVG. Point 1 of a square of side 100 at (0, 0) turned by 90° reads (100, 0).
- **Two boxes**: the turned box is the shape's box turned with it, shown as its selection; the straight box is the smallest upright rectangle around the drawn shape, rounded corners included, shown with its x, y, width and height. A square of side 100 turned by 45° has a turned box 100 × 100 and a straight box 141.42136 × 141.42136.
- **Position**: shown and typed as the top-left of the straight box; stored as the top-left of the box before it turns, so that a radius changed never moves a turned shape. That square at (0, 0) turned by 45° shows X −20.71068, Y −20.71068.
- **A shape 0 × 0** is placed by the point its corners lie on. 0 × 0 at (10, 20) shows X 10, Y 20; 50 typed in X puts it at x 50.
- **A width typed on a turned shape** keeps its turned top-left corner in place; a point typed, added or deleted leaves the others where they are drawn. A sharp 100 × 50 at (10, 20) turned by 30°, made 150 wide, keeps its corner at (29.19873, −1.65064), its position becoming (6.65064, 32.5).
- **The view stays put**: the canvas does not follow a turned shape, which may leave it. A shape turned near the canvas's edge may be cut by it.
- **Instances** in a view turn by quarter turns only (`symbols-and-views.md`). An instance turns by 90°, never 45°.

## 2. Corner radius

- **Typed, never dragged**: a radius is entered as a number. 12 typed rounds the corner by 12.
- **Per vertex**: one or more vertices selected in Points mode take the radius typed. Vertices 1 and 3 selected, 20 typed, round those two.
- **One field for the shape**: in Shape mode the radius field sets every vertex; it shows Mixed, empty, when the radii differ, and a value typed gives it to all. Radii 10, 0, 6 show Mixed; 8 typed makes all 8.
- **Corners of a polygon changed**: the radii are kept by vertex number; removed vertices take theirs away; new vertices start at 0. A hexagon of radii 1 to 6 made a square keeps 1 to 4; made a hexagon again, vertices 5 and 6 get 0.
- **Radii of removed corners kept for the session** (planned with point dragging, not yet built): not in the saved file. A polygon of 12 corners of radii 1 to 12 made a triangle, then 12 again, gets 4 to 12 back.
- **No "independent corners" switch**: one corner's radius is set in Points mode only. Shape mode sets all.
- **Fillet**: an arc tangent to both edges, its setback from the vertex `d = r / tan(θ/2)`, θ the corner's inner angle. A right corner of radius 10 has its tangent points 10 from the vertex.

### Points with handles

- **Radius or handles**: each point of a free shape has a switch, Corner radius or Handles, always shown; switched on a regular polygon, the polygon is first converted to a free shape, unmoved. A rectangle's point 2 switched to Handles keeps its rectangle elsewhere.
- **The radius is kept**: in Handles the point is drawn sharp, its radius kept and shown effective 0; switched back, the radius is drawn as it was. 30 stays 30, 0 stays 0.
- **Handle modes**, chosen per point, set the other handle when one is typed:

  | Mode | Stored | The other handle |
  | --- | --- | --- |
  | Mirror angle and length (default) | `angle-and-length` | opposite, same length: (130, 0) about a point at (100, 0) puts the other at (70, 0) |
  | Mirror angle | `angle` | opposite direction, its own length kept; unchanged when the handle typed has length 0 |
  | No mirroring | `none` | unchanged |

- **A curved side**: a side is a cubic curve when a handle is on and not of length 0 at either of its ends; handles on but of length 0 leave it straight. A square's top side whose point 1 has a handle (0, −20) bulges up.
- **A radius beside a curve**: a corner in radius mode beside a curved side keeps its rounding, an arc tangent to the side and the curve, the curve cut there; a radius too large becomes the largest circle tangent to both that crosses neither. A rectangle 120 × 80 with handles of 40 at point 3: a radius 60 at point 2 becomes 47.7.
- **Handles stretch** with the shape, like its points. A shape made twice as wide doubles its handles' x.
- **A lens is refused**: a free shape whose points lie on one line is refused even when its sides bulge, its points spanning no area. Points (0, 0), (50, 0), (100, 0) with bulging handles are refused.
- **The box is the curve's**: a shape's box holds its curves. A square 100 × 100 whose top bulges by 20 shows about 100 × 115.
- **A point added on a curved side** splits the curve at its middle, the drawing unmoved, but a neighbour's fillet that reached past the middle is rebuilt on the half left and may shrink; the new point is in Handles, Mirror angle and length; a neighbour in Mirror angle and length goes to Mirror angle. A radius 60 beside handles of 40 shrinks from 47.7 to 37.2.
- **A point deleted** joins its two sides into one, keeping the outer handles. Point 2 of a curved triangle deleted leaves one side curved by points 1 and 3's handles.
- **A handle typed**, two ways for the same stored vector: x and y of its end on the page; or an angle in degrees clockwise from the side it leaves along, and a length. Angle 370 gives 10; a negative length, or a side of length 0, is refused; an angle typed while the length is 0 is kept until a length is typed.
- **A symmetric arch**: the same angle and length at both ends of a side give the same arch at the four corners of a rectangle, outward for 180° to 359° on a clockwise contour, inward otherwise. 330° bends a side out by 30°.
- **A handle of length 0** shows the angle it kept, or 0. A handle set to length 0 after 45° still shows 45.

### Radius clamping

- **Local proportional reduction per edge**: a radius too large for its edges is reduced, only on the vertices of the edge in conflict. Two corners of radius 80 on an edge 100 long each stop at its middle, 50.
- **One corner alone** may take the whole edge, no further. A radius 1 000 on one corner of a square of side 100 draws 100.
- **No anti-corner**: two arcs of one edge never overlap. Radii 60 and 60 on an edge 100 long give 50 and 50.
- **No error**: the effective radius caps silently. 1 000 typed on one corner of a square of side 100 draws 100.
- **A spike** — a vertex where the contour turns back on itself, turning by 180° — is consumed by its fillet: rounding may eat length. A spike rounded is cut back by its arc rather than kept sharp.
- **No corner to round** (a size of 0, a vertex on a straight line): the radius stays on the vertex, shown as requested, and applies once the shape grows. A rectangle 0 × 50 of radius 10 shows 10 and draws 10 once 40 wide.
- **Known limit**: on a concave contour an arc may touch an edge that is not its neighbour; only neighbouring edges are checked. An L-shape's inner corner of radius 500 may cross the far edge.
- **Requested and effective**: the model stores the radius typed; the effective one is computed at each drawing, never stored; the shape grown, the rounding grows up to the value typed; the interface shows the value typed and signals the effective one when they differ. 1 000 stored on one corner of a square of side 100 draws 100, and 1 000 once the square's side is 2 000.

### Useful special case

- **A circle is a rounded square**: a square of side `c` with radius `c/2` on its 4 corners. Side 100, radius 50.
- **A capsule** is a rectangle rounded by half its smaller side. 200 × 60 with radius 30.
- **A regular polygon at its largest radius** is its inscribed circle, every fillet stopping at the middle of its edges. A hexagon in 100 × 100 becomes a circle of diameter 86.6.

## 3. Creation tools

### Rectangle

- **A free shape**: the rectangle tool makes a free shape of 4 points; width and height stretch any free shape's points, the top-left of its box fixed, without loss. A rectangle made 0 wide, then 100 again, is the same rectangle.
- **The box's eight handles** (planned with resizing by handles): the opposite side stays fixed. The right handle dragged keeps the left side.
- **What a free shape stores**: its corners, at least 3, spanning a box of positive width and height, each a point in the shape's own box with its radius (integer ≥ 0) and its handles; and its `width`, `height`, `angle`, `x`, `y`. Three corners on one line are refused.
- **A corner's handles**: the switch `isOn`, the `mode`, and the vectors `before` and `after` from the point, stretched with it. A new point has its handles off, in `angle-and-length`, (0, 0).
- **Numbers**: finite; integers when typed; fractional after a conversion or an edit; on a turned shape integers in the drawing, the point being stored unturned. A converted triangle keeps a height of 86.60254.
- **Drawn**: the stored points stretched from the top-left of their curve's box, moved to the position, then turned; the rectangle tool stores the unit square. A rectangle 120 × 80 at (10, 10) stores (0, 0), (1, 0), (1, 1), (0, 1).
- **A point typed** in the SVG's coordinates replaces that point, the others kept as drawn, the box fitted around them; on a turned shape the others stay where they are drawn. Point 3 of a rectangle 100 × 50 at (0, 0) typed (120, 50) gives a box 120 × 50.
- **The tool's values**: `width`, `height` (integers ≥ 0, 0 allowed, negative refused), `angle` (0 to 359), `x`, `y` (integers) and `radii`, one per vertex in contour order (integers ≥ 0); they create the free shape, which is then what is edited; a radius too large is reduced when drawn, its value kept. Width −10 is refused; 0 draws a line.
- **Result**: a contour of 4 vertices. A rectangle 100 × 50 at (0, 0) is (0, 0), (100, 0), (100, 50), (0, 50).

### Regular polygon

- **Values**: a number of corners `3 ≤ n ≤ 12`; `width` and `height`, integers ≥ 0, 0 allowed; `radii`, one per vertex in contour order, integers ≥ 0, reduced when drawn and kept as typed. 13 corners are refused; 3 gives a triangle, 4 a square, 6 a hexagon.
- **Why 12 at most**: symbols use triangles, squares, diamonds, hexagons and an octagon, and beyond 12 sides a polygon is hardly told from a circle. A circle is a rounded square instead.
- **Stored**: `n`, `width`, `height`, the radii typed, the `anchor`, the `stretched` option, the `angle` and the position `x`, `y`; the vertices are computed. A hexagon stores 6, never its 6 points.
- **Uniform fit**: the polygon stays regular and fills as much of its box as it can without exceeding it, touching the width, the height or both. A triangle in 100 × 100 is 100 wide and 86.60254 high.
- **Flat base**: its bottom edge is horizontal. 4 corners give a square, not a diamond.
- **Fitted sharp, rounded after**: the box is fitted on the sharp polygon, then the radius rounds inward as on any shape; at the largest radius the polygon is its inscribed circle and may no longer touch its box; no option pushes it back. A hexagon in 100 × 100 at its largest radius is a circle of diameter 86.6.
- **Other orientations** come from rotation. A diamond is a square turned by 45°; a hexagon standing on a vertex is turned by 30°.

**Stretched option**:

- **Touches the four sides**: with "stretched" on (off by default) the polygon touches the four sides of its box, its base flat at the bottom, symmetric about the box's vertical axis, its sides as even as possible. A hexagon stretched in 200 × 100 touches all four sides.
- **The rule**: all sides equal while every corner turns at least half its regular turn, `π/n`; past that, that limit polygon is stretched on each axis to the box. A triangle stays isosceles and a square becomes a rectangle, having no freedom.
- **Otherwise a polygon**: its corners, radii, angle and position work as for any polygon, the effective radii possibly different by corner; its anchor has no effect; converted to a free shape, it does not move. A stretched pentagon converted keeps its 5 points in place.

**Anchor**:

- **Nine places**: the polygon sits in the room its box leaves by an anchor on a 3 × 3 grid, a horizontal alignment (left, center, right) and a vertical one (top, center, bottom), stored `min`, `mid`, `max`; center on both axes by default. A triangle in 100 × 100 sits in the middle by default.
- **`min` and `max`**: `min` puts the polygon's smallest coordinate on the box's, `max` its largest; an anchored edge is written on the integer edge. A triangle in 100 × 100 anchored at the bottom has its base at y = 100, its apex at 13.39746.
- **Always stored**: the anchor is kept and shown even on an axis the polygon fills, where the three alignments draw alike. A square in 100 × 100 anchored left draws as anchored right.
- **Sharp, then rounded**: the anchor places the sharp polygon; the radius then rounds inward without moving it, a flat face keeping its place, a rounded vertex leaving the edge. A triangle anchored at the top with radius 10 has its top at y = 10.
- **The axis with room counts**: in a given box only one axis has room, so a corner place follows the box's proportions. Bottom left puts a triangle at the bottom of 100 × 100 and a square on the left of 100 × 50.
- **Then rotation**: the box turns with its content. A triangle anchored at the bottom, turned by 90°, keeps its base on the side that was the box's bottom.

### Round shape

- **Fills its box**: a circle in a square box, an ellipse otherwise, touching its four sides. A round shape 100 × 50 is an ellipse of half axes 50 and 25.
- **Stored**: `width`, `height` (integers ≥ 0), `angle` (0 to 359), `x`, `y`, and `kind: "round"`; no radius, no anchor, no point to edit. A circle of diameter 80 stores 80 × 80.
- **Drawn as four cubic sides** through its top, right, bottom and left points, point 1 at the top, then clockwise; each handle `4(√2 − 1)/3` of its half axis; within about 0.00027 of the larger half axis of the true ellipse. A circle of diameter 100 is within 0.014 of the true circle.
- **A box of width or height 0** draws the line it is. 0 × 80 draws a vertical line 80 long.
- **Convert to free shape** gives its 4 points with their handles, the drawing unchanged; a box of width or height 0 is not converted; there is no conversion to a rounded rectangle, which would turn an ellipse into a capsule. A circle of diameter 100 converted has 4 points with handles of 27.61424.
- **A capsule** stays a rectangle rounded by half its smaller side. 200 × 60 with radius 30.

### Text

The groups below: drawing and measuring, font metrics, the default font, characters, font face and measured block, box, scripts and turning, fonts from files, installed fonts, embedded fonts, and what is not built.

#### Drawing and measuring

- **A native `<text>`**, the one exception to "everything is a path", with one positioned `<tspan>` per line; the library measures the text and breaks its lines itself; never converted to paths, no HTML layer. "Hello" over "World" draws one `<text>` and two `<tspan>`.
- **Each line's `x`** follows its alignment in the block: left at its left edge, centered half the room it leaves, right all of it. "Hello" over "World" in Arial 16, centered: "Hello" starts 2.66016 right of the block's edge, "World" at it.
- **Physical sides**: left, center and right are the physical sides in both directions. A right-to-left text aligned left starts each line at the block's left edge.
- **Right to left**: such a line is written at its right edge with `direction="rtl"`. A Hebrew line 40 wide in a block at x 0 is written at x 40.
- **Each line's `y`** is its baseline; every space typed is drawn as measured (`white-space="pre"`, `xml:space="preserve"`). "a␣␣b" keeps both spaces.
- **The font** is written as itself, then its fallback, then `sans-serif`, each name once. `"Inter", sans-serif`.
- **The opacity** is written from 0 to 1 (`fill-opacity`). 40 % gives 0.4.
- **Underline and strike-through**: a rectangle as long as the line, placed and as thick as the font says, in its own color once chosen, else the text's; the underline at the font's standard position, not skipping descenders. A line 40 wide gets an underline 40 long.
- **No stroke on an empty line**: a line 0 wide or less, or a font giving a thickness of 0 or less, has no underline nor strike-through. An empty line draws none.
- **A negative line** keeps its negative width and is placed by the same rule, possibly outside the block, which stays at least 0 wide. Lines 10 and −8 wide, centered, start 0 and 9 from the block's left edge; one line −6.89 wide, aligned right, starts 6.89 right of it.
- **Measured from metrics**: a font's metrics are read once from its file and kept; measuring a text is a sum of stored numbers, so moving, turning, showing, hiding and recoloring stay cheap. "Hi" in Arial 16 is H's advance plus i's, 11.55469 + 3.55469 = 15.10938.

#### Font metrics

The library reads a TrueType or OpenType file into plain numbers, in font units:

- **Read**: the units per em (`head`); the ascender, descender and line gap (`hhea`); the underline (`post`), the strike-out and `fsType` (`OS/2`). Arial: 2048 units per em, ascender 1854, descender −434, line gap 67.
- **Advances**: each character's from its Unicode `cmap` subtable of format 12, else 4, and `hmtx`, a glyph past the last record taking the last record's advance; a character not mapped takes glyph 0's, `.notdef`. A Chinese character in Arial takes the `.notdef` advance.
- **Kerning from `GPOS`**: the lookups of its `kern` feature, whatever the script, in lookup-list order, the first subtable of a lookup matching a pair giving its value, the lookups adding up; only the first glyph's `xAdvance`, every pair adjacent, marks not skipped. Two lookups giving −40 and −20 for a pair kern it by −60.
- **Else the `kern` table**: its horizontal format 0 subtables in order, each adding to those before, or replacing them when its override bit is set. Two subtables of −50 and −30 for a pair give −80.
- **Kept by classes**: characters that kern alike share a class, so a pair's kerning is one look-up. "A" and "À" kern alike with "V".
- **Cap height**: `sCapHeight` of `OS/2` version 2 or later; a table older, too short, or giving 0 or less gives 0.66 em. Arial's is 1467 units, its fallback would be 1351.68 for 2048 units per em; Inter's is 1490.
- **Not a font**: a file that is not TrueType or OpenType, or lacks `cmap`, `head`, `hhea`, `hmtx`, `post` or `OS/2`, gives no metrics. A PNG renamed `.ttf` gives none.
- **Apple's tag**: a font whose `sfntVersion` is `true` (0x74727565) is read as TrueType. Helvetica given by Chrome on macOS is read.
- **Collections**: a `.ttc` (header `ttcf`) is read face by face, each at its offset, its tables' offsets counted from the file's start. Helvetica.ttc holds 6 faces, at 48, 380, 696, 1028, 1344 and 1644 — 400, 700, 400 italic, 700 italic, 300, 300 italic.
- **A face of a collection** is kept and given to the browser as a font file of its own: its tables copied, each padded to four bytes, its directory's offsets rewritten; the library gives it as parts, a new directory and views of the collection's bytes, joined only for a face kept; its PostScript name is read at its directory. Helvetica Bold becomes a file of its own of the 700 face.
- **Any table size**: a table is read without copying it into an array of numbers. Apple Color Emoji.ttc's `sbix` table of 191 134 508 bytes is read.
- **Every read checked**: a table too short for what is read, or a part of `cmap`, `GPOS` or `kern` pointing past its table, gives no metrics, never an error; a `GPOS` or `kern` table too short for its own header is left out. A `cmap` pointing 10 bytes past its end gives no metrics.
- **Ranges looked up**: glyph ranges are searched, never spread out. A range of all 65 536 glyphs costs as little as another.
- **Refused fonts**: units per em outside 16 to 16 384, an `hhea` announcing no `hmtx` record, a `kern` subtable shorter than its header, or more `kern` subtables than the table holds, give no metrics. 8 units per em give none.
- **Empty class pairs**: a `GPOS` class pair subtable whose records hold no value gives no kerning, without listing the pairs. Such a subtable adds 0 to every pair.
- **Characters kept**: 2 080 code points, the blocks Basic Latin, Latin-1 Supplement, Latin Extended-A and B, Latin Extended Additional, Combining Diacritical Marks, Greek and Coptic, Cyrillic, Hebrew, General Punctuation, Currency Symbols, Letterlike Symbols, Arrows and Mathematical Operators. € − → ≤ ™ ș ț are measured; a Chinese character is not.
- **Shipped metrics**: the metrics of Inter 4.1's nine weights, upright and italic, ship with the library; Inter has no Hebrew, so a Hebrew letter takes its `.notdef` advance until a font with Hebrew is read. Inter 400's "A" is measured without reading a file.

#### Default font available

- **Each face checked**: as the page opens, the editor measures `Pump P-101 Running` at size 1000 in each of Inter's 18 faces on a canvas, named as the drawing names it. `700 1000px "Inter", sans-serif`.
- **A face kept** when the browser's width lies within 9 units of its metrics' width. Inter 400 and 500 differ by about 116 units on that line, so a 500 drawn for a 400 is caught.
- **Inter not installed**: no face is kept and Inter leaves the font list. A Mac drawing Helvetica for Inter lists no Inter.
- **No font yet**: the list is empty, no text is drawn until a font is read, and the page says `No font is available yet: Inter is not installed on this computer. Click Installed fonts first, a font is chosen as soon as one is read; or import a font file with Import a font.` The font list and the Content field stay disabled meanwhile.
- **The default font chosen**: once the installed fonts are read, never before, a text whose font has no metrics takes the first family of the list as if chosen by hand, its files read again and held so that it can be embedded. `Helvetica is chosen, the first font read: Inter is not installed on this computer.`
- **Its fallbacks**: a family whose files cannot be read again leaves the list and the next is tried; a font imported meanwhile is left as it is. Helvetica unreadable again, the next family, Arial, is chosen.
- **Every font offered can be embedded**: a font chosen never ends as `its file not held`, Inter's shipped metrics excepted. Helvetica chosen by default exports embedded.
- **No font usable**: when the reading ends with none, the page says `No installed font can be used: none maps every basic Latin letter and permits embedding. Import a font file that does with Import a font.` A computer of emoji fonts only shows it.
- **Installed fonts first**: **Installed fonts** comes first in the Text tool, above the font list, in the accent color, under `Start with Installed fonts: the font list offers only the fonts read.`, **Import a font** beside it.
- **A face read replaces the shipped one**: a face read from a file, imported or installed, replaces the shipped face of its weight and style. Inter 700 imported replaces the shipped Inter 700.
- **No canvas**: a browser that cannot measure on a canvas keeps the 18 shipped faces. A test page without canvas lists Inter's 18 faces.

#### Characters

A text holds:

- **Content**, any, the empty one drawing nothing, its lines broken by a line feed, a carriage return or both. "Pump⏎Tank" is two lines.
- **Font**: a name with a fallback, Inter by default; a name is stored trimmed, at least one character, no control character; the font list holds only names read from font files, the fallback's name under the same rule. `" Inter"` and `"Inter⏎Bold"` are refused.
- **Size**: an integer from 1 to 100 000, in the one unit used everywhere. 0 and 12.5 are refused.
- **Weight**: an integer from 100 (thin) to 900 (black). 700 is bold.
- **Bold button**: it gives 700, and pressed again gives back the weight held before; it shows pressed at 700 or while it holds a weight; a weight chosen in the list, a family that changes the weight, or a guided step opened forgets the weight held, and Bold at 700 with none held gives 400. From 100, Bold gives 700, then 100; from 800, 700 then 800.
- **Italic** button. Italic on draws the italic face.
- **Color**: `#` and six lower-case digits, typed in any form the color fields accept and turned to it, with an opacity, an integer from 0 to 100 %. `F80` is kept `#ff8800`.
- **Spacing**: letter and word spacing, integers from −100 000 to 100 000 user units, negative to tighten. −2 tightens each letter gap by 2.
- **Underline and strike-through**, each with its own color and opacity, checked as the text's: until chosen each follows the text's; once chosen it stays when the text's changes; before one is chosen the kept value is black, opaque. A text `#000000` with an underline chosen `#ff0000` at 50 % keeps a red underline when the text turns green.
- **Case**, written by the library (below).
- **Direction**: left to right or right to left; a case, an alignment or a direction outside its values, read from a file, is refused. `"up"` as a direction is refused.
- **Alignment** of its lines (left, center, right), a **position** — the top-left of its block before it turns, integers when typed at 0°, fractional once a turned text has grown — and an **angle**. A text at (40, 60) at 0° has its block's top-left there.

**Case transforms**: each line on its own, its breaks kept; any other case, `sentence` among them, is refused. For "pump P-101 is running":

| Case | Rule | Gives |
| --- | --- | --- |
| none | as typed | `pump P-101 is running` |
| upper | every letter in upper case | `PUMP P-101 IS RUNNING` |
| lower | every letter in lower case | `pump p-101 is running` |
| title | the first letter of each run between white space (space, no-break space, tab) in upper case, the rest kept (`pump  on` gives `Pump  On`) | `Pump P-101 Is Running` |
| camel | words cut at spaces, hyphens and underscores, lower case, each but the first capitalized, joined | `pumpP101IsRunning` |
| Pascal | the same words, each capitalized, joined | `PumpP101IsRunning` |
| kebab | the same words in lower case, joined by hyphens | `pump-p-101-is-running` |
| snake | the same words in lower case, joined by underscores | `pump_p_101_is_running` |
| constant | the same words in upper case, joined by underscores | `PUMP_P_101_IS_RUNNING` |

A run of separators cuts once, so a line of separators only gives an empty line in the programmers' cases. `--_` in camel gives an empty line.

#### Font face and measured block

- **The face CSS would choose**: the faces of the text's style first, the other style when the family has none; its weight if a face has it, otherwise:

  | Weight asked | Faces tried, in order |
  | --- | --- |
  | 400 to 500 | the heavier up to 500 ascending, then the lighter descending, then those past 500 ascending |
  | under 400 | the lighter descending, then the heavier ascending |
  | over 500 | the heavier ascending, then the lighter descending |

  Inter 450 is measured with Inter 500; a family of 300 and 700 measures 400 with 300 and 600 with 700.

- **Family names** match without regard to case. "arial" names Arial.
- **A font without metrics** is measured with its fallback family's face when that family was read, else with the default font's. `monospace` with a fallback Inter read is measured with Inter.
- **The measured block's lines**: the content in its case is cut at each line break; each line is composed into NFC, its invisible formatting characters (Unicode Cf) measured as absent, and measured from advances, kerning and spacings. e and a combining acute measure as é; a zero-width space measures 0.
- **The measured block's size**: as wide as its widest line, as tall as its lines times the line height, less what a trimmed first line gives up above its cap height. Lines 40 and 30 wide give a block 40 wide.
- **Letter spacing** goes between adjacent character units — a character with its nonspacing marks —, never after a line's last letter, so a right-aligned line ends at the block's edge. "AB" with spacing 2 is A + 2 + B.
- **Word spacing** goes at each space and no-break space. "a b" with word spacing 5 adds 5.
- **Empty content** gives no line and a block 0 × 0; lines all narrower than nothing give a block 0 wide. "" is 0 × 0.
- **A final line break** ends the content with an empty line, which counts in the height. "Pump⏎" is two lines tall.
- **In a box** a text is measured and drawn with the lines its box shows, their ellipses included. "Hello World" in a box 50 wide measures "Hello" over "World".

#### Box

- **Two kinds**: a text fits its content, or sits in a box of a given width and height, integers from 0 to 100 000; the box's Width and Height are number fields, a value above 100 000 taken at 100 000, below 0 at 0. 100 001 typed gives 100 000.
- **Wrap at spaces only**: in a box each paragraph wraps after the last space of a run, never at a no-break space nor a hyphen, lines filled greedily; the spaces where a line wraps are neither measured nor drawn. "Hello World" in Arial 16 in a box 50 wide gives "Hello" over "World".
- **Opening spaces** stay with the paragraph's first word, never a line of their own. "␣␣Hello World" in a box 50 wide keeps "␣␣Hello" on line 1.
- **Closing spaces** of every line are dropped, so they neither place the line nor lengthen its background. "Hello World␣" right-aligned in Arial 16 in a box 50 wide ends "World" at the box's edge, x 8.219, its band 41.781 wide, not 46.227.
- **A word too long** stays whole on its own line: it overflows the box with the ellipsis off, or is cut with "…" with it on, never broken at a character; a no-break space ending a word is measured as a character of its own. "running", 53.4 wide, in a box 50 wide overflows, or shows "runn…".
- **Lines shown**: as many as fit the box's height, at most the maximum number of lines when set (an integer, 0 for none, up to 100 000), at least one even in a box shorter than a line, never more than the lines the text wraps into; an empty text shows none. "Hello" over "World" in a box 60 tall shows 2 lines, though 3 would fit.
- **Ellipsis off**: a cut text stops at its last line shown. "Hello World" in a box 60 wide with at most 1 line shows "Hello".
- **Ellipsis on** (the default): a text cut ends its last line shown with "…" (U+2026), and a line wider than the box ends with one, when at least one character can be removed. "Hello World" in a box 60 wide with at most 1 line shows "Hello…", 52.46 wide; in a box 50 wide "Hell…".
- **One character** wider than the box keeps no ellipsis. "H" alone in a box 5 wide shows "H", overflowing.
- **Empty lines** left out add no ellipsis. "Hello" and a line break with at most 1 line shows "Hello".
- **On the last line holding text**: the ellipsis of a text cut goes on its last line shown that holds text, the empty lines after it dropped, a line of spaces being empty; when no line shown holds text it stays on the last one. "Hello", "World", an empty line and "Hello" in a box 60 wide with at most 3 lines shows "Hello" over "World…".
- **Characters removed** from the line's end, with the spaces before them, until line and ellipsis fit; the first character other than a space always kept; never a cut right after a no-break space, which stays with the next character. "Hello", a no-break space and "World" in a box 70 wide shows "Hello…".
- **Three full stops** stand for "…" in a font without it. "Hell..." in such a font.
- **Past the edge**: when even the first character and the ellipsis do not fit, both are drawn past the box's edge, since clipping needs a primitive the project does not allow. "Hello World" in a box 10 wide with at most 1 line shows "H…", 27.6 wide.
- **Alignment**: the lines across the box (left, center, right), the block of lines down it (top, middle, bottom). "Hello" over "World" in a box 60 tall, middle, has 11.6 of room above.
- **Line height**: automatic, the font's, or an integer from 1 to 100 000, 0 meaning automatic; a chosen one spreads the room it leaves half above, half below the letters, the automatic one keeps the line gap below the line. Inter 16 at line height 30 puts (30 − 19.36) / 2 = 5.32 above.
- **Trim**: the block starts at its first line's cap height and ends at its last line's baseline, the cap height `sCapHeight` or 0.66 of the size. Arial 16 trimmed starts 16 × 1467 / 2048 = 11.46 above the first baseline.
- **No wrap style to choose**: lines always wrap greedily. No balanced wrap is offered.
- **Options by kind**: wrapping, maximum lines, ellipsis and vertical alignment apply in a box only; line height, trim and background to both kinds; every option is checked whatever the kind, so the text stays valid when it changes kind. A text fitting its content ignores its maximum lines.
- **Defaults**: fit content, line height automatic, no trim, no background, ellipsis on; in a box, a box 0 × 0, no maximum, the block at the top; a background turned on starts `#ffff00`, opaque. A new background is yellow.
- **Put in a box, a text keeps its size**: Box after Fit content sets the box to the measured block rounded up, a side within a rounding error of an integer kept; a turned text keeps its turned top-left corner. "Pump P-101" in Inter 16, a block 87.953125 × 19.359375, gets 88 × 20; 88.0000000001 gives 88.
- **Measured at each switch** from Fit content to Box while the box's width or height has not been typed. With a second line "Tank", that text's box becomes 88 × 39.
- **The box typed comes back**: once its width or height is typed, Fit content then Box keeps it whatever changed meanwhile; the maximum number of lines alone does not make it typed. A box typed 100 × 200, then Fit content, then Box, is 100 × 200; "Pump P-101" in 88 × 20, width typed 50, a line "Tank" added while fitting, Box gives 50 × 20.
- **Reset**, shown in Box, puts the box back to the measured size rounded up, which counts as not typed. "Pump P-101" goes back to 88 × 20.
- **Forgotten with the text**: the box typed belongs to the text edited; a new text measures again; the content typed or a tool armed does not forget it. A text placed by the Text tool measures its own box.
- **A field emptied** does not count as typed: back in Box that side takes the measured size, the other kept. A box 50 × 15 whose width is emptied comes back 88 × 15.
- **Turning in a box**: a text in a box turns about the middle of its box, its selection showing the box; a box resized while turned keeps its turned top-left corner. A box 60 × 40 at (0, 0) turns about (30, 20).
- **Background**: a color with an opacity, drawn behind each line from the font's ascender to its descender, as long as the line with its ellipsis, in a box ending with its last character other than a space; all lines in one path under the letters; an empty line has none. Inter 16's band is 19.36 tall.
- **Bands keep their height**: a chosen line height or a trim leaves each band from ascender to descender, with gaps under a large line height and overlaps under a small one, filled once; a trimmed text's first band rises above its block. Inter 16 at line height 30 leaves gaps of 10.64.
- **Scrolling** a long text in its box on a touch screen comes with the synoptic views. Not built in the Symbol Editor.

#### Scripts and turning

- **The Latin script** is the editor's target, the one industrial synoptic views are written in. A label "Pompe P-101" is the case served.
- **Other scripts**: the library also measures Greek, Cyrillic and Hebrew, right to left; Arabic and Indic scripts, which need shaping, are not supported. "Насос" is measured; Arabic is not.
- **Latin letters required**: a font lacking a basic Latin letter is not listed, installed or imported. A Greek-only font is not offered.
- **Turning**: like a shape, by an integer angle from 0 to 359, clockwise, about the middle of its block, of the measured text when it fits its content, of its box otherwise. "Hello" over "World" in Arial 16 without kerning, at (0, 0), a block 41.78125 × 36.796875, turns about (20.890625, 18.3984375).
- **A turned text that grows keeps its turned top-left corner**: whatever changes its block's size, its position is computed again so that the corner stays; the position may be fractional. A block 100 × 20 at (47, 61) turned by 23° has its corner at (54.88207, 42.25840); grown to 120 wide, its position becomes (46.20505, 64.90731), the corner unmoved.
- **X and Y show the top-left of its straight box**, the smallest upright box around its turned frame, at 5 decimals, the exact value kept behind; one text selected or several. That block shows X 47.06745, Y 42.2584.
- **Turning** changes X and Y, the stored position kept; a text that grows keeps its corner, X and Y following its straight box. At 23° the corner is the box's highest point, so Y stays and X moves by the change of height times sin 23°.
- **X or Y typed** moves the text so that its straight box starts at the value, the stored position going by the difference, angle, size and the other coordinate kept; a fraction typed is rounded to the nearest integer, a tie up; the value shown typed back changes nothing. X typed 60 on that block stores (59.93255, 61); at 0° X and Y are the position itself.
- **Turn written** as `transform="rotate(α cx cy)"` on the `<g>` holding the `<text>` and its stroke paths, so the strokes turn with the letters; none at 0°. `rotate(30 30 20)`.
- **Selection**: a selected text shows its block, or its box, dashed, turned with it; an empty text and a box 0 wide or 0 tall show none, except a new text being typed (`interaction.md`, Adding a shape). A text in a box 60 × 40 shows a dashed 60 × 40 frame.
- **A turned text's frames** are drawn as a turned shape's: its block or box in blue, a frame 1 thick filled between its contour and the contour inset by 1, drawn only when wider and taller than 2, its straight box dashed and upright behind. A text in a box 60 × 40 turned by 30° shows the same straight box as a rectangle 60 × 40 turned by 30° at the same place.
- **Its straight box** is around its block or box, not its letters' outlines, shown with its x, y, width and height under the position fields; at 0° the frame alone. A block 40 × 20 at (0, 0) turned by 45° has its straight box at (−1.21320, −11.21320), 42.42641 × 42.42641.
- **Empty text**: drawn as nothing; a new text left empty is removed when the panel leaves it (`interaction.md`, Adding a shape). A text placed and never typed in disappears at the next click elsewhere.

#### Fonts from files

- **Two sources**, read by one reader from the file's bytes, once, kept for the session: a TrueType or OpenType file imported (`.ttf`, `.otf`, `.ttc`), or, in Chrome and Edge, a font installed on the computer after a click and the browser's permission; an installed family read ahead keeps its metrics alone until chosen. Arial imported is read once.
- **The font list** holds the families the editor can measure: the default Inter, the fonts imported and the installed fonts read; the weight list shows the chosen family's faces; a text is measured with its own font's metrics, never Inter's for another. `monospace` cannot be typed in.
- **A face named by its file**: its family is name ID 16, else name ID 1, read from the Unicode strings — Windows encodings 1 and 10 and the Unicode platform in UTF-16BE, Macintosh Roman when ASCII; Windows code pages are not read; English first: Windows English (United States), then Macintosh English, then a Unicode name, then any other. "Arial" is read from its Windows English name.
- **Weight and style**: `usWeightClass` brought into 100 to 900, italic when `fsSelection` bit 0 is set. 50 gives 100, 950 gives 900, 450 stays 450.
- **A face read again**, same family (without regard to case), weight and style, replaces the one kept, its family keeping the name first listed; a new family is listed after those kept. A text in `arial` 700 is then measured with the Arial 700 read.
- **A face kept** holds its file, its version (name ID 5, empty when none) and whether `fsType` permits embedding; a file naming no family, or whose `OS/2` is too short for its style, is refused as not a font. `Version 5.01x`.
- **Refused files**: anything not TrueType or OpenType, a WOFF among them, is refused; a collection gives its faces. `font.woff` is refused; Helvetica.ttc gives 6 faces.
- **Weights offered**: the chosen family's faces, lightest first, each with its common name when it has one. `700 Bold`, `450` alone.
- **A family chosen** keeps the weight held when it has that face, else takes the face the matching rule gives. Arial 700 to Inter stays 700; Inter 300 to a family of 400 and 700 gives 400, 600 gives 700.
- **Bold among the faces**: Bold asks for 700, or gives back the weight held, among the family's faces, and holds the weight before only when the face reached differs. On a family of 300 and 600, from 300 Bold gives 600, pressed, and again 300; on a family of 400 alone, Bold keeps 400, released.
- **Import a font**: one or more `.ttf`, `.otf` or `.ttc` files, each face read said, a collection's in one line; the family of the last face read chosen. `Arial 700 imported.`; `Helvetica 400, Helvetica 700, … imported.`
- **Files refused at import**: each named with its reason, on its own; a file the browser cannot read said, the others kept. `notes.ttf is not a TrueType or OpenType font file (.ttf, .otf, .ttc): refused.`; `Apple Color Emoji.ttc could not be read (RangeError: …).`
- **Drawn with its own outlines**: the browser is given the face's file under its family's name as it is, never as a CSS string. IBM Plex Sans 450 imported is drawn with its 450 face, not the installed 500.
- **A face the browser cannot load** is said in one line. `Broken 450 italic could not be loaded by the browser (…): measured with it, drawn with another face.`

#### Installed fonts

- **Listed after a click** and the `local-fonts` permission. `Installed families listed: 312. The font list can be used once they are read.`
- **Disabled while read**: no installed family is offered before it is read and approved; the font list and **Installed fonts** are disabled until every family is read, and while the default font's files are read again; a reading that fails gives them back. `Reading Helvetica, the default font: the font list can be used once it is read.`
- **Read in the background**, family after family, each face on its own, the page answering between files, a line counting them, then summing up with each reason only when a family has it. `Reading the installed fonts: 12 of 312 families…`, then `Installed fonts read: 309 families; 3 left out: 1 not embeddable (…); 1 no Latin letters (Apple Color Emoji); 1 unreadable (ABC Oracle Book).`
- **Listed under its files' name**: a family read keeps its faces' metrics only, its weights those of its faces read; a family none of whose faces is listed never enters the list; a face read ahead never replaces one whose file is held. Helvetica is listed with 300, 400 and 700.
- **Read whole before listed**: at the first reading each face's file is read whole and checked — TrueType or OpenType, the 52 basic Latin letters, `fsType` permitting embedding —, then its bytes released, its metrics kept, so memory holds one file at a time. A file of 192 MB is held only while it is read.
- **Faces refused at the first reading** are not listed nor read again; a family left empty is not listed; the counting line adds the faces refused. `Installed fonts read: 2 families. 1 faces of the families read refused: 1 unreadable (Helvetica Light).`
- **A family chosen**, by hand or as default, has its faces' files read again to draw and embed them, each face on its own, a face refused counted, the others kept; while read again the list shows it chosen, measured with the metrics read ahead. `Arial: 4 faces read.`; `Arial: 3 faces read, 1 refused (…)`.
- **A face refused then** drops its metrics, a text asking it measured with the nearest face held; a family none of whose faces is read gives the list back to the family measured, saying why. Test Sans 400 read and 700 refused: a text in 700 is measured and drawn in 400.
- **A face in a collection** is read from it, the face of its PostScript name kept when the browser gives the whole collection. `Helvetica: 6 faces read.`
- **A face the browser cannot give**, or any other error reading it, is refused in one line with the browser's reason, proposing to import its file, the other faces kept, no error uncaught. `Helvetica: 5 faces read, 1 refused (Helvetica Light: The font Helvetica-Light could not be accessed. Import its file with Import a font).`
- **Browsers without the list** (Firefox, Safari): the page says so and points to **Import a font**; a permission refused is said too.

**Only usable fonts listed** — a face is listed only when the editor can draw, measure and embed it:

- **Not embeddable**: a face whose `fsType` forbids embedding is left out. 2 is refused; 0, 4 and 8 are not.
- **No basic Latin letter**: a face whose `cmap` lacks any of A to Z (U+0041 to U+005A) or a to z (U+0061 to U+007A) is left out whatever its script, since the browser would draw them with another font while the editor measures `.notdef`. Apple Color Emoji, which maps 28 characters and no letter, is left out.
- **One order of reasons**: not embeddable, no Latin letters, unreadable, for a face, a family and the reading line; a face with two reasons is said by the first; a family none of whose faces is listed is said by the first reason one of its faces has. A family of a face not embeddable and a file that is no font is counted not embeddable.
- **A family chosen says the faces it leaves out**. `Test Sans: 1 faces read, 1 refused (Test Sans Bold: its fsType forbids embedding it in an exported SVG).`
- **Imported faces** follow the same rules, said why, the other faces of a collection kept. `Locked.ttf: Locked 400 refused, its fsType forbids embedding it in an exported SVG.`

#### Embedded fonts

- **An option of the document**, off by default, applied only when a file is exported or saved: the drawing on screen holds no `<style>`. Export with the option on writes the fonts; the screen never does.
- **One rule per face** its texts are measured with, an `@font-face` with the whole file as a base64 `data:` URL in one `<style>` of the `<defs>`. 10 texts in Arial regular and bold write 2 rules.
- **Permission**: a face is embedded only when `fsType` permits it — not bitmaps only, nor restricted without the preview-and-print or editable bit. Arial (8) and Inter (0) are embeddable; 2 is not.
- **Faces left out**, each listed with its reason: no file held, not permitted, family incomplete, generic name, not named. Inter from shipped metrics: `Inter 400: not embedded, its file not held.`
- **The face embedded** is the one the text is measured with, of its font, else its fallback, named as read and escaped as the text's `font-family`. `My "Font"` gives `"My \"Font\""`.
- **Shared faces**: two texts measured with the same face share one rule. Arial 600 and 700 both use Arial's 700 face when it holds 400 and 700.
- **The rule written**, in the order of each face's first use: `@font-face { font-family: "Arial"; font-style: italic; font-weight: 700; src: url(data:font/ttf;base64,…) format("truetype"); }`, `normal` for an upright face, `font/otf` and `"opentype"` for CFF outlines (`OTTO`), never `local()`.
- **Nothing to embed**, option off or no face: no `<style>`. A drawing with no text has none.
- **A family whole or not at all**: when one used face of a family may not be embedded, every used face of it is left out, since an `@font-face` family hides the installed one. Inter 400 held and 700 not embeds no Inter.
- **Generic names** are never embedded. A family named `Monospace` is left out.
- **Named families only**: a face reached only as the default font is left out. Inter reached for a text in Helvetica is left out.
- **Never edited with**: the editor never measures nor edits with a font embedded in a file. A file opened with an embedded Arial is measured with the Arial read.
- **In the playground**: the drawing stands for a document, **Embed fonts** applies to all its texts, off as the page opens; each face a file would leave out is listed while the text is edited; **Export SVG** saves `drawing.svg`, its fonts in `<defs>` when the option is on, without the selection's frame.

#### Not built

- **No HTML layer**: the library computes the lines, about 3 µs a text, and the scripts that would need one are outside the target. No `foreignObject`.
- **Not wanted**: lists, highlight shapes beyond a background color, CSS color names, RGB fields. `red` is refused.
- **Text styles**: named styles (title, body, description) as local templates that forbid nothing, with the libraries. A "title" style presets size and weight.

### Layout box

A box placing its children, simpler than CSS, planned after the shape tree; "layout box" is a working name.

- **A rectangle only**, with a corner radius at most; its vertices cannot be edited. A box 200 × 100 with radius 8.
- **Two modes**: children placed by anchor on the 3 × 3 grid, or stacked in a row or column with gaps; the box may have padding. Three children stacked in a row with gaps of 10.
- **A child's own anchor**: a child may override the anchor, stays in the box, and leaves the stack entirely. In a box placing children top right, one child anchored bottom left.
- **No hug**: the box keeps the size typed, never resizing to its children. A box 200 × 100 stays so with one small child.
- **Children**: shapes, text, later symbols; the polygon in its own box is the one-child case.
- **Stacks are a drawing aid**: they live in the Symbol Editor; once the symbol is built every position is static, and an element hidden by an animation leaves its place empty. A hidden middle child leaves a gap.

### Out of scope (symbols)

- **No pen nor freehand drawing** in the Symbol Editor. Curves come from typed handles only.
- **No other Bézier curves** than cubic sides from handles, and no bend tool dragged on the drawing.
- **No corner smoothing** (squircles). A corner is an arc.
- Static drawings of the View Editor follow their own rules (§8).

## 4. Node editing

See `interaction.md` §3.

## 5. Boolean operations

| Operation | Result |
| --- | --- |
| Union | area covered by at least one shape |
| Difference | bottom shape minus top shape |
| Intersection | common area |
| Exclusion | area covered by exactly one shape |
| Division | bottom shape cut into pieces by the top outline |
| Cut path | outlines cut at intersections, without fill |

- **Non-destructive**: an operation makes a boolean node `{ operation, operands[] }`, its operands staying integer and editable; the result exists in the derived geometry only. A union of two squares keeps both squares editable.
- **Radii first**: the operands' corner radii are applied before the operation. Two rounded squares unite with their arcs.
- **Shape builder**: regions built from the edges and intersections of the selected shapes are kept or merged, stored as "inside A, outside B". The lens between two circles is stored as inside A, inside B.
- **No flattening** into a plain shape, and no radius on the vertices a result creates (open question Q9).

## 6. Strokes

- **Computed as a path**, never by SVG's `stroke`: the contour offset at ± the width, then the ring between both. A square of side 100 with a stroke 10 centered is the ring between the squares of side 110 and 90.

| Property | Values | Example |
| --- | --- | --- |
| Width | integer ≥ 0 | 2 |
| Alignment | `inner`, `center`, `outer` | `outer` puts the whole band outside |
| Join | `miter`, `round`, `bevel`, with a miter limit | `round` |
| Cap (open contours) | `butt`, `round`, `square` | `butt` |
| Dashes | a pattern of integers, measured on exact arc lengths `r × θ` | `4 2` |

- **Open contours** ignore the alignment, having no inside. An open line with `inner` is stroked centered.
- **Exact offsets**: the offset of a circular arc is a concentric arc. An arc of radius 10 offset by 2 is an arc of radius 12.

## 7. Shape tree

See `interaction.md` §6.

## 8. Static drawings

- **Made in the View Editor** by business users, with more freedom than symbols but few options; no parameter and no animation. A grouped frame around a zone is a drawing.
- **A group saved** becomes a drawing, put into a drawing library shared between projects. A legend drawn once is reused in every view.
- **Free curves in drawings only**: a pen places points, and a drag on a point pulls Bézier handles. Symbols keep typed handles.
