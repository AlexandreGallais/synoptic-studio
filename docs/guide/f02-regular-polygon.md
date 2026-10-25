# F02 — A regular polygon with rounded corners

## What you can do now

In the playground, choose **Polygon** in the toolbar above the drawing and type a number of corners, from 3 to 12. You get the largest **regular** polygon — all its sides and all its corners equal — that fits the box of the width and height you typed, without being stretched, centered in it (F07 lets you choose another place), always with a flat side at the bottom. The corner radius works as for the rectangle: too big, it is reduced to fit, and your number is kept.

## The ideas behind it

**As large as the box allows, without stretching.** A hexagon in a 100 × 100 box is 100 wide but only 86.6 high: to touch the top and bottom too, it would have to be stretched, and it would no longer be regular. So it touches the width, and leaves a small margin above and below.

![A hexagon in a 100 × 100 box: it touches the left and right sides](./images/f02-hexagon.svg)

**A flat base, always.** A triangle points up, a square has horizontal sides — it is not a diamond. Another orientation (a diamond is a square turned by 45°) will come from free rotation, F04.

![A triangle in a 100 × 100 box: 100 wide, 86.6 high, centered](./images/f02-triangle.svg)

**It stays regular when the box changes.** A square in a 100 × 50 box is 50 × 50, centered across.

![A square in a 100 × 50 box: 50 × 50, from 25 to 75 across](./images/f02-square-in-100x50.svg)

**Rounded as much as possible, a polygon becomes the circle inside it.** With a radius far too big, every corner is rounded until its curves meet in the middle of the sides: the hexagon becomes its inscribed circle, of diameter 86.6. It no longer touches the box — remove the rounding and it touches it again (accepted at the refinement).

![The hexagon rounded at most: its inscribed circle](./images/f02-hexagon-incircle.svg)

The circle inside a triangle is smaller (radius 28.87), and lower than the middle of the box: it is centered on the triangle, not on the box.

![The triangle rounded at most: its inscribed circle, below the middle](./images/f02-triangle-incircle.svg)

**A small radius** rounds the corners and keeps the sides: an octagon with radius 10 touches all four sides of its box.

![An octagon with radius 10 in a 100 × 100 box](./images/f02-octagon.svg)

| Term | In plain words |
| --- | --- |
| Regular polygon | a shape with equal sides and equal corners; "equilateral" is the word for triangles |
| Box | the width × height you type; the polygon fits inside it |
| Flat base | a horizontal side at the bottom |
| Inscribed circle | the largest circle inside the polygon, touching every side in its middle |
| Requested radius | the number you type; it is kept in the model |
| Effective radius | the radius actually drawn; smaller when the shape is too small for the request |

## Sizes

One unit of the drawing is one pixel of the screen. The calculations have no upper limit, but the interface caps a width, height or radius typed above 100 000 at 100 000: even a wall of 8K screens stays far below (Q20). For a screen seen from far, draw at the screen's size and let the SVG scale.

## Try it

Run `npm run dev` and open `http://localhost:5173/?guide=epic&step=10`: the **Guided test** takes you through the ten steps of F02 (10 to 19), one at a time. **Next** types the values and chooses the shape for you. The same steps are listed in the test cards of US-007 and US-008.

## Limits

- Regular polygons only: a polygon stretched to fill its whole box came with F06.
- One radius for every corner: a radius per corner comes with F03.
- Only a flat base: other orientations come with free rotation (F04).
- From 3 to 12 corners (Q19): symbols use up to 8, and beyond 12 a polygon is hardly told from a circle.

## Validation

Validated by the Product Owner on 2026-10-09 (`VAL-002`): the polygon works as they meant.

| Idea raised along the way | Where it went |
| --- | --- |
| a polygon stretched to its box | F06 |
| the View Editor asks which screen the view targets | E38 |
| oversized SVG imports flagged and simplified | E42 |
