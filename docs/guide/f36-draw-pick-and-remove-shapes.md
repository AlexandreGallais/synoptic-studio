# F36 — Draw, pick and remove shapes

## What you can do now

Choose the Rectangle, Polygon, Round shape or Text tool, press the drawing and drag: the new shape follows the pointer, from the point pressed to the pointer, in any direction, and stays where you release it, at the size dragged. A text dragged sits in a box of that size, Fit set to Box. Hold Shift and the box stays square, so that a round shape is a circle; pressing or releasing Shift during the drag changes the shape at once. A text is never made square. Released where you pressed, the drag is a click: the shape is placed 100 × 100 centred on the point, as in F35. The rest of this page is written at F36's validation (VAL-037).

## The ideas behind it

**The box's corners land on whole numbers.** The point pressed and the point released are each rounded to the nearest whole number, a half rounded up, as a click is; the box runs between them, so a drag from (130, 90) to (10, 10) gives the same rectangle 120 × 80 at (10, 10) as a drag from (10, 10) to (130, 90). With Shift, the side is the larger of the two distances dragged, laid from the point pressed toward the pointer.

**A drag straight down or straight across keeps its width or height of 0.** A shape may have a side of 0, so the agent chose to keep it as drawn: nothing shows until a side is typed, but the panel edits it. Placing it as a click, adding nothing, or making the side 1 were the other ways. To confirm at VAL-037.

| Term | In plain words |
| --- | --- |
| drag | press the drawing, move the pointer with the button held, release |
| box | the rectangle a shape fills, from its top-left corner, with its width and height |
| Shift | held while dragging, keeps the box square, as in Figma |
