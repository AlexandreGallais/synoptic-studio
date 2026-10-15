---
id: US-039
epic: E15
feature: F14
title: See the straight box of a turned text
status: ready
points: 2
---

# E15 · F14 · US-039 — See the straight box of a turned text

As a designer, I want a turned text to show its straight box, the smallest upright rectangle around it, with its x, y, width and height, as a shape does, so that I know the room a turned label takes on the drawing.

Product Owner decision of 2026-10-15, from a try of the playground before VAL-015: a turned text showed only its turned frame, while shapes also show their straight box, as SVG's `getBBox`. Rules: `shapes.md` (Rotation of shapes: two boxes; Text, Turning). The box is taken around the text's turned block or box, not its letters' outlines (agent's reading, to confirm at VAL-015).

Built on the latest branch of the chain, `feature/f15-fonts-html`, after US-036 and EN-042 (a turned text kept in place when it grows).

## Acceptance criteria

- Given a text whose block is 40 × 20, fitting its content at (0, 0), turned by 45 about its middle (20, 10), when selected, then its straight box is at (−1.21320, −11.21320), 42.42641 × 42.42641: its corners are (12.92893, −11.21320), (41.21320, 17.07107), (27.07107, 31.21320) and (−1.21320, 2.92893) (`DERIV-turned-shape`, step 4).
- Given a text in a box 60 × 40 at (0, 0), turned by 90 about (30, 20), when selected, then its straight box is at (10, −10), 40 × 60.
- Given a text at 0°, when selected, then its straight box is its block or its box, drawn as before.
- Given an empty text, or a box 0 wide or 0 tall, then no straight box is shown.
- Given a turned shape, then its straight box is drawn as before.

## Product Owner test

Open `http://localhost:5173/`, click **Text**, click on the drawing, type `Pump P-101`, choose the size 16 and the weight Regular, set X 47 and Y 61.

| Step | Do                                  | You should see                                                                                                               |
| ---- | ----------------------------------- | ---------------------------------------------------------------------------------------------------------------------------- |
| 1    | Set the angle to 45                 | the turned dashed frame, around it an upright dashed box; under X and Y: Straight box x 53.03586, y 32.73899, 75.8814 square |
| 2    | Set the angle to 90                 | the upright box as tall as the text is long: x 81.29688, y 26.70313, width 19.35938, height 87.95313; X and Y still 47, 61   |
| 3    | Choose **Box**, width 60, height 40 | an upright box 40 × 60 around the turned box: x 60.65625, y 26.70313, width 40, height 60                                    |
| 4    | Set the angle to 0                  | one dashed frame only, and no straight box line, as before                                                                   |
| 5    | Click **Rectangle**, angle 90       | the turned rectangle's straight box and its line, as before                                                                  |

In the guided test (`?guide=epic`), the step **F14 · The straight box of a turned text (US-039)** walks through step 3.

## Tasks

One task = one commit, referenced as `US-039.Tn`.

- [x] T1 — The straight box of a turned text from its block or box and its turn, with the turned corners the shapes use (`turnedCorners`, `@see DERIV-turned-shape`); tests first, the cases above (2 h)
- [x] T2 — The playground draws it and shows its x, y, width and height as for a shape; the guided step and the test card; the playground's tests first (2 h)
