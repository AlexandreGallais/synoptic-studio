---
id: US-033
epic: E15
feature: F14
title: Put a text in a box with the Text tool
status: ready
points: 3
---

# E15 · F14 · US-033 — Put a text in a box with the Text tool

As a designer, I want to put a long text in a box of given width and height, wrapped, cut with an ellipsis and aligned in both directions, so that a label or an explanation fits its frame in a synoptic view.

## Acceptance criteria

- Given the Text tool, when **Box** is chosen with a width and a height, then the text wraps in the box, the frame shows the box, and the SVG still holds one `<text>` with one positioned `<tspan>` per line shown, without `text-anchor` nor `dominant-baseline` (ADR-0052, `DERIV-text-box`).
- Given "Hello World" in Inter at size 16 in a box whose width leaves room for "Hello" only, when drawn, then "World" is on a second line; given **Max lines** = 1, then one line ending with "…" is drawn (F14.AC1, F14.AC2).
- Given **Long words** = **Break** or **Move whole**, when a word wider than the box is typed, then it breaks at a character, or stays whole on its line and ends with "…" (F14.AC1).
- Given **Vertical alignment** top, middle or bottom, **Line height** automatic or a number, **Wrap** automatic or balanced, and **Trim**, when chosen, then the lines move as `DERIV-text-box` steps 7 to 9 say (F14.AC3, F14.AC5).
- Given **Fit content**, when chosen again, then the text draws exactly as in F05, and every shape of E01 and E14 draws as before (F14.AC6).
- Given a text in a box turned by 90°, when selected, then its box is shown, turned about its middle.

## Product Owner test

Inter 4 installed, as for VAL-014; otherwise the frame does not hug the letters. Open `http://localhost:5173/` and click **Text**.

| Step | Do                                                                                    | You should see                                                                                                                           |
| ---- | ------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------- |
| 1    | Type **Content** "Hello World", choose **Box**, **Width** 50, **Height** 60           | "Hello" over "World"; the frame is the box, 50 × 60                                                                                      |
| 2    | Type **Max lines** 1                                                                  | one line, ending with "…"                                                                                                                |
| 3    | Type **Max lines** 0, **Content** "Pump P-101 is running", **Long words** = **Break** | "running" breaks at a character, the end on the next line, no hyphen                                                                     |
| 4    | Choose **Long words** = **Move whole**                                                | "running" stays on its line, cut with "…"                                                                                                |
| 5    | Choose **Vertical alignment** = **Middle**, then **Bottom**                           | the block of lines moves to the middle of the box, then to its bottom                                                                    |
| 6    | Type **Line height** 24, then 0                                                       | the lines spread apart, then come back to the font's line height                                                                         |
| 7    | Tick **Trim**                                                                         | the first line's capitals touch the box's top                                                                                            |
| 8    | Type **Width** 110, choose **Wrap** = **Balance**                                     | "Pump P-101" over "is running" instead of "Pump P-101 is" over "running" (in Inter the cut may differ: look for lines of closer lengths) |
| 9    | Type **Angle** 90                                                                     | the text and its box turn about the box's middle                                                                                         |
| 10   | Choose **Fit content**                                                                | the text is on one line again, as in F05                                                                                                 |

## Tasks

One task = one commit, referenced as `US-033.Tn`.

- [ ] T1 — The box's fields in the Text tool: Fit content or Box, width, height, long words, max lines, line height, vertical alignment, wrap, trim (3 h)
- [ ] T2 — The drawing of the lines shown in the box, the frame and the turn about the box's middle (2 h)
- [ ] T3 — The Product Owner test card checked end to end (30 min)
