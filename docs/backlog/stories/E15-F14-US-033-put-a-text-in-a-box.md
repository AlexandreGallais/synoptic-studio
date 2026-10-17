---
id: US-033
epic: E15
feature: F14
title: Put a text in a box with the Text tool
status: done
points: 3
---

# E15 · F14 · US-033 — Put a text in a box with the Text tool

As a designer, I want to put a long text in a box of given width and height, wrapped, cut with an ellipsis and aligned in both directions, so that a label or an explanation fits its frame in a synoptic view.

## Acceptance criteria

- Given the Text tool, when **Box** is chosen with a width and a height, then the text wraps in the box, the frame shows the box, and the SVG still holds one `<text>` with one positioned `<tspan>` per line shown, without `text-anchor` nor `dominant-baseline` (ADR-0052, `DERIV-text-box`).
- Given "Hello World" in Inter at size 16 in a box whose width leaves room for "Hello" only, when drawn, then "World" is on a second line; given **Max lines** = 1, then one line ending with "…" is drawn (F14.AC1, F14.AC2).
- Given **Long words** = **Break** or **Move whole**, when a word wider than the box is typed, then it breaks at a character, or stays whole on its line, ending with "…" when **Ellipsis** is ticked and drawn whole when it is not (F14.AC1).
- Given **Vertical alignment** top, middle or bottom, **Line height** automatic or a number, **Wrap** **Automatic** or **Balance**, and **Trim**, when chosen, then the lines move as `DERIV-text-box` steps 7 to 9 say (F14.AC3, F14.AC5).
- Given **Fit content**, when chosen again, then the text draws exactly as in F05, and every shape of E01 and E14 draws as before (F14.AC6).
- Given a text in a box turned by 90°, when selected, then its box is shown, turned about its middle.

## Product Owner test

Inter 4 installed, as for VAL-014; otherwise the frame does not hug the letters. Open `http://localhost:5173/` and click **Text**. The widths below are Inter 4.1's at size 16, regular, kerning included, as the library measures them (line height 19.35938, cap height 11.64063); a browser may draw them a fraction apart, never on other lines. The page opens the box at 200 × 100, a playground starting value that shows a few lines at once; the model's default stays 0 × 0.

| Step | Do | You should see |
| --- | --- | --- |
| 1 | Type **Size** 16, **Content** "Hello World", choose **Fit** = **Box**, then **Width** 50, **Height** 100 | the box's fields appear; "Hello" (38.6 wide) over "World" (44.5); the dashed frame is the box, 50 × 100 |
| 2 | Type **Max lines (0 none)** 1 | one line, "Hell…" (42.8): "Hello…" (52.4) is wider than the box |
| 3 | Type **Max lines** 0, **Content** "Pump P-101 is running"; **Long words** stays **Break** | five lines: "Pump", "P-101", "is", "runnin", "g"; no hyphen; the box, 100 tall, holds all five (96.8) |
| 4 | Choose **Long words** = **Move whole** | four lines: "Pump", "P-101", "is", "runn…" (48.2) |
| 5 | Untick **Ellipsis** | the last line is "running" whole (57.5), past the box's right edge; tick **Ellipsis** again |
| 6 | Choose **Vertical alignment** = **Middle**, then **Bottom** | the four lines (77.4 tall) move down by 11.3, then by 22.6, the last one's descent on the box's bottom |
| 7 | Choose **Vertical alignment** = **Top**; type **Line height (0 auto)** 24, then 0 | the lines spread apart, 24 from baseline to baseline, then come back to 19.4 |
| 8 | Tick **Trim** | the lines move up: the first line's capitals touch the box's top, its baseline 11.6 below it |
| 9 | Type **Width** 110, then choose **Wrap** = **Balance** | "Pump P-101 is" (104.8) over "running", then "Pump P-101" (88.0) over "is running" (74.4) |
| 10 | Type **Angle** 90 | the text and its box turn together about the box's middle, (55, 50) |
| 11 | Type **Angle** 0, untick **Trim**, choose **Fit** = **Fit content** | the box's fields disappear; the text is on one line again (166.8 wide), its frame hugging it, as in F05 |
| 12 | Choose **Fit** = **Box** and empty **Width** | nothing is drawn, the message says why, the fields are outlined; choose **Fit content**: the box's fields stay shown while Width is refused; type 110 and the text comes back |

## Tasks

One task = one commit, referenced as `US-033.Tn`.

- [x] T1 — The box's fields in the Text tool: Fit content or Box, width, height, long words, max lines, ellipsis, line height, vertical alignment, wrap, trim (3 h)
- [x] T2 — The drawing of the lines shown in the box, the frame and the turn about the box's middle (2 h)
- [x] T3 — The Product Owner test card checked end to end (30 min)
