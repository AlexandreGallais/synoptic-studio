# F03 — A rounding for each corner

## What you can do now

Each corner of a rectangle or a regular polygon can have its own **rounding** (its corner radius). In the playground, the large **Points** button shows the corners of the shape as round **handles**. Click one, or its row in the list of points, then type a radius: only the corners you selected are rounded. The **Shape** button goes back to the whole shape, where the Radius field rounds every corner, as before.

## The ideas behind it

**A corner of its own.** A square with only its top-left corner rounded is now a shape like any other: the other three corners stay sharp.

![A 100 × 100 square, only its top-left corner rounded with radius 40](./images/f03-one-corner.svg)

**Point 1 and point 2.** The outline of a shape starts at a corner and goes around it clockwise. In Points mode, the corner where it starts is **red** (point 1) and the next one **green** (point 2), so that you can read where it starts and which way it goes. The list of points follows the same order.

![The hexagon in Points mode: point 1 red at the left end of the top edge, point 2 green at its right end, the others dark](./images/f03-hexagon-points.svg)

**Choosing corners.** A click selects one point. Ctrl + click (Cmd on a Mac) adds or removes one. In the list, Shift + click selects every point between the last one clicked and this one; on the drawing, Shift + click adds or removes one point, as there is no "between" on a drawing. Here points 1 and 4 got a radius of 20.

![A hexagon with points 1 and 4 rounded, the others sharp](./images/f03-hexagon-two-corners.svg)

**"Mixed".** When the corners the Radius field speaks for do not all have the same rounding, it is empty and reads **Mixed**. Typing a number gives it to all of them. With no point selected in Points mode, it is greyed and reads **Select points**.

**Too big for its corner.** As in F01, a rounding too big for its corner is reduced to fit, and your number is kept. Only the corners next to a short side are reduced. In a 100 × 20 rectangle, a radius of 30 on the top-left corner and 10 on the top-right gives 20 and 10: the left side is only 20 high. The list of points shows both numbers: `radius 30 (effective 20)`.

![A 100 × 20 rectangle: the top-left corner rounded with 20 (30 asked), the top-right with 10](./images/f03-reduced.svg)

**Fewer or more corners.** Each point keeps its rounding by its number: a hexagon with point 1 rounded becomes a pentagon with point 1 still rounded. A point that goes away takes its rounding with it, and a new point is sharp.

| Term             | In plain words                                                                      |
| ---------------- | ----------------------------------------------------------------------------------- |
| Point            | a corner of the shape                                                               |
| Handle           | the round mark drawn on a point in Points mode, to click it                         |
| Rounding, radius | how much a corner is rounded: the radius of the circle drawn in it                  |
| Mixed            | the selected points do not all have the same rounding                               |
| Effective radius | the rounding actually drawn, smaller when the corner is too small for the one asked |

## Try it

Run `npm run dev` and open `http://localhost:5173/?step=29`: the **Guided test** opens on the first step of F03 (steps 29 to 33). **Next** types the shape for you; the clicks on the points are yours, as each step says. The same steps are listed in the test cards of [US-012](../backlog/stories/E01-F03-US-012-points-mode.md#product-owner-test) and [US-013](../backlog/stories/E01-F03-US-013-points-list-selection.md#product-owner-test).

## Limits

- The points cannot be moved, added or deleted yet: that is node editing, F08, which also turns a rectangle or a polygon into a free contour by a deliberate action.
- When the number of corners goes down then up, the points that went away do not get their rounding back (decided at the refinement; a "memory" is noted for later, see below).
- The playground is a demonstration: the tools of the editor applications (drawing, selecting shapes, points) come with them.

## Your feedback

Said by the Product Owner during `VAL-004` (2026-10-11), quoted by the agent: « C'est bien, j'ai bien mon premier point et tout. Ça, c'est trop cool. Comme ça, c'est le sens. J'adore. »

One remark, kept for later: a polygon could remember the rounding of the points that went away — 12 corners with different roundings, then a triangle, then 12 again would get the 9 roundings back. It changes a rule decided at the refinement (Q26).
