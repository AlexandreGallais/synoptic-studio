---
id: US-032
epic: E15
feature: F05
title: Turn a text and see its measured block
status: done
points: 2
---

# E15 · F05 · US-032 — Turn a text and see its measured block

As a designer, I want to turn a text about the middle of its block and see that block, so that a label reads along a tank and I know where it ends.

## Acceptance criteria

- Given a text turned by 90, when drawn, then it turns clockwise about the middle of its measured block by a `transform` on its `<text>`, and its angle is kept from 0 to 359 (ADR-0052, `REF-SVG11-TRANSFORM`, `DERIV-turned-shape`).
- Given a selected text, when shown, then its measured block (`DERIV-text-width`, step 4) is drawn as a dashed frame, as a shape's box.

## Tasks

One task = one commit, referenced as `US-032.Tn`.

- [x] T1 — The turn about the block's middle and the block's frame (2 h)
- [x] T2 — The Product Owner test card (30 min)
- [x] T3 — CHK-014's two points: a font without metrics, the fallback's name checked (1 h)
- [x] T4 — The review's findings: each tool keeps its own placement, citations (1 h)

## Decisions taken (2026-10-15)

- **The turn on the group.** The `transform="rotate(α cx cy)"` is written on the `<g>` that holds the `<text>` and its underline and strike-through paths, so that the strokes turn with the letters; the first criterion's "a `transform` on its `<text>`" is met through that group (provisional, to confirm at VAL-014). A text at 0° carries no `transform`.
- **The middle** is `boxCenter` of the measured block at the text's position (`DERIV-turned-shape` step 4), kept in `DrawnText.center`; the transform is written by `formatRotation` (`REF-SVG11-TRANSFORM`, §7.6). The angle is checked by `isShapeAngle`, as a shape's.
- **The frame.** A selected text's block is drawn by the route of a shape's straight box (`createStraightBoxElement`: dashes as thin filled rectangles), in a `<g>` carrying the text's rotation, so that it turns with the text; an empty block draws none.
- **CHK-014's two points** (ADR-0055): a font without metrics is measured with its fallback family's face, when read, else with the default font's, as the code did since EN-032; ADR-0052's decision 1 is amended. The fallback's name is checked as the font's (`isFontName` in `isValidText`).
- **The playground.** In Text mode the angle row (field, −90° and +90°) is shown; the shapes' modes and straight box stay hidden.
- **Each tool keeps its own placement** (audit finding, a playground behaviour): the Text tool and the shapes each keep their X, Y and angle, and the shapes their position; going from one to the other shows the other's values, a text not yet placed starting at X 0, Y 0, straight. A text's angle never turns the shape, nor the other way round (`saveToolPlacement`, `writeToolPlacement`).

## Product Owner test

Before starting: Inter 4 installed on the computer, as for US-031; without it the frame does not hug the letters, every step still works.

Open `http://localhost:5173/`, click **Text** in the toolbar, and type in **Content** "Hello", a line break, "World" (size 32, weight 400, X 0, Y 0, as the page opens). The **Angle** row (field, −90° and +90°) is shown; the Shape/Points buttons, the size and the corners stay hidden. The text sits in a dashed grey frame, its measured block: 88.98438 wide, 77.4375 tall (Inter: two lines of 2478 × 32 / 2048 = 38.71875).

1. Type **Angle** = 90: the text and its frame turn a quarter turn clockwise, together, about the middle of the frame, which stays in place. Click **Pipeline**: in **SVG output**, the `<g>` reads `transform="rotate(90 44.49219 38.71875)"` (half of 88.984375, half of 77.4375).
2. Tick **Underline** and **Strike-through**: the strokes turn with the letters, still along each line. Untick both.
3. Type **X** = 100, **Y** = 50: the text and its frame move together, still turned; the transform reads `rotate(90 144.49219 88.71875)`.
4. Type **Angle** = 45 and press Enter: the text leans halfway, the frame with it, about the same middle.
5. Type **Angle** = 370 and press Enter: the field shows 10. Type −90 and press Enter: 270. Type 360 and press Enter: 0, the text straight, with no `transform` in **SVG output**.
6. Click **+90°**, then **−90°** twice: 90, then 270.
7. Type **Size** = 48 and **Angle** = 90: the block grows to 133.47656 × 116.15625 and the text turns about its new middle, `rotate(90 166.73828 108.07813)` at X 100, Y 50.
8. Type **Angle** = 1.5: the drawing goes, the message under the fields says the angle must be an integer, the fields are outlined in red. Type 0: the text comes back.
9. Empty **Content**: nothing is drawn, no frame.
10. Type **Content** "Hello", a line break, "World" again, **Size** = 32, **Angle** = 90 and **X** = 0, **Y** = 0. Click **Rectangle**: the fields show the rectangle's own X, Y and angle, and the rectangle is not turned by the text's 90. Type **Angle** = 30 for it. Click **Text**: the fields show 0, 0 and 90 again and the text turns as before, `rotate(90 44.49219 38.71875)`; the rectangle's 30 did not turn it. Click **Rectangle**: 30 again, its turned box and dashed straight box as before.

What to look at closely: the frame turning with the letters without drifting from them (step 1), the underline turning with its line (step 2), and whether the dashed frame reads well as the selection of a text (the look of a turned shape's straight box). The library turns "Hello" over "World" in Arial 16 without kerning about (20.89063, 18.39844), which the tests check; the playground measures every font with Inter until fonts are read from files (F15).
