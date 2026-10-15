---
id: VAL-017
epic: E15
feature: F15
title: Validate F15
status: ready
points: 1
---

# E15 · F15 · VAL-017 — Validate F15

The Product Owner imports and chooses fonts, embeds them and turns the HTML layer on in the playground, guided by short steps, opens the exported SVG in browsers and in another reader, and says what they see (ADR-0020).

## Acceptance criteria

- Given the playground, when the Product Owner follows the guided steps, then each option of F15 is tried, and each rule marked "to confirm at VAL-017" in `docs/domain/shapes.md` (Fonts from files, Embedded fonts, HTML layer), Q39 and ADR-0053 is confirmed or changed, with its example.
- Given an exported SVG with both options on, when opened in Chrome, Firefox and Safari and in Inkscape, then what each shows is recorded: the HTML layer in the browsers, the positioned lines in Inkscape, and whether Inkscape uses the embedded font (research note 0020, Open).

## Tasks

One task = one commit, referenced as `VAL-017.Tn`.

- [ ] T1 — The guided test, tested end to end (1 h)
- [ ] T2 — The demo page (30 min)
- [ ] T3 — The Product Owner's validation (1 h)

## Browser checks of the HTML layer

Steps for the guided test, from the EN-041 audit (checked in Chromium and Firefox; Safari to check): each compares the layer with the positioned lines, the layer on.

1. A box 50 × 20 holding 3 lines, aligned in the middle, then at the bottom: the browser's lines overflow the box as the library's do (`unsafe` alignment, the content never shrunk).
2. A box 80 × 0 and a box 0 × 40: no layer written, the positioned lines shown.
3. "Hello" and a line break, bottom-aligned in a box 100 × 100: "Hello" on the same line in both.
4. A word moved whole, wider than its box, the ellipsis off and a maximum of 1 line: the word overflows sideways, the lines below cut.
5. The same with the ellipsis on: the word ends with "…".
6. A text whose lines pass its block or its box: nothing clipped by the `foreignObject`.
7. A word cut with the ellipsis on, underlined and struck through (US-041): in the text's color, the "…" underlined and struck as in the SVG; the underline `#ff0000`, the strike-through in the text's color: the "…" struck, not underlined, the provisional reading of `shapes.md` (HTML layer).

## Decisions to confirm

The rules marked provisional by SP-018 (2026-10-15), then by EN-039 and EN-040 (rows 11 to 18, gathered at CHK-017); the stories add theirs.

| #   | Rule                                                                                                                                                              | Example                                                               |
| --- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------- |
| 1   | The HTML layer is a `switch`: a `foreignObject` with the XHTML extension for browsers, then the positioned lines for every other reader                           | resvg, Batik and Inkscape draw the lines                              |
| 2   | The HTML layer and the embedding are options of a document, off by default                                                                                        | the page opens with both off, the drawing as before                   |
| 3   | A face's family is name ID 16, else 1; its weight `usWeightClass` brought into 100 to 900; italic from `fsSelection` bit 0                                        | a weight class of 950 gives 900                                       |
| 4   | A face read again, same family, weight and style, replaces the one kept                                                                                           | Arial Regular imported twice: one face                                |
| 5   | Fonts are kept for the session; across sessions with the editor's files (E02)                                                                                     | a reload forgets an imported font                                     |
| 6   | Embeddable: not bitmap only, not restricted without the preview-and-print or editable bit; preview-and-print fonts embedded                                       | Arial (8) and Inter (0) yes, `fsType` 2 no                            |
| 7   | A face whose file is not held is not embedded, Inter's among them until its file is imported                                                                      | Inter listed as not embedded                                          |
| 8   | Each face once per document, the whole file, no `local()`                                                                                                         | 10 texts in Arial regular and bold: 2 rules                           |
| 9   | The layer's line height is the library's, the layer raised by half the line gap for an automatic line height                                                      | Arial 16: 0.26171875 higher; trimmed, 3.28515625                      |
| 10  | Right to left in resvg and Batik, which ignore `direction`: keep the line as written, or write it from its left edge without `direction`                          | a Hebrew line drawn one width too far right in resvg                  |
| 11  | A face's family read from the Unicode strings of its `name` table, English first; Windows code pages (encodings 3, 4, 5) not read                                 | a family named only in Shift-JIS: the file is refused                 |
| 12  | A file that names no family, or whose `OS/2` table is too short to give its style, refused as not a font                                                          | a font without name ID 1 nor 16: refused                              |
| 13  | A face read again keeps its family's first name; a new family is listed after those kept                                                                          | `ARIAL` imported after `Arial`: still `Arial`                         |
| 14  | An `OS/2` table of version 3 or later that sets several permission bits: the least restrictive wins                                                               | `fsType` 10 in version 4: embeddable                                  |
| 15  | A file with CFF outlines (`OTTO`) embedded as `font/otf` with `format("opentype")`                                                                                | an `.otf` file: `data:font/otf;base64,…`                              |
| 16  | A family embedded whole or not at all: one face of it used that may not be embedded leaves out every face of it                                                   | Inter 400 held, 700 not: no Inter embedded                            |
| 17  | A family named like a generic family is never embedded                                                                                                            | a font named `Monospace`: left out, generic name                      |
| 18  | Only a face of a family a text names, as its font or its fallback, is embedded; the default font reached alone is left out                                        | a text in Helvetica measured with Inter: not embedded                 |
| 19  | The HTML layer is a frame `div` holding the content's `div`, one `span` per paragraph                                                                             | the `foreignObject` holds two nested `div`                            |
| 20  | The layer's numbers are written at 5 decimals, as every SVG number                                                                                                | Arial 16's line height `18.39844px`                                   |
| 21  | The layer's color and a background's carry their opacity as one eight-digit color                                                                                 | black at 40 %: `#00000066`                                            |
| 22  | In a box, the browser shows at most the lines the library shows: clamped with "…" with the ellipsis on, cut down only at `n` line heights with it off             | a box 50 × 40 in Arial 16: at most 2 lines                            |
| 23  | A trimmed text in a box: its layer placed so that both first baselines meet, whatever the alignment                                                               | at the bottom, lowered by 3.65234375                                  |
| 24  | The font field is a list of the families the editor measures: Inter first, then those imported, then the installed ones; no name typed (US-036)                   | `monospace` cannot be chosen                                          |
| 25  | A font imported is chosen at once, its face said, a file refused said why (US-036)                                                                                | "Arial 400 imported."                                                 |
| 26  | The installed families are listed without reading their files; a family's files are read when it is chosen (US-036)                                               | "Georgia: 4 faces read."                                              |
| 27  | A family chosen keeps the weight held when it has it, else the face the Font face rule matches; Bold asks for 700 or 400 among the faces (US-036)                 | Inter 300 to Arial gives 400; Bold on a family of 400 alone keeps 400 |
| 28  | The faces left out of embedding listed with their reason, one line each (US-036)                                                                                  | "Inter 400: not embedded, its file not held."                         |
| 29  | **Export SVG** saves the drawing as `drawing.svg`, without the selection's frame (US-036)                                                                         | the exported file opened in a browser                                 |
| 30  | A font imported is also given to the page (`FontFace`), so that a font not installed is drawn with its own outlines (US-036)                                      | a downloaded `.ttf` drawn in its font                                 |
| 31  | While an installed family chosen is read the text keeps the family chosen before it; one none of whose faces is read gives the list back to it, said why (US-036) | Georgia chosen: Arial kept until read                                 |
