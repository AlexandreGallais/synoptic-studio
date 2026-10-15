# ADR-0052 — Text measured from font metrics and drawn as positioned lines

**Status**: Accepted, decision 4 amended by ADR-0054 and ADR-0055, decision 1 amended by ADR-0055 (agent, SP-015, 2026-10-14, from the Product Owner's decisions of F05's refinement, round 2: the library computes the lines, text readable by every SVG reader, measuring as fast as a sum of numbers, no text converted to paths). Settles Q31 for F05; the HTML layer and font embedding are left to ADR-0053 (F15). Adds `tspan` to the SVG elements allowed in symbols (ADR-0002).

## Context

ADR-0002 keeps text a native `<text>`. A text must be measured to be placed, wrapped, aligned and turned about the middle of its block, without a DOM in the portable core (ADR-0025). Research note 0017 found:

- SVG readers draw `<text>` and `<tspan>` with `x` and `y` everywhere; SVG 2 itself suggests producers pre-lay out lines that way (`REF-SVG2-TEXT`), as Illustrator and Figma do (`REF-ADOBE-SVG-TEXT-ALIGN`, `REF-FIGMA-FORUM-SVG-ALIGN`).
- HTML in a `foreignObject` is drawn by browsers only (`REF-DRAWIO-SVG-TEXT`, `REF-BATIK-STATUS`); SVG 2 wrapping (`inline-size`) is drawn by no browser (`REF-SVGWG-1087`, `REF-SVGWG-2026-04-23`).
- `text-anchor`, `dominant-baseline`, `text-decoration` and `direction` are drawn unevenly across readers (`REF-RESVG-SUPPORT-TABLE`, `REF-LIBRSVG-TEXT`); `text-transform` is not required of SVG readers (`REF-SVG2-STYLING`).
- A font file holds every number needed to measure a line: units per em, advances, kerning pairs, vertical metrics (`REF-OT-HEAD`, `REF-OT-HMTX`, `REF-OT-HHEA`, `REF-OT-KERN`, `REF-OT-GPOS`). A canvas measures only in a browser (`REF-MDN-TEXTMETRICS`); in Node.js it needs a native package (`REF-NODE-CANVAS`).

## Decision

1. **Metrics read from font files.** The library reads a TrueType or OpenType file (`REF-OT-OTFF`) once into plain data: `unitsPerEm`, the advance of each character's glyph, the kerning pairs (`kern` format 0, `GPOS` pair adjustment), the `hhea` ascender, descender and line gap, the underline and strike-out metrics (`REF-OT-POST`, `REF-OT-OS2`) and `fsType`. A character missing from the font takes glyph 0 (`REF-OT-CMAP`). The default font's metrics ship with the library as data (Inter, `docs/tooling/dependencies.md`); a text whose font has no metrics is measured with the default font's.
2. **Measuring is a sum.** A line's width and the line height follow `DERIV-text-width`; the block of a text is its longest line by the sum of its line heights. Measuring reads no file and no DOM.
3. **One `<text>`, one positioned `<tspan>` per line.** Each line gets its own `x` and `y`, computed by the library: `x` from the alignment and the line's measured width (no `text-anchor`); `y` the baseline, the top of the line plus the font's ascender times size over units per em (no `dominant-baseline`, no `alignment-baseline`).
4. **What the `<text>` carries**: `font-family` (the font, then its fallback), `font-size`, `font-weight`, `font-style`, `fill` and `fill-opacity`, `letter-spacing` and `word-spacing` when not 0, `direction="rtl"` for right-to-left text, and a `transform` turning it about the middle of its block (`REF-SVG11-TRANSFORM`).
5. **Written by the library, not by the reader**: the case transforms (the transformed string is written); underline and strike-through, drawn as `<path>` rectangles from the font's underline and strike-out metrics (ADR-0002), not `text-decoration`; the `<text>` and these paths are then grouped in one `g`.
6. **Scripts**: the library measures scripts without shaping — Latin, Greek, Cyrillic — and Hebrew right to left. Arabic and Indic scripts need a shaping engine (`REF-HARFBUZZ-SHAPING`) and are left to the HTML layer (F15).
7. **Allowed SVG elements**: `svg`, `g`, `path`, `text`, `tspan`, `defs`.
8. **Left to ADR-0053 (F15)**: the HTML layer, a `foreignObject` inside a `switch` (`REF-SVG2-STRUCT`), and fonts embedded with `@font-face` (`REF-CSS-FONTS-4`, `REF-OT-OS2`).

## Reasons

| Option                                | For                                                                        | Against                                                                                                                |
| ------------------------------------- | -------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| Metrics and positioned lines (chosen) | drawn by every reader; measured anywhere, as fast as a sum; no dependency  | a reader's ligatures or a missing font change the drawn width a little; no shaping                                     |
| HTML in a `foreignObject`             | the browser's whole layout: wrapping, ellipsis, every script               | drawn by browsers only (`REF-DRAWIO-SVG-TEXT`, `REF-BATIK-STATUS`); measured only in a page                            |
| SVG 2 `inline-size`                   | wrapping by the reader                                                     | drawn by no browser (`REF-SVGWG-1087`)                                                                                 |
| Measured by a canvas                  | the browser's own widths                                                   | browser only (`REF-MDN-TEXTMETRICS`); a native package in Node.js (`REF-NODE-CANVAS`), against "no runtime dependency" |
| Text as paths                         | the same drawing everywhere (Figma's default, `REF-FIGMA-EXPORT-SETTINGS`) | not wanted by the Product Owner: the text must stay text                                                               |

## Consequences

- AGENTS.md lists `tspan` among the allowed SVG elements.
- The width the library measures and the width a reader draws agree when the reader has the font and applies only kerning; a reader that applies ligatures, or draws a fallback font, draws a slightly different width.
- Kerning applies only between characters of one line; a font with `CFF` outlines has its kerning in `GPOS` only (`REF-OT-KERN`).
- The vertical metrics come from `hhea`; the `OS/2` typographic metrics, which the specification recommends, may give other line heights (Arial: 18.39844 against 17.41406 at size 16). F15 compares them with what browsers draw.
- Q31 is settled for F05; Q24 (a `foreignObject` tool) stays open with ADR-0053.

## References

`REF-OT-OTFF`, `REF-OT-HEAD`, `REF-OT-HHEA`, `REF-OT-HMTX`, `REF-OT-CMAP`, `REF-OT-KERN`, `REF-OT-GPOS`, `REF-OT-OS2`, `REF-OT-POST`, `REF-SVG2-TEXT`, `REF-SVG2-STRUCT`, `REF-SVG2-STYLING`, `REF-SVG11-TRANSFORM`, `REF-RESVG-SUPPORT-TABLE`, `REF-LIBRSVG-TEXT`, `REF-BATIK-STATUS`, `REF-DRAWIO-SVG-TEXT`, `REF-SVGWG-1087`, `REF-SVGWG-2026-04-23`, `REF-ADOBE-SVG-TEXT-ALIGN`, `REF-FIGMA-FORUM-SVG-ALIGN`, `REF-FIGMA-EXPORT-SETTINGS`, `REF-HARFBUZZ-SHAPING`, `REF-MDN-TEXTMETRICS`, `REF-NODE-CANVAS`, `REF-CSS-FONTS-4`, ADR-0002, ADR-0025, `DERIV-text-width`, [research note 0017](../research/notes/0017-e15-text.md)
