# E15 — Placing text in symbols

## In one paragraph

Before E15 a symbol held only shapes. Now a **Text** tool writes a label such as "Pump P-101" in a symbol, in a font of the computer, at an integer size, weight and color, and the library **measures** it from the font's own numbers, so it knows where the text ends without asking a browser. A text can fit its content or sit in a **box** that wraps it and cuts it with "…"; its colors are typed in hexadecimal or picked; its fonts are read from files or from the computer and written into the exported SVG when they allow it.

## Feature by feature

### F05 — Text and its characters

The **Text** tool and its fields: content on several lines, font, size, weight with a **Bold** button, italic, color and opacity, letter and word spacing, underline and strike-through, a letter case (lower, upper, camelCase, kebab-case, CONSTANT_CASE…), an alignment, a position and an angle. The text is drawn as plain lines that every SVG reader draws the same way. [Demo page](./f05-text.md)

![“Pump P-101” over “Running” in Inter 32, its measured block dashed around it](./images/f05-label.svg)

### F14 — A box of text

**Fit content** lets the text take the size it needs; **Box** gives it a width and a height: its lines wrap at the spaces, a text too long ends with "…" or after **Max lines**, and the lines are placed across and down the box. **Line height**, **Trim** and a **Background** band behind each line complete it; **Reset** gives back the size the text fits. [Demo page](./f14-text-box.md)

![“Pump P-101 is running” in Inter 16, wrapped in a box 110 × 60, the box dashed](./images/f14-box.svg)

### F16 — Colors and their picker

A color is typed or pasted in any common hexadecimal form (`F80`, `#FF8000`, `ff8000cc`) and kept as `#` and six lower-case digits with an opacity. A swatch opens a **picker**: a plane, a hue bar, an opacity bar, and the values in HSB or HSL as whole numbers. The text, its background, its underline and its strike-through each have one; the underline and the strike-through follow the text's color until one is chosen. [Demo page](./f16-colors.md)

![“Pump P-101” in #ff8000; in #0000ff at 80 %; in #1e293b on a #22c55e band at 40 %](./images/f16-colors.svg)

### F15 — Fonts from files and embedded fonts

**Import a font** reads a `.ttf` or `.otf` file; **Installed fonts** reads, in Chrome and Edge, the fonts of the computer. A text is measured with the font's own numbers. **Embed fonts** writes each face used into the exported SVG, once per document, when the font's licence flag (`fsType`) permits it. [Demo page](./f15-fonts.md)

### F17 — Text corrections after the E15 validation

The validation of 2026-10-15 removed three options (the choice for a word too long, the balanced wrap, the HTML layer) and fixed the defects found; two later passes made the font list hold only what can be used. A word too long stays whole; fonts are embedded only when a file is written; collections are read; a weight such as 450 is drawn as measured; values held down redraw without lag; a background band ends with its line; a turned text shows its frames as a turned shape does; only fonts read, holding the basic Latin letters and allowing embedding, are listed, and the font chosen for the designer is embedded as one chosen by hand. A feature is now validated on one **checklist** page (`?checklist=E15`). [Demo page](./f17-text-corrections.md)

## Words you will meet

| Term           | In plain words                                                                                           | Where it shows                   |
| -------------- | -------------------------------------------------------------------------------------------------------- | -------------------------------- |
| Measured block | the rectangle the text really takes, computed from the font's numbers, not read from the screen          | the dashed frame around a text   |
| Font metrics   | the numbers inside a font file: each letter's width, the height above and below the line                 | Import a font, Installed fonts   |
| Face           | one style of a font family: Regular, Bold, Italic, a weight such as 450                                  | the weight list                  |
| Fit content    | the text takes the size its lines need                                                                   | Fit content / Box                |
| Box            | a width and a height the text wraps into; what does not fit ends with "…"                                | Box, Max lines, Reset            |
| Trim           | the space above the first line and below the last one cut down to the letters                            | Trim                             |
| Line height    | the distance from one line to the next; 0 lets the font decide                                           | Line height                      |
| Opacity        | how much a color covers what is behind it, from 0 % (invisible) to 100 %                                 | each color's Opacity             |
| HSB, HSL       | two other ways to say a color — hue, saturation, brightness or lightness — computed from the hexadecimal | the picker                       |
| Embedded font  | the font's file written inside the SVG, so the drawing carries it                                        | Embed fonts, Export SVG          |
| `fsType`       | a flag in the font file saying whether its licence lets it be embedded                                   | the reason a font is not listed  |
| Direction      | left to right, or right to left as Hebrew is written                                                     | Direction (to be removed, below) |

## User test of the epic

The review was the Product Owner's tour of the whole Text tool in Chrome on 2026-10-16, on `feature/f17-text-corrections`, right after the last validation pass.

| Step | Do                                                                                  | You should see                                          | OK? |
| ---- | ----------------------------------------------------------------------------------- | ------------------------------------------------------- | --- |
| 1    | **Installed fonts**, then choose Arial (F15, F17)                                   | the text drawn and measured in Arial                    | yes |
| 2    | Change the size; color white, an opacity, through the picker (F05, F16)             | the text follows each value                             | yes |
| 3    | **Bold**, then italic (F05)                                                         | bold widens the box; italic leaves it as it is          | yes |
| 4    | Letter spacing, reduced below 0; word spacing (F05)                                 | the letters and words tighten and spread                | yes |
| 5    | Underline and strike-through, with their own color, then following the text (F16)   | each stroke in its color, or in the text's              | yes |
| 6    | Every letter case (F05)                                                             | each line written in that case                          | yes |
| 7    | **Fit content**, **Box**, **Reset**; Trim, Max lines, Line height, Background (F14) | the box, its wrap, its "…", its spacing and its bands   | yes |
| 8    | **Import a font** (F15, F17)                                                        | the font listed and used, or refused with its reason    | yes |
| 9    | Angle, then X and Y (F05, F17)                                                      | the text turns; X and Y show its turned top-left corner | yes |

## Decisions taken on the way

- A text is measured from its font's metrics and drawn as plain lines, `text` and `tspan` (ADR-0052); its font list and its spaces are kept (ADR-0054).
- A font without metrics is measured with its fallback's, and a turned text turns by its group (ADR-0055); a turned text that grows keeps its turned top-left corner (ADR-0057).
- A color is stored as hexadecimal and an opacity; HSL and HSB are only views, shown and typed as whole numbers (ADR-0056, ADR-0058); an open picker rebuilds each edit from the color it read (ADR-0059).
- Fonts are embedded once per document when `fsType` permits (ADR-0053); since the validation, only when a file is written, and no HTML layer (ADR-0060).
- The Latin script is the target; a font without the basic Latin letters is not listed, and an imported font follows the same rules as an installed one (ADR-0060, ADR-0061, ADR-0062).
- A feature is validated on its epic's checklist page, one line per option kept.

## Limits and next epic

- **The Direction field is to be removed.** Right to left is of no use for the Latin target, as for fonts; it stays for now and goes at the later epic on the editor's ease of use, where options that make no sense are dropped, the aim being the fewest options doing the most ([E02, ideas](../backlog/epics/E02-edit-shapes.md)).
- No hyphenation, no balanced wrap, no single-line option yet; Arabic and Indic scripts are not supported (ADR-0060).
- Installed fonts only in Chrome and Edge; fonts forgotten at a reload until the editor keeps files (E02).
- Apple Color Emoji must not be chosen yet: its 192 MB collection exhausts the page's memory.
- A text turned is not handled yet like a turned rectangle; a search over fonts, a curated set of fonts and a reset of the stroke colors are ideas for E02.
- Next: the retrospective of E15 (RET-003), then E02, editing shapes with precision.

## Review

Held with the Product Owner on 2026-10-16 (`REV-003`), as a tour of the whole Text tool right after the third pass of VAL-018.

**The tour.** Every option showed what it meant to show: a font chosen (Arial), the size, the color white, the opacity, the picker, bold (the box grows), italic (the box does not change, which the Product Owner prefers), letter spacing (reducible), word spacing, underline and strike-through with their own color or following the text, every letter case, Fit content and Box with Reset, trim, the maximum of lines, line height, the background, Import a font, the angle, X and Y.

**Remarks.**

| Remark                                                                                                                                                                        | Where it went                                                                             |
| ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------- |
| Italic leaves the box as it is; the Product Owner prefers it so.                                                                                                              | kept as built                                                                             |
| The text direction, right to left, is of no use: the target is the Latin script only, a strict choice as for fonts. Kept for now; it must be removed at the later UI/UX epic. | to-do in [E02, ideas](../backlog/epics/E02-edit-shapes.md); `docs/domain/shapes.md`, Text |

**Closure criteria.** The text cases of `docs/domain/shapes.md` render as specified in the playground: the four features validated on 2026-10-15, F17 on 2026-10-16, and the tour above. E15 is closed; its retrospective (RET-003) follows.
