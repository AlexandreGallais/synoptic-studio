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

## Decisions to confirm

The rules marked provisional by SP-018 (2026-10-15); the stories add theirs.

| #   | Rule                                                                                                                                     | Example                                              |
| --- | ---------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------- |
| 1   | The HTML layer is a `switch`: a `foreignObject` with the XHTML extension for browsers, then the positioned lines for every other reader  | resvg, Batik and Inkscape draw the lines             |
| 2   | The HTML layer and the embedding are options of a document, off by default                                                               | the page opens with both off, the drawing as before  |
| 3   | A face's family is name ID 16, else 1; its weight `usWeightClass` brought into 100 to 900; italic from `fsSelection` bit 0               | a weight class of 950 gives 900                      |
| 4   | A face read again, same family, weight and style, replaces the one kept                                                                  | Arial Regular imported twice: one face               |
| 5   | Fonts are kept for the session; across sessions with the editor's files (E02)                                                            | a reload forgets an imported font                    |
| 6   | Embeddable: not bitmap only, not restricted without the preview-and-print or editable bit; preview-and-print fonts embedded              | Arial (8) and Inter (0) yes, `fsType` 2 no           |
| 7   | A face whose file is not held is not embedded, Inter's among them until its file is imported                                             | Inter listed as not embedded                         |
| 8   | Each face once per document, the whole file, no `local()`                                                                                | 10 texts in Arial regular and bold: 2 rules          |
| 9   | The layer's line height is the library's, the layer raised by half the line gap for an automatic line height                             | Arial 16: 0.26171875 higher; trimmed, 3.28515625     |
| 10  | Right to left in resvg and Batik, which ignore `direction`: keep the line as written, or write it from its left edge without `direction` | a Hebrew line drawn one width too far right in resvg |
