# F35 — Several shapes on one drawing

## What you can do now

Choose the Rectangle, Polygon, Round shape or Text tool, click the drawing, and a shape is added where you clicked, above the shapes already there. The export holds every shape, in the order drawn.

Choosing a tool changes nothing yet: its button stays pressed until you click the drawing. The new shape is then the one the panel edits, and the Selector button is pressed again, as in Figma; a click with the Selector places nothing. A shape the page refuses, a value out of its range, is not kept when you place the next one. Reset, and each step of the guided test, start the drawing again as the page opens. The rest of this page is written at F35's validation (VAL-036).

## The ideas behind it

**A new shape starts with the settings of the last shape of its kind.** A polygon set to 3 corners and a radius of 4, then a rectangle set to a radius of 0: the next polygon placed is again a triangle of radius 4, and the next rectangle has a radius of 0. Only its box, its place and its angle are new: 100 × 100 centred on the click, straight. Before any shape of a kind, the new one has the values the page opens with: a radius of 10, 6 corners, the anchor at the center, Inter 32 Regular. A new text has the content `Text`, selected, so that typing replaces it.

Graphic editors do this in three ways (research note 0027): Figma gives every new shape the same fixed values and leaves alike shapes to duplicating, components and styles; Inkscape gives each tool the values last used; Sketch and Affinity Designer let you make a shape's settings the default, by a command. E02 has no duplicating yet, so the agent chose Inkscape's way, which is also Figma's and Inkscape's for a text: what the panel shows is what the next shape gets. To confirm at VAL-036.

| Term | In plain words |
| --- | --- |
| kind | rectangle, polygon, round shape or text: each keeps its own last settings |
| starting values | the settings a new shape has before you change any |
| on integers | the click's point may fall between two pixels once the view is scaled: the shape's corner is rounded to the nearest whole number, a half rounded up, so (250.5, 140.4) gives a shape at (201, 90) |
| selector | the arrow tool, pressed when no shape tool waits for a click |
