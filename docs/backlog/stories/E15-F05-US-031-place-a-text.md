---
id: US-031
epic: E15
feature: F05
title: Place a text with the Text tool
status: done
points: 3
---

# E15 · F05 · US-031 — Place a text with the Text tool

As a designer, I want a Text tool with its fields so that a symbol shows a label in the font, size, weight and color chosen.

## Acceptance criteria

- Given the Text tool, when content, font, size, weight, italic, color, opacity, spacing, underline, strike-through, case and direction are typed, then the drawing shows them, and the SVG holds one `<text>` with one `<tspan>` per line, each with its `x` and `y`, without `text-anchor` nor `dominant-baseline` (ADR-0052, `REF-SVG2-TEXT`, `REF-RESVG-SUPPORT-TABLE`).
- Given a text aligned left, center or right, when it has two lines of different widths, then each line's `x` places it in the block as the alignment says, and each `y` is the line's top plus the ascender times the size over the units per em (`DERIV-text-width`, step 4).
- Given underline or strike-through, when drawn, then each is a `<path>` rectangle from the font's `post` underline or `OS/2` strike-out position and thickness, not `text-decoration` (ADR-0052, `REF-OT-POST`, `REF-OT-OS2`).
- Given the bold and italic buttons, when they are pressed, then the weight becomes 700 and the style italic; the weight list offers 100 to 900 (`REF-CSS-FONTS-4`, §2.2).

## Tasks

One task = one commit, referenced as `US-031.Tn`.

- [x] T1 — The Text tool and its fields in the playground (3 h)
- [x] T2 — The drawing of the text: positioned lines and their attributes (2 h)
- [x] T3 — The Product Owner test card (30 min)

## Notes from EN-032's review (2026-10-15)

- **Spaces kept.** The library measures every space of a line, runs of spaces and spaces at its ends included; the SVG writer must keep them all, with `xml:space="preserve"` or a `white-space` that preserves them, or a reader collapses them and draws a narrower line than the measured block.
- **Letter spacing after the last character.** `DERIV-text-width` adds letter spacing between units only, `u − 1` times, as CSS Text 3 says; the CSS editors' draft now spreads it on both sides of each unit (`REF-CSSWG-10193`) and browsers differ. Measure in Chrome, Firefox and Safari whether a line with letter spacing ends with a trailing space, and record the result before the text's block is trusted for alignment.

## Decisions taken while building (2026-10-15)

Each is written in `docs/domain/shapes.md` (Text), provisional, to confirm at VAL-014.

- **Alignment and direction.** Left, center and right are the physical sides for both directions, as CSS's `left` and `right` are (`REF-CSS-TEXT-3`, §6.1; `REF-CSS-WRITING-MODES-4`, §6.3 and §6.4): a right-to-left text aligned left starts each line on the block's left edge. The other reading, left and right swapped for right-to-left as CSS's `start` and `end`, is the option to weigh at VAL-014.
- **Right-to-left lines.** With the initial `text-anchor`, `start`, SVG draws the right side of right-to-left text at the line's `x` (`REF-SVG2-TEXT`, §11.10.1.1), so each such line is written at its right edge, with `direction="rtl"` on the `<text>` (`DERIV-text-width`, step 5). resvg and Batik do not draw `direction` (`REF-RESVG-SUPPORT-TABLE`): there a right-to-left line would start at that edge and run off the block.
- **Font list and spaces** (ADR-0054, amending ADR-0052's decision 4). The font, its fallback, then `sans-serif`, each family once: `"Inter", sans-serif`. Every space is kept by `white-space="pre"` for SVG 2 readers and `xml:space="preserve"` for SVG 1.1 ones. A tab typed in a line is drawn as a space by an SVG 1.1 reader, while the library measures the font's tab glyph.
- **Font name.** Stored trimmed, refused with a control character such as a line break or a tab; the playground trims what is typed. The writer still escapes a control character by its hexadecimal code.
- **Colors.** The fill is on the `<text>` and on each decoration's `<path>`, the opacity written as a number from 0 to 1 (`REF-SVG2-FILL-OPACITY`).
- **Strokes.** A line with no length, or a stroke the font gives no thickness (0 or negative), draws no underline nor strike-through.
- **Negative widths.** A line tightened past nothing keeps its negative width and is placed by the same rule; the block stays at least 0 wide, so such a line may fall outside it.
- **Bold button.** Pressed at a weight of 700 or heavier; a click sets 400 when it is pressed, 700 otherwise.
- **Playground.** The text is measured with the default font's metrics (Inter, EN-031 AC5): its frame matches the drawing only where the browser has Inter; elsewhere the browser draws its `sans-serif` and the frame shows Inter's block.

## Steps for VAL-014

No headless browser is among the development packages, and none is added for one measure. In Chrome, Firefox and Safari:

1. **Letter spacing after the last character.** A `<text>` "HH" in Inter 16 with `letter-spacing="10"`, read with `getComputedTextLength()` (or `getBBox().width`): 23.78125 + 10 = 33.78125 when the spacing is only between the two letters, as the library measures (`DERIV-text-width`, step 3); 43.78125 when the browser adds it after the last letter too (`REF-CSSWG-10193`). A browser that adds it draws a line 10 wider than its block, so centered lines shift by 5. The result goes into this file and `DERIV-text-width`.
2. **Underline position.** Beside the library's underline, the same text with a native `text-decoration: underline`: the library reads `post`'s `underlinePosition` as the top of the stroke (`REF-OT-POST`); a browser that reads it as the stroke's middle draws its line half a thickness lower (0.55 at Inter 16). Note which reading each browser follows.

## Product Owner test

Before starting: Inter 4 must be installed on the computer (Font Book on macOS shows it). Without it, the browser draws another font and the frame no longer hugs the letters; every step below still works, only the hugging differs.

Open `http://localhost:5173/`, then click **Text** in the toolbar. The panel shows the text's fields; the canvas shows "Pump P-101" over "Running" in a thin blue frame, the measured block. The size, corners, angle row and Shape/Points buttons are hidden.

1. Type **Size** = 48: the text and its frame grow together. Click **Pipeline**: in **SVG output**, the first `<tspan>` has `y="46.5"` (Inter's ascender, 1984 × 48 / 2048) and `x="0"`.
2. Choose **Alignment** = **Center**, then **Right**: "Running", the narrower line, moves to the middle, then to the right edge of the frame; "Pump P-101" stays put.
3. Choose **Weight** = 900: the text blackens and **Bold** is pressed. Click **Bold**: the weight goes to 400 and the button is released. Click it again: 700, pressed.
4. Click **Italic**: the text slants; again: upright.
5. Type **Color** = `#dc2626`, then **Opacity** = 0: the text and its lines vanish, the frame stays. Type **Opacity** = 100: the text is full red again.
6. Type **Letter spacing** = 10, then **Word spacing** = 20: the letters spread, then the gap between "Pump" and "P-101" widens; the frame follows. Look closely at the right end of each line: a gap of about 10 after the last letter means the browser adds the spacing after it too (VAL-014, step 1). Put both back to 0.
7. Tick **Underline** alone: one line under each text line, in the text's color, as long as the line. Untick it and tick **Strike-through** alone: one line through each text line. Untick it.
8. Type **Font** = Arial: the browser draws Arial, while the frame keeps Inter's measure (Arial's metrics are not read yet). In **SVG output**, `font-family` reads `&quot;Arial&quot;, sans-serif`: `&quot;` is how an attribute writes the quotes around a family name. Type **Font** = Inter again.
9. Type in **Content** "pump p-101 is running", then choose each **Case**: UPPER, lower, Sentence, Title, camel, Pascal, kebab, snake, CONSTANT: the line is written as the case says. Choose **As typed** again.
10. Type in **Content** "Pump P-101", a line break, "Running", and choose **Direction** = **Right to left** with **Alignment** = **Left**: each line still starts on the frame's left edge (the decision to confirm above); in **SVG output** each `<tspan>`'s `x` is the right end of its line.
11. Still right to left, type in **Content** two lines in Hebrew, for example "שלום" over "עולם טוב": the letters run from right to left. Inter has no Hebrew letter: each is measured as Inter's missing-glyph box, so the frame does not hug these letters. Choose **Left to right** again.
12. Type in **Content** "Pump P-101" followed by a line break and nothing after it: the frame is two lines tall, the second empty. Empty **Content** completely: nothing is drawn, no frame.
13. Type **Content** "Pump P-101", then **X** = 100, **Y** = 50: the text and its frame move together.
14. Type **Size** = 0: the drawing goes, the message under the fields says why, the fields are outlined in red. Type **Size** = 16: the text comes back, the message and the outlines go.
15. Click **Rectangle**: the text's fields go, the size, corners, angle row and Shape/Points buttons come back, and the rectangle is drawn.

What to look at closely: whether the frame hugs the letters (with Inter installed), the right end of a line with letter spacing (step 6), and right-to-left alignment (step 10).
