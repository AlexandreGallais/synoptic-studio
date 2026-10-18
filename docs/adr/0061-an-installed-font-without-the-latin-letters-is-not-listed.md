# ADR-0061 — An installed font without the basic Latin letters is not listed

**Status**: Accepted (Product Owner, 2026-10-16, at VAL-018's second pass, asked by questionnaire; US-056). Refines ADR-0060, decision 4 (scripts): its other decisions stand. Decision 2 is refined by ADR-0062: an imported font follows the same rules.

## Context

- **ADR-0060, decision 4**, makes the Latin script the editor's target and keeps what F05 built for Greek, Cyrillic, Hebrew and right to left.
- **A wrong frame, measured.** On the Product Owner's Mac, Apple Color Emoji maps 28 characters — the space, `#`, `*` and the digits of its keycaps — and no letter. The browser draws the letters typed with another font while the editor measures them with the font's `.notdef`, 800 units wide on 800 per em: the frame is a square per letter. Al Nile (36 characters) and Apple Braille (3) map no Latin letter either; Helvetica maps them all (US-056).
- **The agent's first reading** (US-056, provisional) left out an installed font lacking any of the 52 letters of the ISO basic Latin alphabet, A to Z and a to z (`REF-WIKI-ISO-BASIC-LATIN`). It also leaves out an installed font written for Greek, Cyrillic or Hebrew alone, which decision 4 of ADR-0060 did not settle.

## Decision

1. **An installed font that lacks any of the 52 letters of the ISO basic Latin alphabet** (U+0041 to U+005A, U+0061 to U+007A) **is left out of the font list, whatever its script**: an installed font for Greek, Cyrillic or Hebrew alone is no longer offered.
2. **Such a font can still be imported by hand** with **Import a font**: a file the designer chooses is not judged, and the library still measures Greek, Cyrillic and Hebrew as F05 built them.

## Reasons

| Option | For | Against |
| --- | --- | --- |
| Left out whatever its script (chosen) | every font listed gives a frame that fits Latin letters; one plain rule | a Greek-, Cyrillic- or Hebrew-only installed font must be imported |
| Kept when it maps the letters of a measured script | such fonts offered from the list | a rule per script; a frame that misfits as soon as Latin is typed |

## Consequences

- `shapes.md` (Text, Only usable fonts listed) states the rule as settled; the other rules of that paragraph stay provisional until VAL-018.
- `hasBasicLatinLetters` and `unlistedFaceReason` (US-056) implement it; the playground reads it for installed faces only.

## References

ADR-0060, `REF-WIKI-ISO-BASIC-LATIN`, [US-056](../backlog/stories/E15-F17-US-056-usable-fonts-only.md)
