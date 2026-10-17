# F14 — A box of text

## What is new

A text can now sit in a **box** of a given width and height instead of fitting its content: its lines wrap at the spaces, a text too long ends with "…", and the lines are placed across and down the box. A label or an explanation then keeps to its frame in a synoptic view, as in Figma or a word processor.

![“Pump P-101 is running” in Inter 16, wrapped in a box 110 × 60, the box dashed](./images/f14-box.svg)

| Option | What it does |
| --- | --- |
| **Fit** | **Fit content**, as in F05, or **Box**; the box's own fields show only for a box |
| **Width**, **Height** | the box, integers from 0 to 100 000; the dashed frame is the box, not the lines |
| **Max lines (0 none)** | the most lines shown; the last one shown ends with "…" |
| **Ellipsis** | on by default; off, a cut text simply stops |
| **Vertical alignment** | the block of lines at the top, the middle or the bottom of the box |
| **Line height (0 auto)** | the distance between two baselines; 0 keeps the font's own |
| **Trim** | the block starts at the capitals' top, not at the font's ascender |
| **Background**, its color and opacity | a band behind each line, as a highlighter, yellow and opaque when ticked |
| **Alignment**, **Angle** (F05's fields) | each line on the left, the center or the right of the box; a text in a box turns about the box's middle |

Rules: [domain, "Text"](../domain/shapes.md#text); formulas: [`DERIV-text-box`](../derivations/text-box.md); decisions: [ADR-0052](../adr/0052-text-measured-and-drawn-as-positioned-lines.md), [ADR-0055](../adr/0055-text-fallback-metrics-and-turn-on-its-group.md).

## The ideas behind it

| Term | In plain words |
| --- | --- |
| **Wrap** | starting a new line when the next word does not fit; the lines break only at spaces, never inside a word |
| **Ellipsis** | the "…" that ends a text cut short; characters are removed until the line and its "…" fit |
| **Line height** | the distance from one baseline to the next; the room it adds is spread half above and half below the letters |
| **Content area** | the height from the font's ascender (the top of tall letters) to its descender (the bottom of "g" or "p"): 19.36 at Inter 16 |
| **Cap height** | the height of a capital letter, 11.64 at Inter 16; trimming starts the block there |
| **No-break space** | a space that keeps the two words around it on one line |

**Wrapping and cutting.** "Pump P-101 is running" in a box 110 wide: "Pump P-101 is" (104.77) fits, "running" goes to the next line. At most one line, "Pump P-101 is…" would be 118.6 wide: characters go until it fits, "Pump P-101…" (101.78).

![At most one line: “Pump P-101…”](./images/f14-ellipsis.svg)

**Placing the lines.** Across the box, each line follows the text's alignment; down the box, the block of lines sits at the top, the middle or the bottom. Below, "Pump P-101" over "Running", centered and in the middle of a box 120 × 80.

![Two lines centered in the middle of a box 120 × 80](./images/f14-aligned.svg)

**Background.** Each line gets a band from the ascender to the descender, as long as the line, its "…" included, ending with its last character other than a space (US-049); an empty line has none. All bands are one `<path>` under the letters, so that where bands overlap, under a small line height, the overlap is filled once.

![A yellow band behind each line](./images/f14-background.svg)

**Turning.** A text in a box turns about the middle of its box, with its frame, bands and strokes.

![The box turned by 30 about its middle](./images/f14-turned.svg)

## How to try it

**Validation**: `npm run dev`, then `http://localhost:5173/?checklist=E15`: one page listing E15's options, one line each, what to try and what to see, a check box before it, the boxes ticked kept in this browser (US-051). Since then the plain address opens no guided test: the steps below come back only in `?guide=epic`, at the numbers given for the epic review.

Install Inter 4.1 first, as for F05. Then `npm run dev` and `http://localhost:5173`: 41 steps, each one line to do and one to look at; **Next** types the text and its box. E15 is validated at once (VAL-017): they are steps 37 to 77 of its 176, numbered so below; at the epic review, steps 124 to 164 of `?guide=epic`. These are the numbers of the run on 2026-10-15: F17 removed steps 39 and 55, the break at a character, a word too long now always kept whole (US-042), then steps 47, 65 and 66, the balanced wrap, lines now always wrapped automatically (US-043). The steps give each example in Inter 16, whose metrics ship with the library.

| Steps | What |
| --- | --- |
| 37–47 | Fit content or Box, width and height, a long word kept whole, the ellipsis, max lines, vertical alignment, alignment across the box, line height, trim (the break at a character and the balance removed by F17) |
| 48–49 | the background, its color and opacity |
| 50–51 | turned about the box's middle, values refused |
| 52 | back to Fit content: the text draws as in F05 |
| 53–74 | the 22 decisions below, one step each with its example |
| 75 | a turned box resized in place, its turned top-left corner kept (ADR-0057, US-038) |
| 76 | a turned box's straight box, upright, with its x, y, width and height (US-039) |
| 77 | back to the rectangle, drawn as before |

## Decided by the agent, to confirm

Each is built as written and shown by its step; the full list, with its examples in Arial 16, is in [VAL-015](../backlog/stories/E15-F14-VAL-015-validate-f14.md#decisions-to-confirm).

| # | Question | As built | Step |
| --- | --- | --- | --- |
| 1 | Box size | integers from 0 to 100 000 | 53 |
| 2 | Spaces opening a paragraph | kept with its first word: " Hello" over "World" | 54 |
| 3 | A word too long, broken | removed at the validation: a word too long always stays whole (US-042) | 55 |
| 4 | A word too long, moved whole | on its own line, cut with "…", or whole and overflowing without the ellipsis | 56 |
| 5 | No-break space | no line ends after it | 57 |
| 6 | A box shorter than a line | one line shown at least; max lines from 0 (none) to 100 000 | 58 |
| 7 | The ellipsis | an option, on by default | 59 |
| 8 | What the ellipsis removes | characters, not whole words: "Hell…" in a box 50 | 60 |
| 9 | One character wider than the box | no ellipsis | 61 |
| 10 | Empty lines left out | no ellipsis | 62 |
| 11 | Even "H…" too wide | drawn past the box's edge; or draw only what fits, or nothing | 63 |
| 12 | Line height | 0 automatic, or 1 to 100 000, the room spread half above, half below | 64 |
| 13 | Balanced wrap | removed at the validation: lines always wrap automatically (US-043) | 65 |
| 14 | Balance and a cut | removed with the balanced wrap (US-043) | 66 |
| 15 | The "pretty" wrap style | dropped: not built (US-043) | 67 |
| 16 | Options of every text | line height, trim and background for every text; the others in a box only | 68 |
| 17 | Defaults | fit content, automatic line height, no trim, no background, ellipsis on; a box 0 × 0 | 69 |
| 18 | The background's height | the content area, from the ascender to the descender | 70 |
| 19 | Bands under a line height or a trim | still from the ascender to the descender, so a band may rise above the box | 71 |
| 20 | Overlapping bands | one path, the overlap filled once | 72 |
| 21 | The turn of a text in a box | about the box's middle (ADR-0055, decision 3); grown or resized, its turned top-left corner kept (ADR-0057) | 73 |
| 22 | Other no-break characters | only U+00A0 holds a line; U+202F, U+2007 and U+2060 may end one | 74 |

## Limits

- Fonts other than Inter are still measured with Inter until fonts are read from files (F15).
- No hyphenation: a word too long stays whole, overflowing its box or cut with "…" (US-042).
- One wrap only, automatic: no balanced nor "pretty" wrap (US-043).
- A line's characters are never clipped at the box's edge: what overflows is drawn.

## Validation

Run by the Product Owner on 2026-10-15 (`VAL-015`, T3), with the whole of E15, in a Chromium-based browser; the details are in `VAL-015`.

| Remark | What was done or where it went |
| --- | --- |
| every rule confirmed as built, except the choice for a word too long, removed (always whole), and the balanced wrap, removed; a background band past a line's end; a turned box's frames drawn as a shape's | US-042, US-043, US-049, US-050 (F17) |
