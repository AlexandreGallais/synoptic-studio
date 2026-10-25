# Interaction

How the user acts on shapes in the editors: tools, selection, node editing, snapping, colors, alignment, the shape tree, the view and the history. In the examples, A is a rectangle 100 × 100 at (0, 0), B one at (150, 0) and C a circle in the box from (300, 0) to (400, 100), drawn in that order, unless an example says otherwise.

## 1. Tools

| Tool | Role |
| --- | --- |
| Selector | select, move, resize objects |
| Node tool | select and modify the vertices of a shape |
| Rectangle | create a rectangle |
| Polygon | create a regular polygon of n corners |
| Text | create a text |
| Port | place a connection point (Symbol Editor) |
| Pipe | connect two ports (View Editor) |

- **No pen and no freehand drawing** in symbols (`shapes.md` §3). Curves come from typed handles.
- **Keys editable**: every shortcut and modifier can be changed by the user in a standard settings place, the defaults below (planned with the editor's settings). Delete may be remapped to X.
- **A key typed in a field is the field's**, never the drawing's, a slider of the color picker counting as a field; but a field with nothing typed since its value was applied leaves Ctrl + Z, Ctrl + Shift + Z and Ctrl + Y to the drawing (§7). Delete in the Width field deletes a character, not a shape.
- **Ctrl stands for Cmd** on macOS in every key below. Cmd + Z undoes on a Mac.

### The default keys

| Action | Default |
| --- | --- |
| select a shape | a click with the Selector |
| add or remove a shape | Shift + click; Ctrl + click kept for picking inside a group once groups exist |
| select by a rectangle | a drag with the Selector from an empty place: the shapes it touches; with Alt, those it encloses; with Shift, added |
| select all | Ctrl + A |
| select none | Escape, or a click on the empty drawing |
| delete the selection | Delete or Backspace |
| nudge | an arrow, by 1, landed on integers |
| nudge further | Shift + an arrow, by 10 |
| undo | Ctrl + Z |
| redo | Ctrl + Shift + Z, or Ctrl + Y |
| zoom in, out by a step | `+` or `=`, and `-`, about the pointer over the drawing, else the canvas's centre |
| zoom to 100 % | Shift + 0, about the canvas's centre |
| fit the drawing, the selection | Shift + 1, Shift + 2 |
| keep the proportions while resizing by a handle | Shift held (planned) |
| turn by steps of 15° by a handle | Shift held (planned) |
| snapping off for a while | Ctrl held while dragging (planned) |

### Adding a shape

- **A click adds a shape** 100 × 100 centred on the point clicked, on integers; a text fits its content instead. The Rectangle tool clicked at (250, 140) adds 100 × 100 at (200, 90).
- **On integers**: the top-left of the box, before it turns, is the point clicked less half the box's size, each coordinate rounded to the nearest integer, a tie up (toward +∞). (250.5, 140.4) places 100 × 100 at (201, 90).
- **A text without metrics** (no font available yet) is not measured, its block 0 × 0, its top-left the point clicked, rounded. (250.5, 140.6) gives (251, 141).
- **A kept text whose metrics are no longer read** is drawn and exported with the metrics it was last drawn with, so the shapes already there do not change. A text in a family whose files left the session keeps its frame.
- **A drag draws the box**: a press, a drag and a release draw the shape's box, the shape following the pointer; a text dragged sits in a box of that size. Dragged from (10, 10) to (130, 90), 120 × 80 at (10, 10).
- **The box dragged**: the press and the pointer each rounded to the nearest integer, a tie up; the box from the smaller to the larger of each coordinate, in any direction. (130, 90) to (10, 10) gives 120 × 80 at (10, 10); (10.4, 9.6) to (130.5, 90.49) gives 121 × 80 at (10, 10).
- **Square with Shift**: its side the larger distance dragged, laid from the press toward the pointer, down or right when level; Shift pressed or released during the drag changes the shape at once; a text's box squares too. (130, 90) to (10, 10) with Shift gives 120 × 120 at (10, −30); the Text tool from (10, 10) to (130, 90) with Shift gives a box 120 × 120 at (10, 10).
- **A release within 3 screen pixels of the press is a click**, at any zoom, the pixels divided by the view's scale: the shape is placed centred on the point pressed; beyond, the press is a drag until its release. At 10 %, a tremble of 1 pixel, 10 units, still places 100 × 100 centred on the press; at 800 %, a drag of 7 pixels across from (10, 10) draws 1 × 0 at (10, 10).
- **A width or a height of 0** adds the shape as drawn, a size the model accepts. (10, 10) to (10.3, 90) adds 0 × 80 at (10, 10).
- **A drag whose release the page never heard** (the window left with the button down) is dropped at the next press on the drawing, its shape left as last drawn. The next press with the Selector does not move it.
- **On top**: the drawing is an ordered list, the last drawn on top; a new shape goes above all. A rectangle added after A, B and C is drawn over C.
- **The toolbar**: a tool's button arms the tool and leaves the shape edited as it is; a click on the drawing places the tool's shape, the panel edits it, and the Selector is armed again; with the Selector armed, a click picks. Rectangle, click, and the next click selects.
- **A shape refused** (a value out of its range) is not kept when another is placed. A polygon refused, then a rectangle placed, leaves the rectangle alone.
- **Duplicating** (planned): a copy remembers the offset of its first move for the next copies, so copies never pile up. A copy moved by (20, 0) puts the next one 20 further.
- **Reset and each guided step** start the drawing as the page opens: the shapes placed and each kind's settings forgotten, the Text tool's fields back to the page's. The next text is Inter 32 Regular.

### What a new shape starts with

- **Box and place** from the click or the drag, straight, angle 0. A polygon placed after one turned by 30° starts at 0°.
- **The last shape of its kind** the panel showed gives every other setting: a rectangle its radii; a polygon its corners, radii, stretched option and anchor. A polygon set to 3 corners, radius 4, anchored bottom center, then a rectangle of radius 0: the polygon tool clicked at (250, 140) adds that triangle, its box 100 × 100 at (200, 90).
- **A text** takes every option of the last text but its content and box — font, size, weight, color, spacing, decorations, case, line height, trim, background, ellipsis. A text in Roboto 20 bold gives the next text Roboto 20 bold.
- **One memory per kind**: a rectangle's values never reach a polygon; before any shape of a kind, the page's values. Radius 10, 6 corners, anchor at the center, Inter 32 Regular.
- **Kept when the panel leaves the shape**, for a new shape or a conversion. A polygon set to 3 corners, then converted, gives the next polygon 3 corners.
- **A new text has no content**: the designer clicks, then types, its Content field taking the focus. Click, type `Valve`.
- **A new text being typed**, from its placing until its Content field is left with a content, or the panel leaves it: while empty it shows a frame 1 wide and one line tall, dashed, where its first character will go; it draws and exports nothing; only a new text shows that frame. A text placed and not typed in shows a thin dashed frame.
- **That frame is centred** on the point clicked, on integers, so the first line is centred on it and the text grows to the right. Inter Regular 32 clicked at (250, 140), its line (1984 + 494) × 32 / 2048 = 38.71875 tall, has its top-left at (249.5, 120.640625), placed on (250, 121).
- **A new text left empty** stays being typed while its field is empty, so its size or font may be set first; the panel leaving it still empty — a click on the drawing, another shape placed, Escape, Ctrl + A, an undo — removes it, no step taken. Text placed, Escape: nothing left, nothing to undo.

## 2. Selector

### Under the pointer

- **The top shape whose painted area holds the point**, on its contour as drawn — turned, rounded, curved — by the nonzero rule; none when no shape holds it. A, then a box from (50, 50) to (150, 150): (75, 75) gives the second, (25, 25) A, (200, 200) none; a rectangle 100 × 100 of radius 20 holds (8, 8) but not (3, 3).
- **A text** is under the pointer anywhere in its frame — its block, or its box — turned with it, not only on its letters. A click between two words picks the text.
- **A shape without area** (a box of width or height 0) is under the pointer within 4 screen pixels of the line its corners join, divided by the view's scale; a shape whose points lie on one line though its box has an area is never picked. At scale 1, 0 × 80 at (10, 10) is under (12, 50), not under (15, 50).

### Two layers

- **The drawing's SVG** holds only the shapes and receives the clicks; a transparent SVG above it, which the pointer passes through, draws every help — the outline under the pointer, the selection, the rectangle; the drawing is never polluted by them. An export holds no outline.
- **No page text selected**: no selection of the page's text starts on either layer, so a text of the drawing pressed, dragged across or double-clicked is picked or moved as a shape; the panel's fields keep selecting their text. A double-click on a drawn text highlights no letter.

### The outline

- **Under the pointer**: with the Selector armed, the shape under the pointer is outlined in the layer above on its contour as drawn — a text's frame, a flat shape's line —, 2 screen pixels wide; none while a tool is armed, nor once the pointer leaves the drawing; drawn again with the drawing; never exported. At 200 % the outline is 1 unit wide.
- **Seen on any color**: every help — outlines, the dashed box, the selection rectangle's line — is one line of white and black dashes, its path stroked white, then black over it, dashed 4 and 4 screen pixels from the path's start; a native stroke, in the layer above only, never on a shape. A's outline is black from 0 to 4 along its top side, white from 4 to 8.
- **Screen pixels**: widths and dashes are divided by the view's scale; a contour's outline is 2 pixels wide, a box's line 1 pixel on half pixels. At 100 % an outline is 2 units wide dashed `4 4`, at 200 % 1 unit and `2 2`, at 25 % 8 and `16 16`.
- **Drawn whole**: an outline is the path of the contour, stroked by the browser at any size and zoom; nothing is computed nor kept between redraws, a pan changing the `viewBox` alone. A rectangle 99 999 × 100 000 is outlined at 1 % by its path, 200 units wide dashed `400 400`.

### A click with the Selector

- **Selects the shape under the pointer alone**, which keeps its place in the drawing order, the panel showing and editing it; a click on the empty drawing selects none. A, B and C, C edited: a click on A edits A, still under B and C.
- **Its place kept**: the drawing is split around the shape edited, the shapes before it below, those after it above; a shape placed next goes on top of all. A rectangle placed after editing A is drawn above C.
- **Which click picks**: only a click whose press was made with the Selector armed, never the click ending a tool's drag or placing a shape, nor the one ending a Selector drag. A drag from (200, 150) to (100, 60) with a tool, released over another shape, edits the shape drawn.
- **A click on the shape selected** loads it again, no point selected. Vertex 2 selected, a click on the shape clears it.
- **The shape the panel leaves** keeps its settings as the last of its kind, unless it was refused. A rectangle of radius 25 left: the next rectangle has radius 25.
- **The selection shown**: the shape edited is outlined as the shape under the pointer, whichever tool is armed; none while it is refused; never exported. With the Text tool armed, the selected rectangle stays outlined.
- **None selected**: every shape stays drawn, the panel hides its fields, its title says no shape is selected and its status how to select or add one. A click on the empty drawing shows "No shape selected".

### Several shapes selected

- **Shift + click** adds the shape under the pointer, or takes it out when selected; a plain click, or Ctrl + click, selects it alone. A selected: Shift + click on B selects A and B; again on B, A alone; Ctrl + click on C, C alone.
- **No range on the drawing**: Shift + click never selects the shapes drawn between two others. Shift + click on C with A selected leaves B out.
- **On the empty drawing** Shift + click keeps the selection; a plain click selects none. A and B selected, Shift + click on empty space keeps both.
- **Two or more selected**: none is edited; every shape stays in its place and order; the selection is held by the shapes' indices; the panel hides the shape's fields, its title says how many, `2 shapes selected`; back to one, that shape is loaded as a click loads it; a shape placed meanwhile goes on top, selected alone. A and B selected, Shift + click on B loads A into the panel.
- **Shown**: each shape by its outline, and a dashed box, the smallest upright box holding their contours as drawn — a shape without area by its corners, 0 × 0 by its point —; a text never measured is left out; one shape gets its outline only. A and B give the box from (0, 0) to (250, 100).
- **The dashed box's line** is 1 screen pixel wide on its sides moved half a pixel inward, so it covers whole pixels. That box has its line on (0.5, 0.5) to (249.5, 99.5).
- **Ctrl + A and Escape**, outside the fields: Ctrl + A selects every shape in the order drawn, Escape none, in either mode; a field keeps both keys, a slider included; an open color picker takes Escape, closing it; a shape refused is not kept by either. Ctrl + A with A, B and C selects the three.
- **Delete or Backspace**, outside the fields, removes the shapes selected, keeps the others in their order and selects none, whichever tool is armed, the tool staying armed; every field keeps the key, the Content field and the sliders included; with none selected nothing changes; in Points mode it deletes the points selected, never the shape. A and C selected, Delete leaves B.
- **Points mode edits one shape**: its button is disabled while several are selected; a click with Shift, or on a vertex's handle, acts on the points only; Ctrl + A is left to the browser. With A and B selected, Points is greyed.

### The selection rectangle

- **What it takes**: the shapes whose painted area it touches; with Alt held, only those it encloses. The rectangle from (−10, −10) to (200, 110) touches A and B and encloses A alone.
- **Touches**: a piece of a shape's contour as drawn has a point in the rectangle, or, none having one, the rectangle's top-left corner is in the painted area; a text by its frame. The rectangle from (390, 90) to (420, 120) touches no C, its nearest point to C's centre (350, 50) lying 56.6 away, beyond the radius 50.
- **Encloses**: the straight box of the contour, arcs and cubic sides by their extremes, lies within the rectangle. A is enclosed by the rectangle from (0, 0) to (100, 100).
- **The outline counts**: a rectangle meeting a shape along a side or a corner touches it, one along all its sides encloses it, the box compared within `EPSILON`, and the rectangle widened by `EPSILON` for touching. A polygon square 100 × 100 at (0, 0) of radius 10, its left side at x ≈ 1.1 × 10⁻¹⁵, is touched by the rectangle from (−10, 40) to (0, 60).
- **A shape without area** is touched when the rectangle meets its line, no tolerance, and enclosed by its corners; a shape whose points lie on one line though its box has an area is touched by its line and enclosed by its box, so it can be selected and removed. 0 × 80 at (10, 10) is touched by (5, 40) to (15, 50), not by (12, 0) to (20, 100).
- **Its drag**: with the Selector and Shape mode, a press of the main button on an empty place starts it; the press and the pointer are rounded to the nearest integer; it shows once the pointer is more than 3 screen pixels from the press. Pressed at (−10.4, −9.6) it starts at (−10, −10).
- **At the release** the shapes it touches are selected, or with Alt held at the release those it encloses, one alone loaded into the panel. Pressed at (−10, −10), released at (200, 110): A and B; with Alt, A alone; from (110, 10) to (140, 90), none.
- **With Shift** at the release, the shapes taken are added, none taken out, so a rectangle taking none keeps the selection. A selected, (140, −10) to (260, 110) with Shift selects A and B; without Shift, B alone.
- **A drag the browser cancels** selects nothing. The selection stays as it was.
- **A drag is never a click**: a press whose pointer went more than 3 screen pixels from it is a drag until its release, even released back on it, and the click it brings picks nothing; a press on a shape moves the shapes selected instead of drawing a rectangle; in Points mode a drag draws no rectangle. Pressed on A at (50, 50), released at (52, 51) at 100 %, √5 = 2.24 pixels away, is a click.
- **Shown**: in the layer above, never exported, a light tint of the outlines' color; the shape under the pointer is not outlined meanwhile. A rectangle drawn over A tints A lightly.
- **Its line** is the outlines' dashed line, 1 screen pixel wide on its sides moved half a pixel inward, a rectangle 0 wide a line along its middle. The rectangle from (−10, −10) to (200, 110) at scale 1 has its line on (−9.5, −9.5) to (199.5, 109.5); (5, 300) to (5, 350) on (5, 300.5) to (5, 349.5).
- **What it would take, shown**: while drawn, the shapes it would take are outlined as under the pointer, and selected at the release only; Alt is read at each move, Shift changes nothing outlined. C selected, (−10, −10) to (200, 110) outlines A and B; with Alt, A alone.

### The panel for several shapes

- **Each shape's values**: X and Y the top-left of its straight box, W and H its own width and height; a text by its frame — block, or box —, X and Y its straight box's. A sharp 100 × 50 at (10, 20) turned by 30° has X 4.19873, Y −1.65064, W 100, H 50; a text whose block is 80 × 30 at (300, 20), turned by 90°, has X 325, Y −5, W 80, H 30.
- **Common or Mixed**: a value is common when every shape's lies within `EPSILON` of the first's, then shown as one shape's; otherwise the field is empty, its placeholder Mixed. A 100 × 50 at (10, 20) and B 40 × 40 at (200, 20) show X Mixed, Y 20, W Mixed, H Mixed.
- **Shown**: four fields of their own, X, Y, Width and Height, under the title, the shape's other fields hidden; hidden with one shape or none; a text never measured is left out; with no shape left to show, each field is empty, without a placeholder. Two texts never measured leave the four fields empty.
- **A value typed**: a number, white space around it ignored; with a sign it is added to each shape's own value, without one each shape is set to it. A 100 wide at (10, 20) and B 40 wide at (200, 20): `+50` in W gives 150 and 90; 460 gives both 460; `-20` in X moves A to −10 and B to 180.
- **X or Y set** moves each shape so that its straight box starts at the new value, its position going by the difference; a shape 0 × 0 is taken by its point. A sharp 100 × 50 at (10, 20) turned by 30°, its straight box at x 4.19873, goes to position x 55.80127 for 50 in X.
- **W or H set** gives each shape its own size, a turned shape keeping its turned top-left corner, its other settings kept. That shape made 150 wide by `+50` has its position at (6.65064, 32.5).
- **A text**: X or Y moves it by its straight box; W or H sets its box's size when it sits in one, its turned corner kept; a text fitting its content keeps its size and never refuses the value; a text never measured is left as it is. A text turned by 23° in a box 100 × 20 at (47, 61), the box made 120 wide, has its position at (46.20505, 64.90731).
- **The arrows**: Up in a field adds 1 to each shape's own value, Down takes 1 away, 10 with Shift, as `+1` typed; a step one shape refuses changes none; a text typed and not applied is dropped; a burst of arrows is one step (§7). W Mixed at 100 and 40: Up gives 101 and 41, Shift + Up 110 and 50.
- **In X and Y** each shape's own straight box then lands on integers. A sharp 100 × 50 turned by 30°, its straight box at (4.19873, −1.65064), with an upright one at (10, 20): Up in X gives (5, −2) and (11, 20).

### Every number field

The rule of every number field of the page — the panel's, the toolbar's, several shapes' and one shape's:

- **A text field acting as a number field**: a value typed is applied on Enter or when the field is left, never while it is typed; the field then shows the values again. 150 typed in Width draws nothing until Enter, then 150 at once.
- **A sign is relative** to the number the field showed, read as it got the focus or at a key while no text typed waits in it; a negative value is relative, so a negative x is reached from the shapes' own. The radius 10: `+5` gives 15; Up after `5` typed at 10 gives 11; `-5` typed in the Angle at 30 gives 25.
- **Any number**: a comma is a decimal separator as a point; the value is computed from the numbers as typed and, when a fraction is typed, rounded to the nearest integer, a tie up; an integer typed keeps a shape's own fraction in the shapes' X, Y, W and H. `1.5` in W gives 2; `+1.5` at 10 gives 12; `1,5` reads 1.5; a converted triangle 86.60254 high gets 96.60254 for `+10`.
- **Fields of whole numbers** — every other field, a point's X and Y and the handles' among them — round their result whatever was typed, so a fraction shown is rounded by an arrow or `+1`; the Zoom alone keeps a fraction. 33.54102 and Up give 35; 11.83476 and `+1` give 13; 141.5 in the Zoom gives 141.5 %.
- **Only a text that is no number is refused**: an empty field, an exponent, a unit, an expression, a number too large for a double; the field shows its number again and nothing applies. `abc` in the Angle at 0 shows 0 again; `-25+12` is refused.
- **A value beyond the range is taken at its bound**, an angle going round: the Angle and the picker's hue. `-15` in a radius of 10 gives 0; 13 in Corners gives 12; 101 in an opacity gives 100; 370 in the Angle gives 10; `-90` at 10 gives 280.
- **Several shapes** each take the value at their own bound, the others as typed. `-50` in W makes A (100) 50 wide and B (40) 0 wide; 200 000 makes each 100 000.
- **Up and Down** step the value by 1, 10 with Shift; the key's default is prevented, Ctrl, Cmd or Alt leaving it to the field. The radius 10: Up 11, Shift + Up 20, Shift + Down 10.
- **The ranges**, each read from what the field sets:

  | Field | Range |
  | --- | --- |
  | Memory, Steps kept | 0 to 1 000 MB, 0 to 100 |
  | Zoom | 1 % to 25 600 % |
  | Corners | 3 to 12 (2 gives 3) |
  | a text's Size; Max lines, Line height | 1 to 100 000; 0 to 100 000 |
  | a text's four opacities | 0 to 100 |
  | Letter spacing, Word spacing | −100 000 to 100 000 |
  | the picker's H; S, B or L | 0 to 359, going round; 0 to 100 |
  | Angle | 0 to 359, going round |
  | Radius, a point's Radius, a width or height | 0 to 100 000 |
  | a point's X and Y, its handles' x and y | any whole number |

- **Mixed radii**: a field showing Mixed takes a relative value, or an arrow, at each vertex's own radius, within 0 and 100 000; a fixed value sets them all; a text that is no number shows Mixed again. Radii 10, 0 and 6 and `+5` give 15, 5 and 11.
- **A shape refused** has no value to start from: a value typed is written as typed, a relative one from 0, a width or height within 0 and 100 000. 200 or `+200` in a refused shape's Width draws 200 once the shape is valid again.
- **The value applied is what is drawn**: a redraw before Enter — a zoom, a font loaded, the canvas resized — reads the value applied, never the text typed. 150 typed in Width and not applied leaves the rectangle 120 wide through a zoom.
- **A text typed back as shown applies nothing**, so an exact value behind the decimals is kept. A turned text's X typed back with its 5 decimals keeps its exact position.
- **One shape's fields**: the X, Y, Width and Height of the shape edited alone, and a text's X, Y and box, follow the same rule, each change computed from the shape as last drawn. The rectangle 120 × 80 at (0, 0): Up in Width gives 121; Shift + Up in X moves it to x 10; Shift + Down in Y to y −10; `-200` in Width gives 0.
- **An arrow in one shape's X or Y** moves it as the arrow on the drawing along that axis — Up in X as Right and Down as Left, Up in Y as Down and Down as Up —, by 1 or 10 with Shift, its straight box's top-left then landed on integers on both axes. A sharp turned by 30°, its straight box at (6.65064, −2.5): Up in X gives X 8 and Y −2; Up in Y gives Y −1, −1.5 rounding up, and X 7.
- **No straight box**: a shape refused or a text never measured takes the arrow's step as a value typed. Up in the X of a text never measured adds 1 to its X.

### Moving shapes

In this section A is 100 × 50 at (10, 20).

- **Integer values**: a move and a resize land on integers. A shape dragged by 30.6 moves by 31.
- **A move lands its anchor**, the top-left of the straight box, on integers at the release; the shape itself is never changed: a side 10.5 long stays 10.5, its points keep their fractions. A dragged by (30.6, 19.4) lands at (41, 39).
- **Several land together** by the top-left of their dashed box, every shape moved by the same shift, their distances kept. A, and B 100 × 50 at (200, 20) turned by 30°, dragged by (10.6, 3.2), go by (11, 3.65064), their dashed box landing from (10, −1.65064) on (21, 2).
- **The shift**: on each axis `nearestInteger(s + d) − s`, `s` the anchor and `d` the pointer's offset, a tie up; a straight box on an integer within a rounding error lands exactly on it, its position taking the error. B's straight box at (194.19873, −1.65064), dragged by (10.6, 3.2), lands at (205, 2), its position at (210.80127, 23.65064).
- **The press**: with the Selector and Shape mode, a press on a shape starts a move once the pointer is more than 3 screen pixels away. At 100 %, A pressed and moved 3 pixels stays at (10, 20); moved 4 pixels it lands at (14, 20).
- **What moves**: a shape already selected moves with the selection, Shift held or not; a shape not selected is selected alone, or added with Shift held at the press; one shape is loaded into the panel at its new place. B selected, A pressed with Shift and dragged: A and B move.
- **A press on an empty place**, inside the dashed box too, draws a selection rectangle. A and B selected, a press between them starts a rectangle.
- **Shift during a move** keeps the axis of the larger displacement, the other set to 0, across on a tie, read at each move and at the release; the axis kept still is not landed. A dragged by (30.6, 19.4) with Shift lands at (41, 20); by (20, −20), at (30, 20).
- **Shown while dragged**: from the first step beyond 3 pixels the shapes are drawn at the pointer's offset, not landed, their outlines and dashed box following, no shape outlined as under the pointer. A pressed at (50, 40), pointer at (80.6, 59.4): drawn at (40.6, 39.4), landing at (41, 39).
- **Keys during a move**: from the press on a shape to the release, Delete, Backspace, Ctrl + A, Escape and the arrows change nothing. B selected, A pressed and moved, Delete leaves A and B.
- **A drag cancelled** by the browser gives back the drawing and the selection of the press. A moved, then cancelled, stays where it was.
- **Nothing moves** in Points mode, and a text never measured stays selected but does not move, left out of the anchor.
- **Free positions**, off the integers, are an open question (Q43).

### The arrows

- **A nudge**, outside the fields: an arrow moves every shape selected by 1 toward it, Shift + an arrow by 10, the y axis pointing down, every shape by the same shift. A at (10, 20): Right to (11, 20), Shift + Left to (0, 20), Up to (10, 19).
- **Then it lands**: the anchor — the straight box's top-left, the dashed box's for several — lands on integers on both axes, a tie up. A sharp turned by 30°, its straight box at (6.65064, −2.5): Right to (8, −2), 7.65064 rounding to 8 and −2.5 to −2.
- **An anchor on integers but for a rounding error** moves by the nudge exactly, its position kept whole. A rounded rectangle whose box starts at −1.8 × 10⁻¹⁵ moves by exactly 1.
- **Several**: their dashed box lands, each shape going by the same shift. A at (10, 20) and B turned by 30°, their dashed box at (10, −1.65064): Right lands it on (11, −2), both going by (1, −0.34936).
- **Which key**: whichever tool is armed; one step at each key the browser sends, repeats included; with shapes selected in Shape mode the page does not scroll. Right held, 30 keys, moves A by 30.
- **Left to the browser**: with no shape selected, the shape edited refused, in Points mode, or with Ctrl, Cmd or Alt held; a field keeps the key, the anchor grid's radios and the picker's sliders included; nothing moves while a press with the Selector moves shapes; a text never measured stays. Up with nothing selected scrolls the page.
- **Two arrows held**: each arrow's key moves the shapes by the sum of the arrows held outside the fields, Left and Right cancelling out, Shift × 10 on both axes. A at (10, 20), Up held, then Right while Up is held, Right repeated 29 times: (10, 19), then (+1, −1) at each, to (39, −10).
- **Held and released**: an arrow is held from its `keydown` outside the fields to its `keyup` anywhere; the window losing the focus releases them all; an arrow pressed in a field, or with Ctrl, Cmd or Alt, is never held; any key or pointer event carrying Cmd releases every arrow, one still down held again by its next repeat without Cmd. Cmd + Up never leaves Up held.

### A picked shape in the panel

- **Shown as the kind it was made as**: each shape is kept with the kind the panel showed it as; a rectangle comes back as a Rectangle with its radius, a polygon converted as a free shape; a rectangle shows x, y, width, height, angle and radius, a polygon also its corners, stretched option and anchor, a round shape its box and angle. A rectangle stored as a free shape is picked as a Rectangle.
- **A text loads its whole style** — family, weight, italic, strokes, every option —, its family chosen in the font list and measured with, so the next text typed takes that style. A text in Roboto 700 italic, struck through, picked while the family is Inter, shows Roboto, 700, italic.
- **A picked text's X and Y** are the top-left of its straight box, its fractions kept. A box 100 × 20 at (47, 61) turned by 23° shows X 47.06745, Y 42.2584.

## 3. Node editing

- **An explicit mode**: the Node tool, in the playground the **Points** button beside **Shape**, never only a double-click; vertex handles large enough to click. Points shows the selected shape's vertices.
- **A polygon becomes a free shape** by a dedicated button: it loses its number of corners and its anchor and does not move; a polygon of width or height 0 cannot be converted, the button disabled; a rectangle needs no conversion; a vertex's own radius keeps the shape what it is. A hexagon converted keeps its 6 points in place.
- **The selected shape only**: its vertices are shown, never the others'; vertex 1 is red, vertex 2 green, the red on top when they coincide. A rectangle at (0, 0) shows red at (0, 0), green at (100, 0).
- **A list of the vertices** in contour order: number, x, y (at most 5 decimals), the radius typed and the effective one when reduced; a row clicked selects its vertex. Row 3 of a hexagon in 100 × 100 reads (0, 50) with its radius.
- **Selecting vertices**: a click selects one; on the drawing Shift + click or Ctrl + click adds or removes; in the list Shift + click selects a range from the last row clicked without Shift, replacing the selection, and Ctrl + click adds or removes; with no start yet Shift + click acts as a click. Rows 2 then Shift + 5 select 2 to 5.
- **The radius field** applies to the selected vertices, shows their common radius or Mixed, and is disabled with none selected; the selection is cleared when a polygon's corners are typed or Shape mode is chosen; the points' radius field and the shape's radius field are both shown. Vertices 1 and 2 of radii 10 and 0 show Mixed.
- **Moving a vertex**: x and y typed, integers, negative allowed (dragged in the editors, planned); each vertex keeps its number; the coordinate typed is an integer, the other keeps its value; a fraction stepped by an arrow or a relative value is rounded, a tie up; the fields apply on Enter or when left. A converted hexagon's point 1 at (75, 93.30127) goes to (75, 94) by Up.
- **Degenerate results** are allowed while the points span some width and height: a vertex typed onto another, vertices on one slanted line, crossing edges; two vertices at one place and crossing edges are flagged with an offer to delete one, never merged on their own (flags planned with the editors); typed coordinates are not capped. Point 2 typed at point 1's place is kept.
- **Adding a vertex**: with two neighbouring vertices selected (the last and the first are neighbours), a button adds one at the middle of their edge, sharp, fractions allowed, right after the edge's first vertex. Vertices 2 and 3 of a rectangle 100 × 100 at (0, 0) give a new vertex 3 at (100, 50).
- **After it**: the vertices after it are numbered one more; on a curved side the curve is split; the new vertex is selected alone. The old vertex 3 of that rectangle becomes vertex 4.
- **Deleting vertices**: a button or Delete and Backspace, outside the fields, on a rectangle or a free shape; each vertex goes with its radius; its two neighbours are joined by a straight edge, or by a side curved by their outer handles. Vertex 3 of that rectangle deleted leaves a triangle (0, 0), (100, 0), (0, 100).
- **After a deletion** the others keep their order, numbered from the first left; the box shrinks around them; a polygon is converted first; the selection is cleared. Vertex 1 of a square deleted makes old vertex 2 the new vertex 1.
- **At least 3 vertices spanning an area** remain: otherwise Delete is greyed and a line says why. A triangle's vertex cannot be deleted.
- **Planned**: several vertices moved at once by a relative value; arithmetic in number fields (`+`, `−`, `×`, `÷`, parentheses). `+10` in x moves every vertex selected by 10.

## 4. Snapping

| Target | Description |
| --- | --- |
| Integer grid | every position is rounded to an integer; for a turned shape, the x and y of its straight box, its stored position staying fractional |
| Objects | edges, corners, centres, midpoints of bounding boxes; vertices; ports |
| Alignment | temporary horizontal and vertical guides when a point aligns with a point of another object |
| Distribution | a stop where the gap to a neighbour equals the gap between two other objects |

- **Tolerance in screen pixels**, independent of the zoom. A tolerance of 8 pixels is 4 units at 200 %.
- **An object wins over an alignment**, even a farther one. A corner 6 pixels away wins over a guide 2 pixels away.

## 5. Colors, align and distribute

### Colors

- **A color** is a hexadecimal value and an opacity, an integer from 0 to 100 %, in two fields always shown beside its swatch for copy and paste; no RGB fields, no CSS color names. `#ff8000` at 100 %.
- **Uses**: a text's color, its background, its underline's and strike-through's; the shapes' fills and strokes later.
- **Typed or pasted**: with or without `#`, any case, white space ignored, in 6 digits, in 3 digits each doubled, or in 8 or 4 digits whose last pair or digit is the alpha, setting the opacity to the nearest percent. `f80` gives `#ff8800`; `#0000ffcc` gives `#0000ff` and 80 %; `#F80C` gives `#ff8800` and 80 %.
- **One stored form**: `#` and six lower-case digits, the only form the model accepts, every reader turning a color to lower case first. `#FF8800` read from a file is kept `#ff8800`.
- **Refused** when the field is left or Enter pressed: the field shows again the color it had at the focus, or the last one kept, and a line under it says why; while typed, the text is drawn with what the field holds once it is a color. `red` is refused.
- **Copied** as its six digits, the opacity copied from its own field. `#ff8000` at 50 % copies the color without its 50 %.
- **The underline's and strike-through's colors** have the same swatch, fields and picker, with a box "follows the text's color", ticked as the page opens: ticked, the fields show the text's color and the stroke follows it; a value committed or picked unticks it, a value refused or being typed leaving it ticked; ticked again, it follows again. `#ff0000` typed for the underline unticks it.
- **The picker** opens on a click on the swatch and closes on another click on it, on Escape or on a click outside; it holds a hue slider of whole degrees from 0 to 359, a hue rounding to 360 shown 0, an opacity slider from 0 to 100 %, a plane in HSL or HSB (HSB first), the plane's three values as numbers, and the hexadecimal and opacity fields. `#ff8000` shows hue 30°.
- **In the playground** the picker opens under its color's fields, which stay in sight, a click on them does not close it, and Escape gives the focus back to the swatch.
- **Another model chosen** reads the color again in it, a grey keeping the hue the picker holds. `#7e7f80`, its hue typed 200 in HSB, shows HSL 210°, 1 %, 50 %.
- **The plane** shows the colors of the hue: saturation across from 0 at the left to 100 % at the right, lightness or brightness up from 0 at the bottom to 100 % at the top; a click or a drag picks both. A click at the top right in HSB gives the pure hue.
- **Whole numbers shown and typed**: HSL and HSB values are integers; the hexadecimal stays the truth, kept as typed, the values keeping their decimals inside; the picker reaches fewer colors than the hexadecimal. `#ff8000` is HSL 30°, 100 %, 50 % and HSB 30°, 100 %, 100 %.
- **One value edited** rebuilds the color from that integer and the exact values of the two others, so the others never move; a value typed equal to the one shown changes nothing. `#336699` is HSB 210°, 66.66667 %, 60 %, shown 210, 67, 60; its brightness typed 61 gives `#34689c`, not `#33679c`.
- **While the picker is open** each edit is rebuilt against the color last read — at opening, a hexadecimal typed, another model chosen —, a value equal to the one read standing for its exact value, a value typed staying as typed. H 30, S 100, L 50 typed one after the other on `#1e293b` give `#ff8000`.
- **To the hexadecimal**: red, green and blue each rounded to the nearest integer, a tie up; the hexadecimal and the opacity are what is kept. HSL 30°, 100 %, 50 % gives `#ff8000`.
- **A grey** has no hue: it shows 0°, while an open picker keeps its slider's hue, so a grey made lighter keeps it. `#808080` shows 0° when the picker opens.
- **Beyond a range**: saturation, lightness and brightness at their end, a hue going round; the End key gives 359°; a value left empty shows its value again; a click on the opacity slider gives the nearest percent. A hue typed 400 is 40, −10 at 0 is 350, 360 is 0.
- **The keyboard**, each slider and the plane naming their value for a screen reader:

  | Where | Keys | Change |
  | --- | --- | --- |
  | a slider | Right, Up | adds 1 (1° or 1 %): 30° gives 31° |
  | a slider | Left, Down | takes 1 away |
  | a slider | Page Up, Page Down | adds and takes 10 |
  | a slider | Home, End | 0 and the end (359° for the hue, 100 % for the opacity) |
  | the plane | Left, Right | saturation by 1 % |
  | the plane | Up, Down | lightness or brightness by 1 % |
  | the plane | Page Up, Page Down, Home, End | as on a slider, on the saturation |

### Align and distribute

- **Align**: left, horizontal centre, right, top, vertical middle, bottom. A and B aligned top share y 0.
- **Distribute**: equal horizontal or vertical gaps. Three shapes 100 wide over 0 to 400 get gaps of 50.
- **Shape builder**: select shapes, then keep or merge the regions built from their edges and intersections (`shapes.md` §5).
- **Whatever the group**: computed on bounding boxes in world coordinates. A shape in a turned group aligns by its box in the view.

## 6. Shape tree (layers)

- **A hierarchy** of groups and shapes, with a stacking order; rename, hide, lock, reorder. A group "Body" holding two rectangles.
- **Planned**: the tree in the model (groups, children), then the layout box, both before booleans; the layers panel near the end of the project.

## 7. Navigation and history

### The view

The examples use a canvas of 1 200 × 800 CSS pixels.

- **A scale and an origin**: the view is a scale, CSS pixels per user unit, and an origin, the drawing's point at the canvas's top-left; it is not in the drawing, the history nor the export, and changes no shape's path, coordinate or field. The page opens at scale 1, origin (−40, −40), and Reset goes back to it.
- **Its `viewBox`**, given to both SVGs: `(o.x, o.y, W / s, H / s)`. The opening view is `-40 -40 1200 800`; scale 2, origin (180, 130), is `180 130 600 400`.
- **The canvas's size**: read as the page is drawn, and the page drawn again whenever the canvas changes size, before the paint, the view keeping its scale and origin; the stage around it clips what overflows and is never scrolled. In a window of 2 560 × 1 000 the canvas 1 960 × 1 000 gives `-40 -40 1960 1000`; the pipeline column of 300 pixels hidden, `-40 -40 2260 1000`.
- **The point under the pointer** is `o + p / s`, computed, never read back from the page. At scale 2, origin (180, 130), the offset (440, 340) is over (400, 300).
- **A zoom about a point** keeps it under the pointer: to `s′` about `p`, the origin becomes `o + p / s − p / s′`. From the opening view, the pointer at (440, 340) over (400, 300), zoomed to 2, the origin is (180, 130).
- **The bounds**: 1 % to 25 600 %, a scale beyond taken at the bound. 300 gives 256; 0.001 gives 0.01.
- **The steps**: a key's or a button's step multiplies the scale by √2 or divides it; the wheel and the pinch zoom by the sum `δ` of their vertical deltas in a frame, × (1 + 0.0057 · |δ|) toward the screen, its inverse away. 100, 141, 200, 283, 400 %; δ −100 gives × 1.57, 100 gives × 0.63694.
- **A pan** keeps the scale: the wheel's deltas `δ` move the origin by `δ / s`, a drag of `d` by `−d / s`. At scale 2, origin (180, 130), a drag of (100, 50) gives (130, 105), the wheel's (0, 100) gives (180, 180).
- **A fit** shows a straight box whole and centred in the canvas less 40 pixels on each side, within the bounds, `s = min((W − 80) / w, (H − 80) / h)`; a box 0 wide fitted by its height, 0 high by its width, 0 × 0 centred at the current scale. The box from (10, 20) to (410, 220): scale 2.8, origin (−4.28571, −22.85714).
- **The gestures**: over the canvas, Ctrl or Cmd with the wheel, and a pinch, zoom about the pointer; the wheel alone pans; the wheel's default is prevented; a line counts 40 pixels, a page 800. At 200 %, origin (180, 130), `deltaY` 100 pans to (180, 180).
- **The wheel with Shift**: off macOS it pans across, the vertical delta and a horizontal one added; on macOS by its deltas as sent; macOS is read once, from a platform starting with `Mac`. At 200 %, origin (180, 130), Shift and `deltaY` 100 pan off macOS to (230, 130); a trackpad's (30, 40) with Shift on macOS to (195, 150).
- **A drag that pans**: the middle button, or the main one while Space is held outside the fields, pans by the pointer's moves, nothing selected, moved nor placed; Space held has its default prevented, and in a field it is the field's. At 200 %, origin (180, 130), a drag of (100, 50) gives (130, 105).
- **Space released** once the window loses the focus, or at any key or pointer event carrying Cmd; Space pressed with Cmd is not held; Space still down after Cmd is released is held again by its next repeat. Space, Cmd, Space released, Cmd released: the next Selector drag draws a rectangle.
- **A change of the view takes no step** and changes only the `viewBox`; Reset and the guided steps bring back the opening view. A 100 × 50 at (10, 20) reads X 10, Y 20, W 100, H 50 at any zoom.
- **Read through the view**: the pointer's point is read through the view at each move, so shapes dragged while zooming land where the pointer takes them. That shape pressed at (50, 40), moved to (60, 40), zoomed to 157 %, moved 15.7 pixels right, lands at (30, 20).
- **The helps keep their size on screen**, in screen pixels over the scale: outlines 2 wide dashed 4 and 4, the dashed box's and the rectangle's line 1, the reach of a shape without area 4, a vertex's handle of radius 8 and its halo 13, a frame's line 1, its dashes 4 long every 7 and 1 thick, a curve's handle a stem 1.5 wide and a square of 8. At 200 % a handle has radius 4 units, its halo 6.5.
- **The zoom keys**, outside the fields: `+` or `=` steps in, `-` out, about the pointer over the drawing, else the canvas's centre; Shift + 0 gives 100 % about the centre. From the opening view, the pointer over (400, 300), `+` gives 141 %, origin (88.87302, 59.58369); at 25 600 % it changes nothing.
- **The fit keys**: Shift + 1 fits every shape, Shift + 2 the shapes selected, a text never measured left out, nothing changing with none. A, B and C from (10, 20) to (410, 220) fit at 280 %.
- **Read by the key's place**: Shift with a digit is read by its place, whatever character the layout sends; a key with Ctrl, Cmd or Alt is left to the browser. Shift + 0 is `)` on a US keyboard and `0` on a French one, both 100 %.
- **The percentage** in the toolbar, rounded to the nearest integer, a tie up, opens a menu: Zoom in, Zoom out, Zoom to 100 %, Fit the drawing, Fit the selection, each with its key and about the canvas's centre. A scale of √2 shows 141 %.
- **Its field** takes a percentage about the canvas's centre, following every number field's rule. 200 typed from the opening view gives 200 %, origin (260, 160); 30 000 gives 25 600 %; 0 gives 1 %; `-50` at 200 % gives 150 %.
- **The export** is written in the opening view's frame whatever the zoom. At 100 % and 200 % the file's `viewBox` is `-40 -40 1200 800`.

### The history of states

- **States**: the drawing's past states, its present and its states to come; a state is the drawing and its selection, the indices of the shapes selected. A nudged once: one past state, the present.
- **A step**: each finished action records the drawing and the selection; a shape whose JSON text is a present shape's is shared with it, the one at the same index first; a drawing of the same objects in the same order takes no step, only the selection replaced; any other change pushes the present onto the past and drops the states to come. Selecting B alone takes no step; deleting it takes one.
- **What one step is**: a drag at its release, a value when Enter is pressed or the field left, a text when its editing is left, an addition, a deletion; selecting alone never. A drag of A across the drawing is one step.
- **The selection kept**: a state keeps the selection of its step; undo gives back the selection before the step, redo the one after. A′ and B, B selected, B deleted: undo brings B back selected, redo leaves A′, none selected.
- **Undo and redo**: undo makes the last past state the present, redo the first to come; with none, nothing changes. Undo with nothing past leaves the drawing.
- **Its size** counts the data, in bytes: each shape a past or future state holds and the present does not, once, its JSON text's length × 2; plus 8 for each reference to a shape and each selection index the past and future hold. A rectangle 100 × 50 of 614 characters nudged once, selected: 1 228 + 16 + 8 = 1 252 bytes.
- **Its size kept**: beside its states the history keeps its size and, for each shape, how many past and future states hold it, so a step, an undo, a redo and the bound update the size from the shapes they move, never counting it again. A nudged from A and B, A selected: 24 bytes for the references, then 1 228 for A left by the present, 1 252.
- **Its bound**: a memory size and a number of steps kept whatever their size, set by the designer, 100 MB and 5 steps by default; beyond the size the oldest past states are dropped first, never leaving fewer than the steps kept. A move of A alone adds 1 244 bytes, so 12 440 bytes keep 10 moves, the eleventh dropping the first.
- **What the bound spares**: a step larger than the size is kept within the steps kept; the states to come count in the size but only the next step drops them. A size of 0 with 5 steps kept still keeps 5 steps.
- **The moves held**: the size over the 1 244 bytes of one move of A, rounded down to tens. 100 MB holds about 80 380 moves; 20 MB 16 070; 0, 0.
- **The list of steps**: the states held, oldest first, each named by the step that led to it, read from the drawings before and after, the oldest state `Start`. A nudged, B deleted, 150 typed in A's W: `Start`, `Move`, `Delete`, `Width`.
- **The names**: shapes added `Add`; removed `Delete`; the same shapes in another order `Change`; a kind changed `Convert`; only `x` and `y` `Move`; one other field, the panel's label of it; several fields `Change`. A and B swapped is `Change`.
- **Field labels**: `isItalic` `Italic`, `isUnderlined` `Underline`, `isStruckThrough` `Strike-through`, `strikeThroughColor` `Strike-through color`, a polygon's `radii` `Radius` and `stretched` `Stretched to its box`, `boxWidth` `Width`, `textCase` `Case`, `maximumLines` `Max lines`; any other field its name in words. `letterSpacing` gives `Letter spacing`.
- **The window of the list**: at most 15 rows, the present always, at most 7 states to come after it and the rest before, the side holding fewer giving its rows to the other, the states left out counted on each side. Start and 20 moves, the last present: `6 earlier steps`, 14 moves and the present.
- **Going to a step**: a click on a row makes its state the present by undos or redos, the steps after it kept to redo until a new step drops them; the present, or a state no longer held, changes nothing. A nudged, B deleted, 150 typed in W, then `Move` clicked: B back, `Delete` and `Width` to redo.

### Undo and redo in the playground

- **Where a step is taken**: at each click on the drawing, each release of the pointer on it, a cancelled drag included, and each key released outside the fields, once the page is redrawn; a click, a selection rectangle, Ctrl + A or Escape keep only the selection; a cancelled drag takes none. A shape placed by a click is one step.
- **A key held** is one step from its press to its release; arrows held together one step from the first press to the last release, the window losing the focus taking it. Right held, 30 keys, A from (10, 20) to (40, 20), then Right once more, (41, 20): Ctrl + Z gives (40, 20), then (10, 20).
- **The panel's changes**: a step at each value applied, each choice (the anchor, Stretched, a handle's mode, a weight, the fit), each button (a turn, Convert, Add a point, Delete points, the handles, Bold, Italic, the box's Reset) and the end of a burst of arrows; a change that leaves the drawing as it was takes none. 150 typed in A's W is one step; 100 typed back over 100 is none.
- **In the picker**, a step at each key released on a slider or the plane and each release of the pointer on them. The picker open on `#ff8000`, Right 5 times on the hue, the plane dragged once and `#336699` typed: 7 steps.
- **A text placed and typed** is one step, taken when its Content field is left with a content; Ctrl + Z then removes it; a content typed later is a step of its own; Ctrl + Z while a new text is typed, its field not typed in, removes it, the history unmoved. The Text tool at (250, 140), `Valve` typed and the field left: one Ctrl + Z removes the text, Ctrl + Shift + Z brings it back.
- **Taken from the page as last drawn**, by the field's or the button's own redraw, so a value a field refuses stays as typed. A refused value leaves the history as it was.
- **A redraw asked is drawn before the drawing acts**: a release or a click on the drawing first draws a redraw waiting for the next frame, its step taken. B 100 × 100 at (200, −5), 20 typed in Y and the empty drawing clicked at once: B stays at y 20.
- **A burst of arrows in a field** is one step, taken once the arrow is released and no arrow follows within 500 ms. A's W at 100, Up held, 30 keys: 130, one Ctrl + Z giving back 100; Up pressed once more 300 ms after a release joins the burst, 2 s after it is a step of its own.
- **A burst ended early**: an arrow in another field ends it at once and opens its own; any other action takes its step with the burst's changes; Ctrl + Z within the pause undoes the burst; the window losing the focus ends it. Up in W, then Up in H at once: two steps.
- **A value typed in the burst's field ends it**: a key there other than an arrow or a modifier takes the burst's step at once, the value typed then taking its own. W stepped from 100 to 101, then `150` and Enter within the pause: two steps, Ctrl + Z giving back 101, then 100.
- **Cmd ends a burst**: an arrow with Ctrl, Cmd or Alt opens none, and any key or pointer press carrying Cmd ends the burst at once, no `keyup` waited for; the picker's sliders keep a step at each key released. W stepped from 100 to 103, then Cmd pressed: one step, 103.
- **A shape refused** takes no step while refused; a click on the empty drawing then drops it, a step that Ctrl + Z undoes, the shape back with its last valid values and selected; Ctrl + Z, or Ctrl + Shift + Z, while it is refused gives back its last valid values, the history unmoved. A polygon refused, then 8 typed in its Corners: one step, from 6 to 8.
- **A press holding its action** — the Selector moving shapes, a tool drawing one — takes no step and leaves the history's keys without effect until its release. Ctrl + Z during a drag does nothing.
- **The keys**: Ctrl + Z undoes, Ctrl + Shift + Z and Ctrl + Y redo, Cmd for Ctrl on macOS, the letter read in either case, the default prevented even with nothing to undo; with Alt held the key is the browser's. Cmd + Shift + Z sent as a small z on macOS redoes.
- **The page recorded first**: undo and redo first record the page as it is, so a change not yet recorded is the one undone. A value applied and not yet recorded is undone first.
- **A state written back**: the panel leaves the shape it edits, its settings kept as the last of its kind; the state's drawing is written, then its selection made, one shape loaded into the panel, several kept by their indices; the tool armed stays armed. A at (10, 20) clicked, dragged by (30.6, 19.4) to (41, 39): Ctrl + Z gives A at (10, 20), selected.
- **A drag of a shape not selected**, undone, gives back the selection before its press. B selected, A dragged, Ctrl + Z: B selected again.
- **The fields**: a field is typed in from its first `input` until its value is applied or the focus leaves it; typed in, it keeps Ctrl + Z, Ctrl + Shift + Z and Ctrl + Y; any other field, a slider included, leaves them to the drawing. 150 typed in A's W and Enter, the focus in W: Ctrl + Z gives A 100 wide.
- **The buttons**: Undo and Redo in the toolbar, their titles naming the keys, Undo disabled with nothing past, Redo with nothing to come, beside them the history's size and the size set, then the moves of one shape the size holds, digits grouped by commas. `History 1.2 kB of 100 MB`, `about 80,380 moves`.
- **A size written**: below 1 000 bytes in bytes; above, in the first of kB, MB and GB (each 1 000 times the one before) keeping it below 1 000 once rounded, to one decimal rounded half up, a decimal of 0 left out. 999 bytes `999 B`; 1 248 `1.2 kB`; 1 250 `1.3 kB`; 999 950 `1 MB`.
- **The bound's fields**: Memory, in MB from 0 to 1 000, and Steps kept, from 0 to 100, 100 and 5 as the page first opens, beside Undo and Redo, each title saying what it bounds and that a move of the rectangle the page opens with adds about 1,248 bytes; a value applied follows every number field's rule; a text that is no number writes back the value in force. 1 001 MB gives 1 000; 2.5 MB gives 3.
- **The bound kept in the browser** under `synoptic-studio.history-bound`, `{"size":20000000,"steps":3}`, read when the page opens, each part kept when it is a whole number within its range, else its default; without storage, or storage full, it holds until the page closes. `{"size":-1,"steps":3}` reads 100 MB and 3 steps.
- **A bound applied** bounds the history at once, the oldest steps dropped until it fits, never the steps to redo. The rectangle nudged 4 times, 2 undone, then 0 MB and 1 step: 1 to undo and 2 to redo, `History 3.7 kB of 0 B`, 3 × 1 248 = 3 744 bytes.
- **When it waits**: while a press holds its action, a new text is typed or the shape edited is refused, the bound applies at the next step; Reset and the guided test keep it; its fields keep their keys as the panel's. Delete in the Memory field deletes no shape.
- **The list of steps in the playground**: `Steps` in the toolbar, after Undo and Redo, opens the list under it, each row a button named by its step, the present marked current, the steps to redo greyed and in italics, a line for the steps left out; a click moves the history as Undo does, nothing while a press holds its action. The page as it opens lists `Start`.
- **The list is written again only when the history changes**. A click that takes no step leaves it as it is.
- **A step's fields** are its shape's as the panel shows them — a rectangle's corners as `radius`, a free shape's as `points`, a polygon's `corners` — with the kind the panel shows. A rectangle made a hexagon by the panel's kind, then converted, is `Convert` twice.
- **Reset and the guided test** empty the history; it starts again from the page once redrawn. `History 0 B of 100 MB`, the list showing `Start` alone.
