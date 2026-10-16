# F15 — Fonts from files and embedded fonts

## What is new

A text can now use any font of the computer: a `.ttf` or `.otf` file imported, or, in Chrome and Edge, a font installed, read once its family is chosen. The text is measured with that font's own numbers, as with Inter. An option of the drawing goes further: **Embed fonts** writes each face used into the SVG, so that it carries its fonts. **Export SVG** saves the drawing as `drawing.svg`. The **HTML layer**, a second copy of each text for browsers, was removed at the validation (ADR-0060, US-044): it brought no speed, and served only scripts that industrial views do not use; every reader now draws the same positioned lines.

| Part                | What it does                                                                                                                                                         |
| ------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Font** list       | the families the editor measures: Inter first, then those imported, then the installed ones; no name typed                                                           |
| **Import a font**   | reads `.ttf`, `.otf` and `.ttc` files (a collection gives each of its faces), chooses the family at once and says it: “Arial 400 imported.”; a file refused says why |
| **Installed fonts** | in Chrome and Edge, after a permission, lists the computer's families; a family's files are read when it is chosen                                                   |
| **Weight** list     | only the weights the family has: Arial gives 400 Regular and 700 Bold                                                                                                |
| **Embed fonts**     | each face used written once, the whole file, in the `<defs>` of the file exported, never the drawing on screen (US-045); the faces left out listed with their reason |
| **Export SVG**      | saves `drawing.svg`, without the dashed selection frame                                                                                                              |

Rules: [domain, "Text"](../domain/shapes.md#text) (Fonts from files, Embedded fonts); formulas: [`DERIV-font-embedding`](../derivations/font-embedding.md), [`DERIV-html-layer`](../derivations/html-layer.md), kept as a record; decisions: [ADR-0053](../adr/0053-html-layer-and-embedded-fonts.md), [ADR-0060](../adr/0060-no-html-layer-and-fonts-embedded-at-export.md); research: [note 0020](../research/notes/0020-f15-fonts-and-html-layer.md).

## The ideas behind it

| Term                       | In plain words                                                                                               |
| -------------------------- | ------------------------------------------------------------------------------------------------------------ |
| **Face**                   | one file of a family: Arial regular, Arial bold, Arial italic                                                |
| **Metrics**                | the font's numbers the library measures with: each letter's width, the ascender, the descender, the line gap |
| **`fsType`**               | the permission a font's maker writes in it: whether the font may travel inside a document                    |
| **`@font-face`**           | a CSS rule naming a family and giving its file, here the whole file written in base64                        |
| **resvg, Batik, Inkscape** | readers of SVG files other than browsers: they draw the positioned lines, as browsers do                     |

**Measured, not guessed.** Choosing a font reads its file into numbers once; measuring stays a sum of those numbers, a thousand texts in a few milliseconds. “Hello World” in Arial 16 measures 82.3984375 × 18.3984375, in Inter 87.5546875 × 19.359375.

**Embedded only when allowed.** Arial (`fsType` 8) and Inter (0) may travel; a font marked 2 may not. Inter's numbers ship with the library, not its file: until its file is imported, Inter is listed as “Inter 400: not embedded, its file not held.”

**One layout for every reader.** A text is written once, as its positioned lines, which browsers, resvg, Batik and Inkscape all draw. The HTML layer, a `switch` holding a `foreignObject` laid out by the browser, was removed at the validation (ADR-0060): no `switch`, `foreignObject` nor XHTML is written, and F15's third criterion is withdrawn. Arabic and Indic scripts, which only the browser's layer joined, are not supported.

## How to try it

**Validation**: `npm run dev`, then `http://localhost:5173/?checklist=E15`: one page listing E15's options, one line each, what to try and what to see, a check box before it, the boxes ticked kept in this browser (US-051). Since then the plain address opens no guided test: the steps below come back only in `?guide=epic`, at the numbers given for the epic review.

`npm run dev` and `http://localhost:5173`: E15 is validated at once (VAL-017), 176 steps, F05's, F14's and F16's first; F15's are steps 120 to 176. At the epic review they come back as steps 207 to 263 of `?guide=epic`. **Next** types the text; Embed fonts starts unticked at each step, and a font imported stays for the session, so import Arial at step 121 and keep it. Arial is in `/System/Library/Fonts/Supplemental` on macOS. These are the numbers of the run on 2026-10-15: F17 removed the HTML layer's 20 steps (US-044), among them steps 128 and 129, the seven browser checks 133 to 139, and decisions 1, 9, 19 to 23 and 33 to 36.

| Steps   | What                                                                                                                                                |
| ------- | --------------------------------------------------------------------------------------------------------------------------------------------------- |
| 120–126 | the font list, a font imported, a file refused, a family's weights, installed fonts in Chrome and elsewhere, the text measured with the font chosen |
| 127–130 | Embed fonts, Export SVG (the HTML layer and Arabic in it removed by F17)                                                                            |
| 131–132 | the exported file in Chrome, Firefox and Safari, then in Inkscape                                                                                   |
| 133–139 | removed with the HTML layer (US-044)                                                                                                                |
| 140–175 | the 36 decisions below, one step each with its example                                                                                              |
| 176     | back to the rectangle, drawn as before                                                                                                              |

## Decided by the agent, to confirm

Each is built as written and shown by its step; the full list, with its examples, is in [VAL-017](../backlog/stories/E15-F15-VAL-017-validate-f15.md#decisions-to-confirm). Steps saying “Nothing to try” need a font file not at hand: the tests prove them, the step asks to confirm the rule.

| #   | Question                          | As built                                                                                                 | Step |
| --- | --------------------------------- | -------------------------------------------------------------------------------------------------------- | ---- |
| 1   | The HTML layer's form             | removed with the HTML layer (ADR-0060, US-044)                                                           | 140  |
| 2   | Both options                      | one option of the document, Embed fonts, off by default; the HTML layer removed (US-044)                 | 141  |
| 3   | A face's name                     | family from name ID 16, else 1; weight brought into 100 to 900; italic from `fsSelection`                | 142  |
| 4   | A face read again                 | replaces the one kept                                                                                    | 143  |
| 5   | How long fonts are kept           | for the session; across sessions with the editor's files (E02)                                           | 144  |
| 6   | Which faces may be embedded       | not bitmap only, not restricted without the preview-and-print or editable bit                            | 145  |
| 7   | A face whose file is not held     | not embedded, Inter's among them                                                                         | 146  |
| 8   | How a face is embedded            | once per document, the whole file, no `local()`                                                          | 147  |
| 9   | The layer's line height           | removed with the HTML layer (ADR-0060, US-044)                                                           | 148  |
| 10  | Right to left in resvg and Batik  | to choose: keep the line as written, or write it from its left edge without `direction`                  | 149  |
| 11  | The family name's encoding        | read from the Unicode strings, English first; Windows code pages not read                                | 150  |
| 12  | A file without a family           | refused as not a font, as one whose `OS/2` table is too short                                            | 151  |
| 13  | A family imported again           | keeps its first name; a new family listed after those kept                                               | 152  |
| 14  | Several permission bits           | the least restrictive wins                                                                               | 153  |
| 15  | CFF outlines (`OTTO`)             | embedded as `font/otf`, `format("opentype")`                                                             | 154  |
| 16  | A family partly embeddable        | embedded whole or not at all                                                                             | 155  |
| 17  | A family named like a generic one | never embedded                                                                                           | 156  |
| 18  | The default font reached alone    | left out: only a family a text names is embedded                                                         | 157  |
| 19  | The layer's elements              | removed with the HTML layer (ADR-0060, US-044)                                                           | 158  |
| 20  | The layer's numbers               | removed with the HTML layer (ADR-0060, US-044)                                                           | 159  |
| 21  | The layer's opacity               | removed with the HTML layer (ADR-0060, US-044)                                                           | 160  |
| 22  | The browser's lines in a box      | removed with the HTML layer (ADR-0060, US-044)                                                           | 161  |
| 23  | A trimmed text in a box           | removed with the HTML layer (ADR-0060, US-044)                                                           | 162  |
| 24  | The font field                    | a list of the families measured; no name typed (US-036)                                                  | 163  |
| 25  | A font imported                   | chosen at once, its face said; a file refused said why (US-036)                                          | 164  |
| 26  | Installed families                | listed without reading their files; read when chosen (US-036)                                            | 165  |
| 27  | The weight held                   | kept when the family has it, else the nearest face; Bold within the family's faces (US-036, US-037)      | 166  |
| 28  | Faces left out                    | listed with their reason, one line each (US-036)                                                         | 167  |
| 29  | Export SVG                        | `drawing.svg`, without the selection's frame (US-036)                                                    | 168  |
| 30  | A font not installed              | also given to the page, so that it is drawn with its own outlines (US-036)                               | 169  |
| 31  | While a family is read            | the text keeps the family before; one none of whose faces is read gives the list back, said why (US-036) | 170  |
| 32  | The underline and descenders      | the lines' underline drawn across them (AUD-018)                                                         | 171  |
| 33  | The layer's underline position    | removed with the HTML layer (ADR-0060, US-044)                                                           | 172  |
| 34  | Clipping with a line limit        | removed with the HTML layer (ADR-0060, US-044)                                                           | 173  |
| 35  | The background under the “…”      | removed with the HTML layer (ADR-0060, US-044)                                                           | 174  |
| 36  | ADR-0053's one `div`              | removed with the HTML layer (ADR-0060, US-044)                                                           | 175  |

## Limits

- Arabic and Indic scripts are not supported: the library's lines would need a shaping engine (ADR-0060).
- A font is embedded once per symbol; once per synoptic view for all its symbols comes with the views (E08).
- Fonts are forgotten at a reload: kept across sessions with the editor's files (E02).
- No WOFF nor WOFF2; a collection (`.ttc`) is read face by face (US-046); a family's localized names are not aliases of it.
- Installed fonts are listed only in Chrome and Edge; elsewhere, import the font's file.
- A right-to-left line in resvg and Batik is drawn one width too far right until decision 10 is made.

## Validation

Run by the Product Owner on 2026-10-15 (`VAL-017`, T3), with the whole of E15, in a Chromium-based browser; the details are in `VAL-017`.

| Remark                                                                                                                                                                                                                                                                             | What was done or where it went                |
| ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------- |
| every rule confirmed as built, except the HTML layer, removed (ADR-0060), and the embedding, applied only at export or save; installed collections and faces failing with a `TypeError` to read; weight 450 drawn with another face; a search field over installed fonts for later | ADR-0060; US-044 to US-047 (F17); E02 (ideas) |
