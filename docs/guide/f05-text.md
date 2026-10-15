# F05 — Text and its characters

## What is new

A **Text** tool writes a label in a symbol: "Pump P-101" over "Running", in the font, size, weight and color chosen. The library **measures** the text itself, from the font's numbers, so that it knows where the text ends without showing it in a page; the SVG then holds plain lines that every SVG reader draws the same way.

![“Pump P-101” over “Running” in Inter 32, its measured block dashed around it](./images/f05-label.svg)

| Option                               | What it does                                                                                       |
| ------------------------------------ | -------------------------------------------------------------------------------------------------- |
| **Content**                          | the lines, broken where the designer pressed Enter; empty, nothing is drawn                        |
| **Font**                             | any font name, written with `sans-serif` after it for a reader that lacks it                       |
| **Size**                             | an integer from 1 to 100 000, in the one unit used everywhere                                      |
| **Weight**, **Bold**, **Italic**     | 100 (thin) to 900 (black); Bold sets 700, Italic slants                                            |
| **Color**, **Opacity**               | `#` and six hexadecimal digits; 0 to 100 %                                                         |
| **Letter spacing**, **Word spacing** | added between the letters, or at each space; negative to tighten                                   |
| **Underline**, **Strike-through**    | a thin rectangle under or through each line, where the font says                                   |
| **Case**                             | the text written in upper, lower, sentence or title case, or as `camelCase`, `kebab-case` and more |
| **Direction**                        | left to right, or right to left (Hebrew)                                                           |
| **Alignment**                        | each line on the left, the center or the right of the block                                        |
| **X**, **Y**, **Angle**              | the block's top-left corner, and a turn about its middle; each tool keeps its own                  |

Rules: [domain, "Text"](../domain/shapes.md#text); decisions: [ADR-0052](../adr/0052-text-measured-and-drawn-as-positioned-lines.md), [ADR-0054](../adr/0054-text-font-list-and-spaces-kept.md), [ADR-0055](../adr/0055-text-fallback-metrics-and-turn-on-its-group.md); formula: [`DERIV-text-width`](../derivations/text-width.md).

## The ideas behind it

| Term               | In plain words                                                                                                                                   |
| ------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| **Font metrics**   | the numbers a font file holds for each letter: how wide it is (its **advance**), how far the letters reach up and down, where the underline sits |
| **Kerning**        | a small correction between two letters that look too far apart: in "To", the o slides under the T, 2.5 closer at size 32                         |
| **Measured block** | the rectangle the text fills: as wide as its widest line, as tall as its lines; shown dashed when the text is selected                           |
| **Baseline**       | the line the letters sit on; each line is written at its baseline                                                                                |
| **Missing glyph**  | the box a font draws for a character it does not have, such as a Hebrew letter in Inter                                                          |

**Measuring.** A line's width is the sum of its letters' advances, scaled to the size, plus kerning and spacing. "Pump P-101" in Inter 32 measures 175.90625; each line is 38.71875 tall, so the two lines make a block 175.90625 × 77.4375. Fonts are read once and kept: measuring a text is then a sum of stored numbers, fast enough for a view that changes a thousand texts a hundred times a second.

**Inter.** The default font is Inter 4.1, whose numbers ship with the library. The playground measures every font with Inter until fonts are read from files (F15): with Inter installed, the frame hugs the letters; with another font, the browser draws that font and the frame keeps Inter's measure.

**Alignment.** Centered, each line starts halfway into the room it leaves; aligned right, after all of it. Below, "Running" starts 26.15 into the block, and each line is underlined as long as itself.

![The same text centered and underlined](./images/f05-centered.svg)

**Cases.** A value typed once comes out formatted. "pump P-101 is running" gives:

![camelCase, kebab-case and CONSTANT_CASE of the same words](./images/f05-cases.svg)

**Turning.** A text turns clockwise about the middle of its block, by an integer angle kept from 0 to 359: 370 becomes 10, −90 becomes 270. Its underline, strike-through and dashed frame turn with it.

![The text turned by 90 about the middle of its block](./images/f05-turned.svg)

## How to try it

Install Inter 4.1 first (Font Book on macOS shows it). Then `npm run dev` and `http://localhost:5173`: 35 steps, each one line to do and one to look at; **Next** types the text. At the epic review they come back as steps 88 to 122 of `?guide=epic`.

| Steps | What                                                                                                                       |
| ----- | -------------------------------------------------------------------------------------------------------------------------- |
| 1     | Inter installed: the frame hugs the letters                                                                                |
| 2–14  | every field: content, empty text, size, weight, italic, color, spacing, underline, cases, alignment, right to left, Hebrew |
| 15–21 | position, turning, the angle kept, the dashed frame, the turn on a group, values refused, each tool's placement            |
| 22–32 | the decisions to confirm and the audit's questions, one step each with its example                                         |
| 33–34 | two checks in Chrome, Firefox and Safari, in the browser's console                                                         |
| 35    | back to the rectangle, drawn as before                                                                                     |

The two browser checks: "HH" in Inter 16 with letter spacing 10 measures 33.78125 when the spacing goes between the letters only, as the library measures, and 43.78125 when the browser adds it after the last letter too; a native underline drawn over ours shows whether the browser reads the font's underline position as the top of the stroke, as the library does, or as its middle, 6.84 lower at size 200.

## Decided by the agent, to confirm

Each was built as the first option and is shown by its step; the full list, with every option, is in [VAL-014](../backlog/stories/E15-F05-VAL-014-validate-f05.md#decisions-to-confirm).

| Question                                | As built                                                                                 | Step |
| --------------------------------------- | ---------------------------------------------------------------------------------------- | ---- |
| Cases (Q36)                             | words cut at spaces, hyphens and underscores: `pumpP101IsRunning`                        | 11   |
| Left and right in a right-to-left text  | the physical sides: aligned left, each line starts on the block's left edge              | 13   |
| The turn                                | one `rotate` on the group of the text and its strokes                                    | 19   |
| A font without metrics                  | measured with its fallback's, else Inter's: Arial measures 175.91 like Inter             | 22   |
| A font's name                           | stored trimmed; a line break or a tab refused                                            | 23   |
| The bold button                         | pressed from 700; from 800 a click gives 400, from 600 it gives 700                      | 24   |
| Spacing bounds                          | −100 000 to 100 000                                                                      | 25   |
| A line tightened past nothing           | kept negative: at letter spacing −20, "Pump P-101" is −4.09 wide and may leave its block | 26   |
| Strokes under such a line               | none                                                                                     | 27   |
| Line breaks                             | LF, CR and CR LF; the line separator U+2028 is a missing glyph                           | 28   |
| Characters measured                     | Latin, Greek, Cyrillic, Hebrew and the symbols labels use (€ − → ≤ ™): 2 080 code points | 29   |
| Line gap (audit)                        | after each line, the last included: Arial 16 two lines 36.80 tall                        | 30   |
| A font of line height 0 or less (audit) | kept as read                                                                             | 31   |
| Kerning across scripts (audit)          | every script of the font at once                                                         | 32   |
| The dashed frame                        | dashed, as a shape's straight box                                                        | 18   |

## Limits

- No box of text yet: lines are broken only where the designer pressed Enter. Wrapping in a box, an ellipsis, a maximum number of lines, a line height and a background come with F14.
- Fonts other than Inter are drawn by the browser but measured with Inter until fonts are read from files or from the computer (F15); the HTML layer for browsers comes with F15 too.
- No color picker yet (F16).
- Arabic and Indic scripts need shaping: they are left to the browser's HTML layer (F15).

## Validation

To be run by the Product Owner (`VAL-014`, T3).

| Remark | What was done or where it went |
| ------ | ------------------------------ |
