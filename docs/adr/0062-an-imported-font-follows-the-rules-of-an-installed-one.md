# ADR-0062 — An imported font follows the rules of an installed one

**Status**: Accepted (Product Owner, 2026-10-16, asked by questionnaire after US-056; US-059). Refines ADR-0061, decision 2: its decision 1 stands.

## Context

- **ADR-0061, decision 2**, let a font lacking a basic Latin letter be imported by hand with **Import a font**, the file not judged since the designer chose it (US-056). A face whose `fsType` forbids embedding it could be imported too: it was then left out of the exported SVG and listed under the drawing's options (F15, `DERIV-font-embedding`).
- **Testing US-056**, the Product Owner chose that every font the editor offers be one it can measure and export, whatever its source: a font imported that lacks the letters gives the wrong frame described by ADR-0061 (Apple Color Emoji: a square per letter), and a font that cannot be embedded gives a drawing that another computer draws with another font.

## Decision

1. **A face imported with Import a font is refused** when its `cmap` lacks any of the 52 letters of the ISO basic Latin alphabet (U+0041 to U+005A, U+0061 to U+007A, `REF-WIKI-ISO-BASIC-LATIN`), **or** when its `fsType` forbids embedding it (`DERIV-font-embedding`, step 3: 2 is refused, 0, 4 and 8 are not), the same rules as an installed face (`unlistedFaceReason`).
2. **The refusal is said**, the file and the face named with the reason: `Apple Color Emoji.ttc: Apple Color Emoji 400 refused, it lacks letters of the basic Latin alphabet (A to Z, a to z).`; `Locked.ttf: Locked 400 refused, its fsType forbids embedding it in an exported SVG.` The other faces of a collection are kept.
3. **Left for the epic on ease of use** (E02, ideas): whether a font that cannot be embedded may be exported once the designer forces it with a confirmation, or stays refused.

## Reasons

| Option | For | Against |
| --- | --- | --- |
| Same rules for an imported font (chosen) | one rule for the font list; every font offered measures and exports | a font for Greek, Cyrillic or Hebrew alone, or locked by its licence, cannot be used |
| An imported file not judged (ADR-0061, decision 2) | the designer's choice respected; scripts F05 built remain reachable | a wrong frame, or an export drawn with another font, from a font the list offers |

## Consequences

- `shapes.md` (Text, Only usable fonts listed, Import a font) states the rule; ADR-0061, decision 2, no longer holds.
- The library still measures Greek, Cyrillic and Hebrew (ADR-0060, decision 4) and still leaves a face that cannot be embedded out of a file (ADR-0053); the playground no longer reaches them through a font imported.
- The F15 guided step on CFF outlines imports STIXGeneral, Noto Sans Javanese lacking the Latin letters.

## References

ADR-0053, ADR-0060, ADR-0061, `DERIV-font-embedding`, `REF-WIKI-ISO-BASIC-LATIN`, [US-059](../backlog/stories/E15-F17-US-059-approved-fonts-only.md)
