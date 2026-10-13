# F12 — Points on a curve, with handles

## What is new

A point of a free shape can now bend its two sides with two **handles**, as in Figma. A handle is an arrow leaving the point: the side starts in its direction, and the longer it is, the further the side follows it before turning towards the next point.

- A switch per point: **corner radius** or **handles**. The radius is kept under the handles and comes back as it was.
- Four **modes** for the other handle when one is typed: Mirror angle and length (the default), Mirror angle, No mirroring, Smooth by a tension.
- Seven **methods** to type a handle, to try one by one and choose from (Q32).
- The shape's width and height are those of its **curve**, as in Figma.

Rules: [domain, "Points with handles"](../domain/shapes.md#points-with-handles-f12); decision: [ADR-0046](../adr/0046-cubic-sides-in-symbols.md).

## How to try it

`npm run dev`, then `http://localhost:5173`: the guided test of F12 opens, 13 steps, each saying what to do and what to look at; **Next** types its values.

| Step | Tries                                      |
| ---- | ------------------------------------------ |
| 1    | the switch radius / handles                |
| 2–4  | the three mirror modes                     |
| 5–10 | the methods: dx, dy; angles; shares; paste |
| 11   | the tension                                |
| 12   | a point added on a curve                   |
| 13   | a point deleted next to a curve            |

Step 2 is the method x, y on the page.

## What to decide

| Question                                                                     | Options                                     |
| ---------------------------------------------------------------------------- | ------------------------------------------- |
| Q32 — which methods of typing a handle to keep                               | one, several, or all seven                  |
| Q33 — a shape whose points lie on one line but whose sides bulge (a lens)    | allowed, or refused as today                |
| ADR-0046 says handles are typed as integers; pasted and computed ones aren't | keep their decimals (as now), or round them |

## Words

| Word    | In plain words                                                                        |
| ------- | ------------------------------------------------------------------------------------- |
| handle  | an arrow from a point that pulls the side towards it                                  |
| cubic   | the kind of curve a side becomes: it starts along one handle and ends along the other |
| tension | how tight a smoothed point is: 0 the roundest, 100 straight sides                     |
| paste   | a curve copied from an SVG file (`C …`) or from CSS (`cubic-bezier(…)`)               |
