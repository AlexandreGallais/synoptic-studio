# F36 — Draw and pick shapes

## What you can do now

Choose the Rectangle, Polygon, Round shape or Text tool, press the drawing and drag: the new shape follows the pointer, from the point pressed to the pointer, in any direction, and stays where you release it, at the size dragged. A text dragged sits in a box of that size, Fit set to Box. Hold Shift and the box stays square, so that a round shape is a circle; pressing or releasing Shift during the drag changes the shape at once. A text is never made square. Released where you pressed, the drag is a click: the shape is placed 100 × 100 centred on the point, as in F35. With the Selector, pass the pointer over the drawing: the shape under it is outlined in blue, the top one where shapes overlap, and the outline goes when the pointer leaves it; the outline is never in the SVG exported. With the Selector, click a shape: it is outlined as selected and the panel edits it, while it stays where it was among the others, under the shapes drawn after it; the next shape you add goes on top of all. A rectangle comes back as a Rectangle with its radius, and a text with its whole style, its font chosen in the list, so that the next text you type is in that style. Click the empty drawing and none is selected: the panel says so until you click a shape or add one. The rest of this page is written at F36's validation (VAL-037).

## The ideas behind it

**The box's corners land on whole numbers.** The point pressed and the point released are each rounded to the nearest whole number, a half rounded up, as a click is; the box runs between them, so a drag from (130, 90) to (10, 10) gives the same rectangle 120 × 80 at (10, 10) as a drag from (10, 10) to (130, 90). With Shift, the side is the larger of the two distances dragged, laid from the point pressed toward the pointer.

**A drag straight down or straight across keeps its width or height of 0.** A shape may have a side of 0, so the agent chose to keep it as drawn: nothing shows until a side is typed, but the panel edits it. Placing it as a click, adding nothing, or making the side 1 were the other ways. To confirm at VAL-037.

**Only what is painted is under the pointer.** The shape under the pointer is the last one drawn whose painted area holds it, computed from the shapes themselves, never read back from the page: the cut corner of a rounded rectangle, or the inside of a circle's box outside the circle, picks nothing. A text counts anywhere in its block, or in its box when it sits in one, so that the room between its words and lines picks it too; the agent chose this, its letters' outlines and its backgrounds being the other ways. To confirm at VAL-037.

**A shape of width or height 0 is reached within 4 pixels.** It paints nothing, so the agent chose to find it when the pointer is within 4 pixels of the screen of its outline, whatever the zoom. Never picking it, or widening it to a side of 1, were the other ways. To confirm at VAL-037.

**The outline follows the shape's own edge.** The shape under the pointer is outlined by a line 2 pixels wide along its edge as drawn, rounded corners and curves included, a text along its block; the agent chose this, outlining its box or tinting it being the other ways. It shows only with the Selector, since with a tool a click places a shape rather than picks one. It is drawn in a layer above the drawing which the pointer passes through, so the drawing and its export never hold it. To confirm at VAL-037. Since F43 the blue line lies over a white one 4 pixels wide, a thin white edge showing on each side, so that it shows on a blue shape too, as Inkscape draws its helps.

**A click picks only when it started with the Selector.** The click that ends a drag with a tool, or that places a shape, arms the Selector again, but never also picks the shape under the pointer: the agent chose to look at the tool chosen when the button was pressed. To confirm at VAL-037.

**The shape selected is outlined as the hover is.** The same blue line, 2 pixels wide along its edge, shows the shape the panel edits, even while a tool is chosen, as in Figma; the agent chose this, its box with handles coming with moving and resizing. To confirm at VAL-037.

**None selected hides the panel's fields.** After a click on the empty drawing, every shape stays drawn and the panel only says how to select or add one; the agent chose this. To confirm at VAL-037.

| Term | In plain words |
| --- | --- |
| drag | press the drawing, move the pointer with the button held, release |
| box | the rectangle a shape fills, from its top-left corner, with its width and height |
| Shift | held while dragging, keeps the box square, as in Figma |
