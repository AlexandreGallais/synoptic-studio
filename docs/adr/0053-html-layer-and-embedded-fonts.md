# ADR-0053 — The HTML layer of a text and its embedded fonts (`switch`, `foreignObject`, `style`)

**Status**: Accepted (agent, SP-018, 2026-10-15, from the Product Owner's decisions of F05's refinement, round 2: an HTML layer for browsers on top of the positioned lines, an option that embeds a font in a symbol or once in a synoptic view, no text converted to paths; provisional, to confirm by the Product Owner at VAL-017). Number reserved by ADR-0052, decision 8. Adds `switch`, `foreignObject` and `style` to the SVG elements allowed (ADR-0002, ADR-0052, decision 7), each in one place only.

## Context

- **The refinement.** The library computes a text's lines and draws them for every SVG reader (ADR-0052); a browser should also lay the text out live with its own fonts, Arabic and Indic scripts among them, which the library does not shape; a font may be embedded in a symbol, and once in a view whatever the number of its symbols using it (F05, round 2, 2026-10-14).
- **`switch` and `foreignObject`.** `switch` renders "the first child for which these attributes evaluate to true" (`REF-SVG2-STRUCT`, §5.6.3); a `requiredExtensions` not supported skips its element (§5.6.4). SVG 2 pairs `foreignObject` with `switch` and `requiredExtensions` "to provide an alternate rendering", but "does not define how 'requiredExtensions' values should be mapped" (`REF-SVG2-EMBEDDED`, §12.4). The three browser engines accept the XHTML namespace `http://www.w3.org/1999/xhtml` as an extension (`REF-BLINK-SVG-TESTS`, `REF-GECKO-SVG-TESTS`, `REF-WEBKIT-SVG-TESTS`), Baseline since 2020 (`REF-MDN-REQUIRED-EXTENSIONS`). resvg fails any `requiredExtensions` (`REF-USVG-SWITCH`), Batik supports no extension and has no `foreignObject` bridge (`REF-BATIK-SWITCH-CODE`), Inkscape fails any value (`REF-INKSCAPE-CONDITIONS`): each draws the next child. `switch` passes 92 % of its tests in Chrome, Firefox and Batik, 100 % in resvg, 69 % in Safari and Inkscape (`REF-RESVG-SUPPORT-TABLE`).
- **Embedded fonts.** `@font-face` rules are "considered before other available fonts on a system" (`REF-CSS-FONTS-4`, §4.1); a `data:` URL is the only way a font reaches an SVG used as an image (`REF-MDN-SVG-AS-IMAGE`, `REF-RFC-2397`). The `style` element holds a style sheet, never rendered (`REF-SVG2-STYLING`, §6.2). Batik loads a TrueType font from `@font-face` (`REF-BATIK-FONT-FACE-CODE`) but refuses a `src` holding `local(…)` (`REF-BATIK-1159`); resvg does not read `@font-face` (`REF-RESVG-541`); Inkscape's support was incomplete in 2026 (`REF-INKSCAPE-MR-7806`). A font may be embedded only when its `fsType` permits it (`REF-OT-OS2`, `DERIV-font-embedding`).

## Decision

1. **The HTML layer**, an option of a document (a symbol or a view), off unless turned on: each text is written as one `switch` holding, in this order,
   - a `foreignObject` with `requiredExtensions="http://www.w3.org/1999/xhtml"`, at the text's block (or box), holding one XHTML `div` with the text's content and the CSS of its options (EN-041);
   - the text's group as ADR-0052, ADR-0054 and ADR-0055 write it, unchanged: its background, `<text>`, lines and decorations.
     A browser draws the first, every other reader the second. The `switch` sits inside the group that turns the text, so that both turn alike.
2. **Embedded fonts**, an option of a document, off unless turned on: one `<style>` inside the document's `<defs>`, holding one `@font-face` per face used — family name, weight, style, `src: url(data:font/ttf;base64,…) format("truetype")` (`font/otf` and `"opentype"` for a CFF file), no `local()` — each face once, whatever the number of its texts (`DERIV-font-embedding`, step 7). A face whose `fsType` does not permit embedding is not written, and the editor says so (`DERIV-font-embedding`, step 3). A face whose file the editor does not hold is not written either: the library ships Inter's metrics, not its files, so Inter, whose licence allows embedding (`REF-INTER-LICENSE`), is embedded once its file is imported.
3. **Allowed SVG elements**: `svg`, `g`, `path`, `text`, `tspan`, `defs`, and
   - `style`, only inside `defs`, holding only `@font-face` rules;
   - `switch`, only around one text, its first child the HTML layer and its second the text's group;
   - `foreignObject`, only as that first child, with that `requiredExtensions`, holding only XHTML `div` and `span` elements and text.
4. **Never read back.** The editor measures with the metrics of the font files it reads (ADR-0052); it never measures nor edits with a font embedded in a file, nor reads the HTML layer (ADR-0005).

## Reasons

| Option                                         | For                                                                                               | Against                                                                                                       |
| ---------------------------------------------- | ------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------- |
| `switch` with the XHTML extension (chosen)     | browsers lay out live; resvg, Batik and Inkscape draw the positioned lines, as read in their code | two copies of each text; the extension's value is not defined by SVG 2, only by what the engines do           |
| `foreignObject` alone                          | one copy                                                                                          | "SVG editors cannot display it" (`REF-DRAWIO-SVG-TEXT`); nothing in resvg, Batik or Inkscape                  |
| `requiredFeatures` instead                     | older SVG 1.1 readers                                                                             | SVG 2's conditional processing names only `requiredExtensions` and `systemLanguage` (`REF-SVG2-STRUCT`, §5.6) |
| Fonts as `data:` URLs in `@font-face` (chosen) | browsers and Batik draw them; the SVG stands alone, also as an image                              | a third more bytes than the file (`DERIV-font-embedding`, step 6); resvg ignores them                         |
| Fonts linked by URL                            | small SVG                                                                                         | not loaded in an SVG used as an image (`REF-MDN-SVG-AS-IMAGE`); a file that depends on another                |
| Text converted to paths                        | the same drawing everywhere                                                                       | not wanted by the Product Owner (ADR-0052)                                                                    |
| Both options off by default (chosen)           | every text of F05, F14 and F16 is written as before; no font sent without a choice                | a designer turns them on for each document                                                                    |

## Consequences

- `AGENTS.md`'s Never line lists the three elements with their place; ADR-0052's decision 7 is extended, not rewritten.
- A browser's lines may differ from the library's: other wraps, a font the reader lacks, its own vertical metrics (research note 0020). The library's measured block stays the text's block for selection, turning and layout.
- Q24 (a `foreignObject` tool) stays open: this ADR allows a `foreignObject` only as a text's HTML layer.
- Q39 (`docs/domain/README.md`) records F15's provisional rules; `shapes.md` (Text) states them.

## References

`REF-SVG2-STRUCT`, `REF-SVG2-EMBEDDED`, `REF-SVG2-STYLING`, `REF-MDN-REQUIRED-EXTENSIONS`, `REF-BLINK-SVG-TESTS`, `REF-GECKO-SVG-TESTS`, `REF-WEBKIT-SVG-TESTS`, `REF-USVG-SWITCH`, `REF-BATIK-SWITCH-CODE`, `REF-INKSCAPE-CONDITIONS`, `REF-RESVG-SUPPORT-TABLE`, `REF-CSS-FONTS-4`, `REF-MDN-SVG-AS-IMAGE`, `REF-RFC-2397`, `REF-BATIK-FONT-FACE-CODE`, `REF-BATIK-1159`, `REF-RESVG-541`, `REF-INKSCAPE-MR-7806`, `REF-OT-OS2`, `REF-INTER-LICENSE`, `REF-DRAWIO-SVG-TEXT`, ADR-0002, ADR-0005, ADR-0052, ADR-0054, ADR-0055, `DERIV-font-embedding`, [research note 0020](../research/notes/0020-f15-fonts-and-html-layer.md)
