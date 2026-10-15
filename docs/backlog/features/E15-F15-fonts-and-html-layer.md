---
id: F15
epic: E15
title: Fonts from files, embedded fonts and the HTML layer
status: done
---

# E15 · F15 — Fonts from files, embedded fonts and the HTML layer

**Benefit hypothesis**: any font of the computer can be used and measured, a symbol can carry its fonts, and browsers lay text out live.

Refined with F05 on 2026-10-14 (its rounds 1 and 2); its stories were written by SP-018 (research note 0020, ADR-0053).

## Acceptance criteria

1. A TrueType or OpenType font file imported, or a font installed on the computer (read by the browser with permission), is read into its metrics once and kept.
2. An option embeds a font in a symbol, each face once (`@font-face` in `<defs>`), its licence permitting it (OS/2 `fsType`); met since 2026-10-15 in the file exported or saved only, never the drawing on screen (ADR-0060, US-045).
3. ~~An HTML layer in a `<foreignObject>` inside a `<switch>` lets a browser lay the text out live with its own fonts; other readers show the positioned lines (ADR-0053).~~ Withdrawn on 2026-10-15 at the validation of E15: the HTML layer brought no speed and served only scripts that industrial views do not use (ADR-0060); US-044 removed it.
4. Measuring stays a sum of stored numbers: a thousand texts measured in a few milliseconds.

## Stories

In delivery order.

| ID                                                                       | Type       | Title                                                                 | Status |
| ------------------------------------------------------------------------ | ---------- | --------------------------------------------------------------------- | ------ |
| [SP-018](../stories/E15-F15-SP-018-research-f15-fonts-and-html-layer.md) | Spike      | Research for F15: fonts from files, embedded fonts and the HTML layer | done   |
| [EN-039](../stories/E15-F15-EN-039-a-font-file-read-and-kept.md)         | Enabler    | A font file read, named and kept, and whether it may be embedded      | done   |
| [EN-040](../stories/E15-F15-EN-040-fonts-embedded-once-per-document.md)  | Enabler    | Fonts embedded once per document as @font-face rules                  | done   |
| [CHK-017](../stories/E15-F15-CHK-017-checkpoint-f15.md)                  | Checkpoint | Checkpoint in the middle of F15                                       | done   |
| [EN-041](../stories/E15-F15-EN-041-the-html-layer-of-a-text.md)          | Enabler    | The HTML layer of a text in a switch, over its positioned lines       | done   |
| [US-036](../stories/E15-F15-US-036-fonts-embedding-and-html-layer.md)    | User story | Import or choose a font, embed it, and turn the HTML layer on         | done   |
| [AUD-018](../stories/E15-F15-AUD-018-audit-f15.md)                       | Audit      | Audit F15                                                             | done   |
| [VAL-017](../stories/E15-F15-VAL-017-validate-f15.md)                    | Validation | Validate F15                                                          | done   |

## Feature plan

| Acceptance criterion | Realized by                    | Verified by       |
| -------------------- | ------------------------------ | ----------------- |
| F15.AC1              | SP-018, EN-039, US-036         | AUD-018, VAL-017  |
| F15.AC2              | EN-039, EN-040, US-036, US-045 | AUD-018, VAL-017  |
| F15.AC3 (withdrawn)  | SP-018, EN-041, US-036         | removed by US-044 |
| F15.AC4              | EN-039                         | AUD-018           |

## Left for later

- Arabic and Indian scripts: not supported; the HTML layer that laid them out was removed at the validation, the editor targeting the Latin script (Product Owner, 2026-10-15, ADR-0060, F17).
- A font embedded once in a synoptic view for all its symbols: with the views (E08); F15 embeds each face once per document, a symbol.
- Fonts kept across sessions: with the editor's files (E02).
- WOFF and WOFF2 files: not asked. Installed faces held in a collection: read by F17 (US-046).
- A font's localized family names (a Windows name in another language) as aliases of its family: only the English name, else the first, names it for now (EN-039 audit, 2026-10-15).
- A right-to-left line in resvg and Batik, which ignore `direction`: kept as written (VAL-017, 2026-10-15).
