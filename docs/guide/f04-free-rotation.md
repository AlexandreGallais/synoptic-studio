# F04 — Turning a shape by any angle

## What you can do now

In the playground, the **Angle and position** fields turn the shape you drew by any whole number of degrees, and move it by its x and y. The **−90°** and **+90°** buttons turn it by a quarter turn. A turned shape can still be resized, and its points moved, added or deleted, without jumping elsewhere.

## The ideas behind it

**It turns about the center of its box, clockwise.** A positive angle turns the shape clockwise on screen, as SVG does. The angle is kept from 0 to 359: typing 370 gives 10, typing −90 gives 270. Here the 120 × 80 rectangle (dashed) turned by 90° about the red dot, the center of its box.

![The 120 × 80 rectangle, dashed, and the same rectangle turned by 90° about its center](./images/f04-rectangle-turned.svg)

**Nothing of the shape changes.** Width and height stay those of the shape before it turned, as in Figma; the rounded corners, the anchor and the numbers of the points do not change. Back to 0°, the shape is exactly the straight one.

**Two boxes.** The **turned box** is the shape's own box, turned with it: the thin blue frame, the selection, so that a turned square does not seem to grow. The **straight box** is the smallest upright rectangle around the drawn shape, rounded corners included, as a browser measures it: drawn dashed, it grows at 45°.

**Where the shape is.** The **X** and **Y** fields are those of the straight box, as in Inkscape: typing them moves the shape. Turning keeps the center of the box in place, so a square turned by 45° gets smaller x and y.

**The box turns with its content.** A polygon is placed in its box by its anchor first, then the box turns: a triangle whose base touches the bottom of its box has that base on the left at 90°. A square turned by 45° is the ISA diamond; a hexagon turned by 30° stands on a vertex, as in IEC symbols.

![A triangle anchored at the bottom of its grey box, dashed, then turned by 90°: its base on the left](./images/f04-triangle-turned.svg)

**Editing a turned shape keeps it in place.** A width typed keeps the shape's turned top-left corner where it is, as Figma; a point typed in the SVG's x and y moves only that point; adding or deleting a point, or converting a turned polygon to a free shape, moves nothing else.

| Term | In plain words |
| --- | --- |
| Angle | how far the shape is turned, in whole degrees, clockwise on screen, from 0 to 359 |
| Turned box | the shape's box turned with it, drawn as a thin blue frame |
| Straight box | the smallest upright rectangle around the drawn shape, dashed; its x and y are the shape's position |
| Quarter turn | 90° |

## Try it

Run `npm run dev` and open `http://localhost:5173/?guide=epic&step=47`: the **Guided test** opens on the first step of F04 (steps 47 to 57). **Next** types the shape for you, straight; the angles and the clicks are yours, as each step says. The same steps are in the test cards of US-018 and US-019.

## Limits

- The shape turns about the center of its box only. The rotation mode — a center you move, snapped to a point of the shape or of another shape, handles on both boxes — comes with the editor applications (E02).
- The playground's view stays put: a turned shape may leave the canvas.
- Turning by an animation, from a property's value, comes with E35.

## Validation

Validated by the Product Owner on 2026-10-12 (`VAL-007`), after a first free try and the guided test: turning, moving and resizing a turned shape work together as they meant.

| Remark | What was done |
| --- | --- |
| the shape did not seem to turn about its center: the view followed a turned shape into negative coordinates | fixed at once: the view stays put; a shape may leave it |
