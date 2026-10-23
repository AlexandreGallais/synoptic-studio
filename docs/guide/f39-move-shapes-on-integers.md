# F39 — Move shapes on integers

## What you can do now

**Drag a shape with the mouse.** With the Selector, press a shape and drag it: it follows the pointer smoothly, its outline with it. Release it, and its X and Y become whole numbers: the rectangle 100 × 50 at (10, 20), dragged 30.6 to the right and 19.4 down, ends at (41, 39), the nearest whole numbers to where the pointer took it, a half rounded up, as a click places a shape. Its size, its radius, its angle and every setting stay as they were: a move never changes the shape itself.

**A turned shape lands by its upright box.** X and Y of a turned shape are the top-left corner of the upright box around it as drawn, as the panel shows them. The same rectangle placed at (200, 20) and turned by 30° shows X 194.19873 and Y −1.65064; dragged 10.6 to the right and 3.2 down, it shows X 205 and Y 2. Its own corners keep their fractions, since a turned shape's corners are rarely whole numbers.

**Several shapes move together.** Select shapes with Shift + click, Ctrl + A or a selection rectangle, then drag one of them: all of them move by the same amount, so their distances stay exactly as they were. The top-left corner of the dashed box around them lands on whole numbers: the straight rectangle and the turned one above, dragged by (10.6, 3.2), have their dashed box go from (10, −1.65064) to (21, 2), and both move by (11, 3.65064).

**Press a shape that is not selected.** It becomes the selection alone, and moves. With Shift held at the press, it is added to the shapes selected, and all of them move. Press one of several shapes selected, with or without Shift, and they all move.

**A click stays a click.** A move starts as soon as the pointer leaves the whole-numbered point where it was pressed, the same rule as a selection rectangle and a drag with a tool: a press released on that point is a click, which selects the shape alone, as before; a hand trembling to the next whole number moves the shape by one.

**Shift keeps one direction.** Hold Shift while dragging and the shapes move only across or only down, whichever way the pointer went farther; release Shift during the drag and they follow the pointer freely again. A tie goes across. Only the direction they move in lands on whole numbers: the direction Shift keeps still keeps its fraction, as Penpot does, so that a rectangle at y 20 dragged across with Shift together with the turned one, whose dashed box starts at y −1.65064, stays at y 20 (set on 2026-10-23).

**Nudge with the arrow keys.** With shapes selected and the focus outside the panel's fields, an arrow key moves them by exactly one unit, Shift + an arrow by ten: the rectangle at (10, 20) goes to (11, 20) with Right, to (20, 20) with Shift + Right, to (10, 19) with Up. A nudge never rounds: the turned rectangle whose X is 194.19873 goes to 195.19873 with Right, keeping its fraction until a drag lands it on a whole number. Several shapes move together, by the same amount, and stay selected. Held down, an arrow moves them on at each repeat. The sliders of the color picker count as fields: with the focus on one, every key is its own, so Delete or Ctrl + A there leaves the shapes as they are (set on 2026-10-23).

**Empty places still draw a rectangle.** A press on an empty place of the drawing, even inside the dashed box around several shapes, draws a selection rectangle, as before; it never moves the shapes.

## The choices the agent made

For the review's summary; each can be changed.

- While dragged, the shapes are drawn exactly where the pointer takes them, with fractions, and land on whole numbers only at the release, so that the move looks smooth; the other way was to jump by whole units at each step.
- No shape is outlined as under the pointer while shapes move, as while a selection rectangle is drawn.
- Shift is read at each move of the pointer and at the release, so that releasing it during a drag frees the other direction again; a tie, as far across as down, goes across.
- A drag the browser cancels, taking the pointer for a gesture of its own, puts the shapes back where they were, the selection as it was at the press.
- In Points mode, which edits the points of one shape, a drag moves nothing.
- A text never measured, which is not drawn, stays selected but does not move, as when a value is typed for several shapes.
- The arrows move the shapes whichever tool is armed, as Delete removes them, and one step at each key the browser sends, so that an arrow held moves on; with shapes selected, the page does not scroll.
- The arrows are left to the browser, which may scroll the page, with no shape selected, in Points mode, and with Ctrl, Cmd or Alt held, which browsers use for some commands of their own. A field of the panel keeps them, the anchor grid's buttons included.
- While a drag moves shapes, an arrow moves nothing, and neither Delete, Ctrl + A nor Escape changes anything; the drag lands them at the release.
- A box that starts on a whole number up to a rounding error, as a rounded rectangle at x 0 whose box starts at −0.0000000000000018, lands exactly on that number, its position taking the tiny error, as when X is typed.

## The ideas behind it

**Whole numbers at the release.** The editor stores whole numbers wherever the user places something, so that symbols line up exactly. On 2026-10-22 a drag was set to follow the pointer freely and land on whole numbers at the release, the shape itself never changed; on 2026-10-23, at the refinement of this feature, the point that lands was set to the top-left of the upright box, the X and Y the panel shows, as Penpot's pixel grid rounds each shape's upright box.

**One shift for several shapes.** Penpot rounds each shape on its own, which can change the distances between them by a fraction. Here the shapes move together, by the shift that lands their dashed box, chosen on 2026-10-23 so that a group of shapes drawn with care keeps its spacing exactly.

**What a press does, as other editors.** Inkscape and Penpot select a shape not selected when it is pressed and dragged, and move it; Inkscape and Sketch move every shape selected when one of them is dragged. Shift at the press adds a shape, as Penpot; Shift during a move keeps one direction, as Sketch and Penpot.

**Nudges as other editors.** The arrows move by 1 and by 10 with Shift, as Figma; Penpot never rounds a nudge, which was chosen on 2026-10-23 so that a shape keeps its fraction, a drag being the move that lands on whole numbers.

| Term | In plain words |
| --- | --- |
| straight box | the smallest upright rectangle around a shape as it is drawn, turned or not |
| dashed box | the smallest upright rectangle around every shape selected, drawn dashed when several are |
| shift | how far the shapes move across and down at the release |
