# F12 — Points on a curve, with handles

## What is new

A point of a free shape can bend its two sides with two **handles**, as in Figma: arrows leaving the point that pull its sides towards them.

| Option                                   | What it does                                                                     |
| ---------------------------------------- | -------------------------------------------------------------------------------- |
| **Handles** switch                       | a point turns from its corner radius to its handles and back; the radius is kept |
| **Typed as** x, y on the page            | the handle's end, as in Figma                                                    |
| **Typed as** angle, length from the side | the same values give the same bend on any side: copy them from corner to corner  |
| **Mode** Mirror angle and length         | the other handle opposite, same length (default)                                 |
| **Mode** Mirror angle                    | the other handle opposite, its own length                                        |
| **Mode** No mirroring                    | each handle alone                                                                |
| A corner beside a curve                  | keeps its rounding; a radius too large is drawn smaller, never sharp             |
| A polygon's point                        | **Handles** converts the polygon to a free shape first, unmoved                  |
| Adding or deleting a point on a curve    | the curve is split, or kept to the next point                                    |

Rules: [domain, "Points with handles"](../domain/shapes.md#points-with-handles-f12); decisions: [ADR-0046](../adr/0046-cubic-sides-in-symbols.md), [ADR-0047](../adr/0047-fillet-beside-a-curve.md).

## How to try it

`npm run dev`, then `http://localhost:5173`: nine steps, one per option; **Next** types the rectangle 120 × 80, radius 20.

Example values for the angle from the side: point 1 **After** 330 and 40, point 2 **Before** 330 and 40 — a symmetric arch over the top side.

## Validated

Validated by the Product Owner on 2026-10-14, every option as built:

| Question                                             | Decision                                                                       |
| ---------------------------------------------------- | ------------------------------------------------------------------------------ |
| Q33 — points on one line whose sides bulge (a lens)  | refused, as today                                                              |
| Q34 — a point added on a curve beside a large fillet | split at the middle of the whole curve; the fillet shrinks                     |
| The reduced radius beside a curve                    | as built: 47.7 for a radius 60 beside handles of 40, 46.9 beside handles of 80 |
