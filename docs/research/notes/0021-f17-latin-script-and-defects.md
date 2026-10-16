# 0021 — Sources of F17: the Latin script in industrial HMIs, and the causes of the defects

Spike SP-019, 2026-10-15, for F17 (E15). The validation of E15 (VAL-014 to VAL-017) removed the HTML layer on the reason that industrial synoptic views are written in Latin script (ADR-0060), and found five defects whose causes had to be found before any fix. Every page below was read in the local library before it was cited; browser behaviors are read in their code where their documentation is silent (level 5, read only). No browser could be run on the machine of the spike: each cause is reproduced in the library's own model, in the font files of the machine, or in CoreText, and what remains to be seen in a browser is said.

## The scripts of industrial HMIs (ADR-0060)

| Finding                                                                                                                                                                                                                | Source                      |
| ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------- |
| A SCADA vendor's views are translated per user: "Localization lets you display projects in multiple languages", "users in different regions" seeing "the same project in their preferred language"; no script is named | `REF-IGNITION-LOCALIZATION` |
| ISA-101, the HMI standard, is paid; no readable edition nor free part on text or scripts was found                                                                                                                     | `REF-ISA-101` [unverified]  |

**No readable source says that industrial HMIs mostly use the Latin script**, nor which other scripts appear and where. The one vendor manual read treats languages as a translation of the same views, chosen by each user, which means a view may be shown in a script other than the one it was drawn in. Vendor pages on Chinese, Japanese, Korean and Arabic in HMI runtimes were found only on third-party blogs and resellers' listings, not read as sources (research protocol: a blog never alone).

So ADR-0060's reason stands as the Product Owner's scope decision (the editor targets the Latin script; Greek, Cyrillic, Hebrew and right to left kept), not as a fact the sources confirm. The question is left to the Product Owner in SP-019's report; nothing in F17 depends on it, since ADR-0060 removes the HTML layer whatever the answer.

## Installed faces refused as "not a font" (US-046)

The playground said "not a TrueType or OpenType font, a collection among them" for some installed faces. The words "a collection among them" are only part of the playground's one message for every file `parseFontFace` refuses (`playground/fonts/read-installed-family.ts`); they do not say that a collection was read.

| Finding                                                                                                                                                                                                                                       | Source                                               |
| --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------- |
| `blob()` gives a face's data bytes, expected to be the font file as it is on disk, not normalized                                                                                                                                             | `REF-WICG-LOCAL-FONT-ACCESS`, §4.1                   |
| Chrome gives the whole stream Skia opens for the face; the collection index it receives is not used                                                                                                                                           | `REF-CHROMIUM-FONT-BLOB-CODE`                        |
| On macOS, Skia does not give the file: it builds one SFNT from the face's tables, so a face held in a `.ttc` comes as a single font; its `sfntVersion` follows CoreText's format, `true` for `kCTFontFormatTrueType`                          | `REF-SKIA-MAC-STREAM-CODE`                           |
| The library reads only `0x00010000` and `OTTO` (`src/io/font-tables/parse-table-directory.ts`), as OpenType asks; Apple's manual makes `true` a TrueType font too, and OTS, Blink's sanitizer, accepts `true` and rewrites it to `0x00010000` | `REF-OT-OTFF`, `REF-APPLE-TT-SCALER`, `REF-OTS-CODE` |
| A collection file: a `ttcf` header, then one table directory per face at `tableDirectoryOffsets[i]`, every table offset counted from the start of the whole file, tables shared between faces                                                 | `REF-OT-OTFF`, font collections                      |
| OTS given a collection without an index returns the whole collection; Blink's decoder gives no index                                                                                                                                          | `REF-OTS-CODE`                                       |

**Reproduction** (CoreText on the spike's macOS, `kCTFontFormatAttribute` of each face): Helvetica, Helvetica Neue, Menlo, Courier, Lucida Grande and Times, all in `.ttc` files, are of format 3, `kCTFontFormatTrueType`; Avenir (`.ttc`), Arial (`.ttf`) and IBM Plex Sans (`.ttf`) are of format 2, `kCTFontFormatOpenTypeTrueType`. The files themselves start with `0x00010000` inside the collection (Helvetica.ttc: `ttcf` version 2.0, 6 faces, directories at 48, 380, 696, 1028, 1344 and 1644, each directory's first table past byte 1900, so offsets from the file's start). The library refuses Helvetica.ttc read whole (`parseFontFace` gives `undefined`). Each of its 6 faces, written as a standalone SFNT as Skia assembles one (tables copied, padded to 4 bytes, offsets rewritten), is read with `0x00010000` — Helvetica 400, 700, 400 italic, 700 italic, 300, 300 italic — and refused when its first 4 bytes are set to `true`, as Chrome gives them on macOS.

**Cause**: on macOS, the blob of a face whose CoreText format is TrueType starts with `true` (0x74727565), which `parseTableDirectory` refuses. Those faces are mostly the system's `.ttc` faces, hence the refusal seen on collections; a face held in a `.ttc` whose format is OpenType (Avenir) is read already. On Windows and Linux, Skia opens the font file itself (not read in this spike): a `.ttc` there would come whole, starting with `ttcf`, the face's index lost. An imported `.ttc` is refused the same way today, `parseFontFace` reading a directory at byte 0 only.

**Fix for US-046**: accept `true` as TrueType (`REF-APPLE-TT-SCALER`); read a collection's faces at their offsets (`REF-OT-OTFF`), table offsets from the file's start, each face kept with its family; give `FontFace` one face as a standalone SFNT, its tables copied and its directory's offsets rewritten as Skia does, since a collection given to `FontFace` is sanitized whole, no index chosen (`REF-OTS-CODE`; which face Blink then draws is to see in a browser). An installed face whose blob is a whole collection is matched to its face by its PostScript name (name ID 6, as `FontData.postscriptName`).

## Faces whose access fails with a `TypeError` (US-046)

| Finding                                                                                                                                                                                                                               | Source                            |
| ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------- |
| Chrome rejects `blob()` with a `TypeError` in two cases only: "The font X could not be accessed." when the face is not found by its PostScript name, and "Font data for X could not be accessed." when Skia gives no stream in memory | `REF-CHROMIUM-FONT-BLOB-CODE`     |
| Chrome's own note: "openStream rarely fails, but it happens sometimes" (`crbug.com/1086840`)                                                                                                                                          | `REF-CHROMIUM-FONT-BLOB-CODE`     |
| On macOS a listed face is named by CoreText (`kCTFontNameAttribute`), duplicates removed by PostScript name                                                                                                                           | `REF-CHROMIUM-FONT-ENUM-MAC-CODE` |

**Cause**: the browser cannot give that face's bytes, and the page has no other way to read them. The playground writes the reason as `String(reason)`, so the Product Owner's line held one of the two messages above with the face's PostScript name; which faces is not known from the validation (open: the line to copy at US-046's guided test). **Fix**: refuse the face with that cause in one line naming it, the other faces kept, and offer to import its file; no error left uncaught.

## Weight 450 measured but drawn with another face (US-047)

| Finding                                                                                                                                                                                                                | Source                            |
| ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------- |
| On macOS a listed face's family is CoreText's family name: IBM Plex Sans Text (name ID 1 "IBM Plex Sans Text", ID 16 "IBM Plex Sans", `usWeightClass` 450) is listed in "IBM Plex Sans", with the family's other faces | `REF-CHROMIUM-FONT-ENUM-MAC-CODE` |
| The page registers each face read as `FontFace(family, bytes, { weight: "450", style })`, the family from name ID 16 (`playground/fonts/add-font-file.ts`)                                                             | `REF-OT-NAME`, `REF-OT-OS2`       |
| CSS matching for 450: weights from 450 up to 500 first, then below 450 descending; faces defined by `@font-face` (or `FontFace`) shadow the installed family of the same name                                          | `REF-CSS-FONTS-4`, §5.2, §10.2    |

**Reproduction**: the library measures "Pump P-101" at 16 as 90.688 with Regular (400), 91.264 with Text (450) and 91.92 with Medium (500), from the files installed on the spike's machine; the 450 face is 0.576 wider than 400 and 0.656 narrower than 500, so a width read in a browser tells the face drawn within 0.5 of a unit. Read in the code, the path matches 450 once the 450 `FontFace` has loaded. Two causes remain, which only a browser can tell apart:

1. **The 450 face's load failed, unseen**: `registerFontFace` runs `face.load()` in `runTask` with `update` as its error handler, so a failed load says nothing; CSS then matches 500 (§5.2), drawn 0.656 wider than measured.
2. **The drawing comes before the load**: the drawing names the family at once; until its faces load, the browser draws another face, then redraws when the load ends (`update`).

**Fix for US-047**: say a face's failed load in one line (its name and the browser's reason); a test page drawing "Pump P-101" in IBM Plex Sans at 400, 450 and 500 compares each drawn width (`getComputedTextLength()`) with the measured one, within 0.5; the cause seen there written in the story.

## The lag of a value changed fast (US-048)

Every `input` event redraws at once: `mountPlayground`'s `update` runs `updateTextTool` whole, synchronously, at each event. An arrow key held sends about 30 events a second; typing sends fewer, which is why typing feels fluid while a held key lags: when one redraw lasts longer than the time between two events, the events queue and the drawing follows the key late, and goes on after it is released.

What one redraw of a text does, read in the code:

| Step                                                                                                                                                                   | Cost measured (Node.js, the spike's machine)                                           |
| ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------- |
| `writeFontFields`: `writeFontOptions` lists the families of the session, each name compared with every name before it (`findIndex` with `isSameFamilyName`), quadratic | 6.1 ms for 590 installed faces in 186 families (CoreText's count here); 15 ms for 1000 |
| `writeSelectOptions` compares the font list's options with the names, joined, and `writeWeightChoice` writes the weight list                                           | not measured (DOM)                                                                     |
| `textFontMetrics`, then `textDrawing` in `updateTextTool`, and `textDrawing` again in `showText`                                                                       | 0.052 ms each for a wrapped text of 5 words                                            |
| `createTextDocument`: with **Embed fonts** on, each face used written as a `data:` URL in base 64, at every redraw (ADR-0060's reason)                                 | 21 ms for one 200 500-byte face (IBM Plex Sans Regular), before the browser decodes it |
| `getBoundingClientRect` on the canvas after the previous writes, the SVG replaced whole, the model, block and pieces written as JSON, the SVG serialized for display   | not measured (DOM)                                                                     |

The text's own computation is small (0.1 ms); the time goes to what is redone though its inputs did not change: the font list, rewritten at each event, and the embedded fonts. A profile in the browser, before and after, is US-048's first and last measure.

| Finding                                                                                                                                                                       | Source        |
| ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------- |
| `requestAnimationFrame` calls back once "before the next repaint", at about the display's rate (60 Hz), one-shot; a redraw asked several times before it is gathered into one | `REF-MDN-RAF` |

**Fix for US-048**: an `input` event asks a redraw, one at most per frame (a request ID held, cleared when the frame runs); the font list written only when the session changes, its names deduplicated in one pass by their case-folded name; the metrics kept by their font, weight and style, the lines and block by the text's content, font, size, spacing and box; `textDrawing` computed once per redraw; the embedded fonts written only at export (US-045).

## The background band past a wrapped line's end (US-049)

**Reproduction** in the model (`textDrawing`, Arial 16, `ARIAL_BOX_METRICS`, a box 50 wide, background on):

| Content                    | Lines and widths                | Bands (x, width)                       |
| -------------------------- | ------------------------------- | -------------------------------------- |
| `Hello World`              | "Hello" 36.461, "World" 41.781  | (0, 36.461), (0, 41.781)               |
| `Hello World ` (one space) | "Hello" 36.461, "World " 46.227 | (0, 36.461), (0, 46.227)               |
| `Hello World  `            | "Hello" 36.461, "World " 50.672 | (0, 36.461), (0, 50.672), past the box |
| `Hello World `, right      | "World " at 3.773, 46.227 wide  | (3.773, 46.227)                        |

**Cause**: a line ended by a wrap is cut of its trailing spaces (`endedLine`, `withoutTrailingSpaces`), but the last line of a paragraph keeps them (`wrappedParagraph` returns `line.text` as it is). Its width, and so its band, counts each space: 4.445 (569 units of 2048 at 16) per space past "World". While a word is typed at the end of a wrapped text, the space typed before it makes the band of the new line run past its end, to the right. `DERIV-text-box`, step 2, chose this: "The last line of a paragraph keeps the spaces typed, drawn and measured as in F05 for its placement", as CSS `pre-wrap` does before a forced break, the spaces hanging only when they would overflow (`REF-CSS-TEXT-3`, §4.1.2). The same spaces also shift a right-aligned or centred last line by their width (3.773 instead of 8.219 for "World" alone).

**Fix for US-049**: the band of a line ends at its last character other than a space. Whether the last line's trailing spaces still place the line (alignment, the block's width) is a rule of `DERIV-text-box`, step 2, which the band alone does not settle: two readings, for the Product Owner at US-049 (left open, no option chosen here):

1. **Spaces hang at the end of every line in a box**, as CSS `white-space: normal` (`REF-CSS-TEXT-3`, §4.1.2): "World " right-aligned in a box 50 sits at 8.219, its band 41.781 wide.
2. **Spaces kept for placement, band trimmed**: "World " right-aligned sits at 3.773, its band 41.781 wide, a space's width left between the word and the box's edge.
