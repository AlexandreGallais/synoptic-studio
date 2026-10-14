---
id: US-031
epic: E15
feature: F05
title: Place a text with the Text tool
status: ready
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

- [ ] T1 — The Text tool and its fields in the playground (3 h)
- [ ] T2 — The drawing of the text: positioned lines and their attributes (2 h)
- [ ] T3 — The Product Owner test card (30 min)

## Notes from EN-032's review (2026-10-15)

- **Spaces kept.** The library measures every space of a line, runs of spaces and spaces at its ends included; the SVG writer must keep them all, with `xml:space="preserve"` or a `white-space` that preserves them, or a reader collapses them and draws a narrower line than the measured block.
- **Letter spacing after the last character.** `DERIV-text-width` adds letter spacing between units only, `u − 1` times, as CSS Text 3 says; the CSS editors' draft now spreads it on both sides of each unit (`REF-CSSWG-10193`) and browsers differ. Measure in Chrome, Firefox and Safari whether a line with letter spacing ends with a trailing space, and record the result before the text's block is trusted for alignment.

## Decisions taken while building (2026-10-15)

- **Alignment and direction.** Left, center and right are the physical sides for both directions, as CSS's `left` and `right` are (`REF-CSS-TEXT-3`, §6.1): a right-to-left text aligned left starts each line on the block's left edge. The refinement names left, center and right without saying more; to be confirmed by the Product Owner at VAL-014, with the other reading (left and right swapped for right-to-left, as CSS's `start` and `end`) as the option.
- **Right-to-left lines.** With the initial `text-anchor`, `start`, SVG draws the right side of right-to-left text at the line's `x` (`REF-SVG2-TEXT`, §11.10.1.1), so each such line is written at its right edge, with `direction="rtl"` on the `<text>` (`DERIV-text-width`, step 5). resvg and Batik do not draw `direction` (`REF-RESVG-SUPPORT-TABLE`): there a right-to-left line would start at that edge and run off the block.
- **Font family.** Written as the font, its fallback, then `sans-serif`, each family once, names quoted and generic families as keywords (`REF-CSS-FONTS-4`, §2.1.1 and §2.1.2): `"Inter", sans-serif`.
- **Colors.** The fill is on the `<text>` and on each decoration's `<path>`, the opacity written as a number from 0 to 1 (`REF-SVG2-FILL-OPACITY`), which SVG 1.1 readers read too.
- **Spaces.** `xml:space="preserve"` on the `<text>` (`REF-SVG2-TEXT`, §11.10.3.2). A tab typed in a line is drawn as a space by such a reader, while the library measures the font's tab glyph.
- **Playground.** The text is measured with the default font's metrics (Inter, EN-031 AC5): its frame matches the drawing only where the browser has Inter; elsewhere the browser draws its `sans-serif` and the frame shows Inter's block.

## Letter spacing after the last character

No headless browser is among the development packages, and none is added for one measure. The measure is a step of VAL-014: in Chrome, Firefox and Safari, a `<text>` "HH" in Inter 16 with `letter-spacing="10"`, read with `getComputedTextLength()` (or `getBBox().width`): 23.78125 + 10 = 33.78125 when the spacing is only between the two letters, as the library measures (`DERIV-text-width`, step 3); 43.78125 when the browser adds it after the last letter too (`REF-CSSWG-10193`). The result goes into this file and `DERIV-text-width`; a browser that adds it after the last letter draws a line 10 wider than its block, so centered lines shift by 5.

## Product Owner test

Open `http://localhost:5173/`, then click **Text** in the toolbar. The panel shows the text's fields; the canvas shows "Pump P-101" over "Running", in a thin blue frame, the measured block. Whether Inter is installed matters: without it, the browser draws another font and the frame no longer hugs the letters.

1. Type **Size** = 16, then 48: the text and its frame grow together, the frame's top on the top of the block, X 0 and Y 0.
2. Choose **Alignment** = **Center**, then **Right**: "Running", the narrower line, moves to the middle, then to the right edge of the frame; "Pump P-101" stays put.
3. Click **Bold**: the button is pressed, **Weight** reads 700 Bold and the text gets heavier, the frame wider. Click it again: back to 400. Choose **Weight** = 100, then 900: the text thins, then blackens.
4. Click **Italic**: the text slants; again: upright.
5. Type **Color** = `#dc2626` and **Opacity** = 40: the text turns pale red.
6. Type **Letter spacing** = 10, then **Word spacing** = 20: the letters spread, then the gap between "Pump" and "P-101" widens; the frame follows. Look closely at the right end of each line: a gap of about 10 after the last letter means the browser adds the spacing after it too (see above).
7. Tick **Underline** and **Strike-through**: each line gets a line under it and one through it, in the text's color, as long as the line.
8. Type in **Content** "pump p-101 is running", then choose each **Case**: UPPER, lower, Sentence, Title, camel, Pascal, kebab, snake, CONSTANT: the line is written as the case says.
9. Type in **Content** two lines in Hebrew, for example "שלום" over "עולם טוב", and choose **Direction** = **Right to left**: the letters run from right to left. With **Alignment** = **Left**, each line starts on the frame's left edge (the decision to confirm above). Inter has no Hebrew letter: each is measured as Inter's missing-glyph box, so the frame does not hug these letters.
10. Type **X** = 100, **Y** = 50: the text and its frame move together.
11. Type **Size** = 0: the drawing goes, the message under the fields says why, the fields are outlined in red.
12. Click **Pipeline**: **SVG output** shows one `<text>` with one `<tspan>` per line, each with its `x` and `y`, `xml:space="preserve"`, no `text-anchor` and no `dominant-baseline`; underline and strike-through are `<path>` elements.

What to look at closely: whether the frame hugs the letters (with Inter installed), the right end of a line with letter spacing (step 6), and right-to-left alignment (step 9).
