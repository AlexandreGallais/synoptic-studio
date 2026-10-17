# 0020 — Sources of F15: fonts from files, embedded fonts and the HTML layer

Spike SP-018, 2026-10-15, for F15 (E15). F05's refinement, round 2, gave the business rules: a font named with a fallback; TrueType and OpenType files, imported or installed, read once and kept; an option embeds a font in a symbol, and once in a synoptic view; no text converted to paths; Arabic and Indic scripts laid out by the browser's HTML layer. Every page below was read in the local library before it was cited; browser and reader behaviors are read in their code where their documentation is silent (level 5, read only). The rules decided here are provisional, to confirm at VAL-017 (ADR-0053, `shapes.md`, Q39).

## A font file read by the page (EN-039, US-036)

| Finding | Source |
| --- | --- |
| A file input gives `File` objects; `accept` takes extensions such as `.ttf` and `.otf`; `arrayBuffer()` gives a file's bytes | `REF-MDN-INPUT-FILE`, `REF-W3C-FILE-API` |
| `queryLocalFonts()` needs a click (transient activation), the `local-fonts` permission and a secure context; a `FontData` gives its family, style and `blob()`, the SFNT bytes | `REF-WICG-LOCAL-FONT-ACCESS`, `REF-MDN-QUERYLOCALFONTS` |
| Chrome and Edge from version 103 only, not Firefox nor Safari; the permission persists between reloads | `REF-CHROME-LOCAL-FONTS`, `REF-MDN-QUERYLOCALFONTS` |
| `http://localhost` is a potentially trustworthy origin: the playground may call it | `REF-MDN-SECURE-CONTEXTS` |
| The family is name ID 16 when present (extended families), else name ID 1; Windows strings in UTF-16BE | `REF-OT-NAME` |
| The weight is `usWeightClass`, 1 to 1000, 100 to 900 in common use; italic is `fsSelection` bit 0 | `REF-OT-OS2` |

An installed font goes through the same reader as a file: its `blob()` holds the same bytes, so its names come from its `name` table, as an imported file's.

## Embedding permissions (EN-039)

`fsType` bits 0 to 3: 0 installable, 2 restricted, 4 preview and print, 8 editable; bit 8 no subsetting, bit 9 bitmap only; versions 0 to 2 take the least restrictive of several bits, versions 0 and 1 ignore bits 4 to 15 (`REF-OT-OS2`). The rule for a whole file, outlines included, is `DERIV-font-embedding` step 3: refused when bitmap only (from version 2) or restricted without 4 nor 8. Arial 5.01 (8, editable), Times New Roman and Verdana (8), Inter (0) are embeddable, read from their files.

## Fonts written into the SVG (EN-040)

| Finding | Source |
| --- | --- |
| `@font-face` faces "are considered before other available fonts on a system"; `src: url(…) format(…)`; `truetype` and `opentype` "must be considered as synonymous"; `font/ttf`, `font/otf` | `REF-CSS-FONTS-4`, §4.1, §4.3, §11.2 |
| `data:[mediatype][;base64],data`; base 64 turns 3 bytes into 4 characters, padded with `=` | `REF-RFC-2397`, `REF-RFC-4648` |
| An SVG used as an image loads no external resource, but `data:` URLs work | `REF-MDN-SVG-AS-IMAGE` |
| The `style` element embeds a style sheet, never rendered, CSS when `type` is absent | `REF-SVG2-STYLING`, §6.2 |
| Batik loads an `@font-face` font through Java's TrueType reader; a `src` holding `local(…)` is refused, a `data:` URL alone works | `REF-BATIK-FONT-FACE-CODE`, `REF-BATIK-1159` |
| resvg does not read `@font-face` (issue open; its CSS parser learnt the rule in 2025) | `REF-RESVG-541`, `REF-RESVG-UNSUPPORTED` |
| Inkscape's import of embedded `@font-face` fonts was "incomplete" in 2026; a merge request was open | `REF-INKSCAPE-MR-7806` |

A file grows by a third in base 64: Arial's 773 236 bytes become a URL of 1 031 005 characters (`DERIV-font-embedding`, step 6). A document writes each face once (step 7); once in a synoptic view for all its symbols comes with the views (E08).

## The HTML layer (EN-041)

| Finding | Source |
| --- | --- |
| `switch` renders its first child whose conditions are true; an unsupported `requiredExtensions` skips its element; absent, it is true | `REF-SVG2-STRUCT`, §5.6.3, §5.6.4 |
| `foreignObject` with `switch` and `requiredExtensions` gives "an alternate rendering"; SVG 2 does not map extension values to languages | `REF-SVG2-EMBEDDED`, §12.4 |
| Blink, Gecko and WebKit accept `http://www.w3.org/1999/xhtml` (and MathML) as an extension; `requiredExtensions` is Baseline since 2020 | `REF-BLINK-SVG-TESTS`, `REF-GECKO-SVG-TESTS`, `REF-WEBKIT-SVG-TESTS`, `REF-MDN-REQUIRED-EXTENSIONS` |
| resvg fails every `requiredExtensions`; Batik supports no extension and has no `foreignObject` bridge; Inkscape fails every value: all three draw the next child | `REF-USVG-SWITCH`, `REF-BATIK-SWITCH-CODE`, `REF-INKSCAPE-CONDITIONS` |
| `switch` passes 92 % of its tests in Chrome, Firefox, Batik; 100 % resvg; 69 % Safari, Inkscape; 62 % librsvg; 8 % SVG.NET | `REF-RESVG-SUPPORT-TABLE` |
| `white-space: pre` keeps spaces and breaks only at line breaks; `pre-wrap` also wraps, the spaces at a wrap hanging | `REF-MDN-WHITE-SPACE` |
| `overflow-wrap: anywhere` breaks a word too long at any point, without a hyphen | `REF-MDN-OVERFLOW-WRAP` |
| `-webkit-line-clamp` with `display: -webkit-box` and `-webkit-box-orient: vertical` cuts at a number of lines with "…", kept supported; `line-clamp` itself is not Baseline | `REF-MDN-LINE-CLAMP` |
| `text-overflow: ellipsis` acts only on a line that overflows across, not at the bottom of a box | `REF-MDN-TEXT-OVERFLOW` |
| `text-wrap: balance`; `text-decoration-thickness: from-font` | `REF-MDN-TEXT-WRAP`, `REF-MDN-TEXT-DECORATION-THICKNESS` |
| With a `line-height`, a line's leading is split half above and half below; with `normal`, the line gap may be added half on each side | `REF-CSS-INLINE-3`, §5.3 |
| macOS browsers use `hhea` (Firefox the typographic metrics when `USE_TYPO_METRICS` is set); Windows browsers the typographic metrics when it is set, else `win` | `REF-GLYPHS-VERTICAL-METRICS`, `REF-GF-VERTICAL-METRICS` |

The layer gets the library's line height in user units, so its lines are spaced as the library's; its first baseline then meets the library's except for an automatic line height, where the browser puts half the line gap above the first line: the layer is placed that much higher (`DERIV-html-layer`). Arial 16: 0.26171875; Inter 4.1, whose line gap is 0: none.

The layer is the browser's own layout: where its font or its wrapping differs, its lines differ from the library's. The library's block stays the text's block.

## Vertical metrics of some fonts (read from their files)

| Font | `hhea` ascender, descender, gap | `OS/2` typographic | `win` ascent, descent | `USE_TYPO_METRICS` |
| --- | --- | --- | --- | --- |
| Arial 5.01 (2048 units) | 1854, −434, 67 | 1491, −431, 307 | 1854, 434 | no |
| Times New Roman 5.01 (2048) | 1825, −443, 87 | 1420, −442, 307 | 1825, 443 | no |
| Verdana 5.01 (2048) | 2059, −430, 0 | 1566, −423, 202 | 2059, 430 | no |
| Inter 4.1 (2048) | 1984, −494, 0 | 1984, −494, 0 | 1984, 494 | yes |

For these three system fonts, `hhea` and `win` agree on ascent and descent, so a browser places their lines alike on macOS and Windows; their typographic values, which would give other line heights, are unused since the flag is unset. Inter 4.1, read from `Inter-4.1.zip` (`extras/ttf/`, Regular, Bold, Italic, Thin and Black alike), sets the flag and gives equal `hhea` and typographic values: browsers place its lines alike everywhere.

## Decisions of the spike (provisional, to confirm at VAL-017)

Written in `docs/domain/shapes.md` (Text, Fonts), Q39 and ADR-0053:

| Decision | Reading |
| --- | --- |
| `switch`, `foreignObject` with the XHTML extension, `style` in `defs`, each in one place only | the code of the three browser engines and of resvg, Batik and Inkscape |
| The HTML layer and the embedding are options of a document, off by default | every text of F05, F14 and F16 written as before; no font sent without a choice |
| A face's family from name ID 16, else 1; its weight `usWeightClass` brought into 100 to 900; italic from `fsSelection` bit 0 | `REF-OT-NAME`, `REF-OT-OS2`; the model's weights |
| A face read again (same family, weight and style) replaces the one kept | the agent's choice: the last file chosen wins |
| An installed font is read from its bytes, as a file | one reader, one naming |
| Fonts are kept for the session in the playground; across sessions with the editor's files (E02) | the agent's reading of "kept": no storage asked for before the editor's documents |
| Embeddable: not bitmap only, not restricted without 4 nor 8; preview and print embeddable, the editor never editing with an embedded font | `DERIV-font-embedding` step 3; ADR-0053 decision 4 |
| A face the editor holds no file for (Inter, shipped as metrics only) is not embedded, and the editor says so | the library ships Inter's metrics, not its 18 files |
| `src` as one `data:` URL, no `local()` | `REF-BATIK-1159` |
| The layer's lines spaced by the library's line height, placed by `DERIV-html-layer` | `REF-CSS-INLINE-3`; the vertical metrics browsers use |

## Points left by F05

| Point | Outcome |
| --- | --- |
| Inter has no Hebrew | settled by F15: a font with Hebrew is imported or chosen among the installed ones (Arial 5.01 maps U+05D0, read from its `cmap`); in a browser, the HTML layer falls back to a font that has it |
| `hhea` against `OS/2` typographic line metrics | settled for the HTML layer: the library keeps `hhea`, which macOS browsers use, and Windows browsers use for the fonts above; the layer's line height is the library's (`DERIV-html-layer`) |
| Right to left in resvg and Batik | open: neither reads `direction` nor `unicode-bidi` (`REF-RESVG-UNSUPPORTED`, `REF-RESVG-SUPPORT-TABLE`), so they draw a right-to-left line from its right edge rightwards, one line width too far right; browsers draw it right, and so does the HTML layer. Options for VAL-017: keep; or write such a line from its left edge without `direction`, at the risk of another order for a line mixing scripts |
| Inter's data, 1.6 MB | open, not F15: 307 KB compressed with gzip (measured); splitting it by face, read on demand, is an improvement for the retrospective |
| `GPOS` read for the common case only (AUD-015) | open: an imported font kerning through those cases is measured a little off; the HTML layer gives a browser its own kerning. Not a story of F15 |
| Letter spacing after the last letter, the underline's position | measured in three browsers at VAL-014 (steps 33 and 34); the HTML layer uses the browser's own (`letter-spacing`, `text-decoration`) |

## Retained

| Story | Uses |
| --- | --- |
| EN-039 | `REF-OT-NAME`, `REF-OT-OS2`, `REF-W3C-FILE-API`, `DERIV-font-embedding` steps 1 to 3, `DERIV-bit-of-integer` |
| EN-040 | `DERIV-font-embedding` steps 4 to 7, `REF-RFC-4648`, `REF-RFC-2397`, `REF-CSS-FONTS-4`, `REF-SVG2-STYLING`, `REF-BATIK-1159`; ADR-0053 |
| EN-041 | `DERIV-html-layer`, `REF-SVG2-STRUCT`, `REF-SVG2-EMBEDDED`, `REF-BLINK-SVG-TESTS`, `REF-MDN-WHITE-SPACE`, `REF-MDN-OVERFLOW-WRAP`, `REF-MDN-LINE-CLAMP`, `REF-MDN-TEXT-WRAP`; ADR-0053 |
| US-036 | `REF-MDN-INPUT-FILE`, `REF-WICG-LOCAL-FONT-ACCESS`, `REF-CHROME-LOCAL-FONTS`, `REF-MDN-SECURE-CONTEXTS` |

## Open

- Inkscape and embedded fonts: tried at VAL-017 with an exported SVG; the merge request read says its import was incomplete.
- Batik and a CFF OpenType file: Java's `Font.createFont(TRUETYPE_FONT, …)` reads it or not; not read.
- A browser's own wrapping, ellipsis and balance may differ from the library's (`text-overflow` acts across only; CSS clips the ellipsis that the library draws past the box): listed for VAL-017.
- WOFF and WOFF2 files, and collections (`.ttc`): not asked; refused as in F05.
- Scrolling a long text in its box on a touch screen (F14): the HTML layer could carry it; with the views (E08).
