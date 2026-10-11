# F08 — Moving, adding and deleting points

## What you can do now

In the playground, in **Points** mode, the points of a shape can now be edited one by one:

- type the **X** or the **Y** of the point selected, then press Enter: it moves there;
- with two neighbouring points selected, **Add a point in the middle** adds one between them;
- **Delete points**, or the delete key, removes the points selected.

A rectangle can be edited straight away. A regular polygon is first turned into a **free shape** with **Convert to free shape**: it does not move, but it loses its number of corners and its anchor.

## The ideas behind it

**Converting does not move anything.** The hexagon keeps its exact points, decimals included (25, 6.69873…), so that it stays exactly where it was. Here point 1 (red) was then moved to x 80; the dashed outline is the hexagon before.

![A converted hexagon, its point 1 moved right to x 80; the original hexagon dashed](./images/f08-hexagon-moved.svg)

**Only what you type is a whole number.** Typing X 80 on that point leaves its Y at 93.30127: nothing is rounded behind your back. Width and height still stretch the whole shape, every point following, and moving a point after a stretch starts from where the points are drawn.

**A point in the middle.** The new point lands exactly in the middle of the side, without rounding, so the drawing does not change; it is selected alone, ready to be moved. Here it was moved up by 20 to make a peak.

![The 120 × 80 rectangle with a point added in the middle of its top side and moved up by 20](./images/f08-midpoint-peak.svg)

**Deleting joins the neighbours.** The two points around a deleted one are joined by a straight side. A shape keeps at least 3 points: otherwise Delete is greyed and a line says why.

![The rectangle with its top-right point deleted: a triangle](./images/f08-deleted-point.svg)

**The rounding follows its point.** Each point carries its own rounding when it moves; a point added in the middle is sharp. In Points mode a Radius field of the points rounds the points selected, while the shape's Radius field, shown in both modes, rounds every corner.

| Term           | In plain words                                                                    |
| -------------- | --------------------------------------------------------------------------------- |
| Free shape     | a shape whose points are edited one by one; the rectangle is one from the start   |
| Convert        | turn a regular polygon into a free shape, without moving it                       |
| Neighbours     | two points joined by a side; the last and the first are neighbours too            |
| Point selected | the point clicked in the list or on the drawing; X and Y show when exactly one is |

## Try it

Run `npm run dev` and open `http://localhost:5173/?step=39`: the **Guided test** opens on the first step of F08 (steps 39 to 46). **Next** types the shape for you; the clicks on the points are yours, as each step says. The same steps are in the test cards of [US-014](../backlog/stories/E01-F08-US-014-convert-and-move.md#product-owner-test), [US-015](../backlog/stories/E01-F08-US-015-add-middle-point.md#product-owner-test) and [US-016](../backlog/stories/E01-F08-US-016-delete-points.md#product-owner-test).

## Limits

- One point moves at a time. Moving several selected points together, by typing `+10`, and simple arithmetic in every number field are noted for the editor applications.
- Points are typed, not dragged: dragging comes with the editor applications (E02).
- Two points at the same place, points on one slanted line, or sides that cross are allowed and may draw nothing; the warnings you asked for come with the editor applications (Q29).

## Your feedback

Said by the Product Owner during `VAL-005` (2026-10-11), quoted by the agent: « Putain, c'est génial, ça marche de fou. J'adore, j'adore ça. »

On deleting: « Je peux delete le point. Oh putain, c'est fait un triangle. C'est trop trop bien. » On a point moved after a stretch: « ça va mettre à jour tous les points. C'est génial, c'est exactement ça que je voulais. » On decimals: « Il reste à la virgule. C'est trop de la balle. \[…\] C'est non destructif. » These settled Q28 (ADR-0034).

Remarks kept for later: moving three points at once by `+10` (« on ne peut pas modifier trois points en même temps, leur position »), and arithmetic in the fields, as in a spreadsheet, with parentheses (« dans les inputs, ce qui pourrait être craft cool, c'est de pouvoir faire des maths »). Acted on at once: the points have a Radius field of their own, and the shape's Radius field stays shown in Points mode (« pour moi, le radius, ce serait le radius commun dans shape, et puis tu aurais un autre truc radius dans point »).

After a second look: a field lost the focus at each key (fixed: typing goes on, and the arrow keys repeat). Two points at the same place should not be refused nor merged, as some editors do, but flagged in the list of points, asking whether to delete one — a shape may pass several times at the same place on purpose (« je ne veux pas que ce soit destructif comme Figma »); points on one slanted line are not refused (Q29).
