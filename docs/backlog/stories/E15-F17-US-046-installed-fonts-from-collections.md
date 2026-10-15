---
id: US-046
epic: E15
feature: F17
title: Installed fonts read from collections, and faces that failed
status: done
points: 3
---

# E15 · F17 · US-046 — Installed fonts read from collections, and faces that failed

As a symbol designer, I want every installed font I choose to be read so that I can use the fonts of my computer.

From the Product Owner's validation of E15 (2026-10-15): installed faces held in a collection were refused as "not a TrueType or OpenType font, collection among them", and some faces failed with a `TypeError`. Causes found by SP-019 ([research note 0021](../../research/notes/0021-f17-latin-script-and-defects.md)):

- **Refused faces**: on macOS, Chrome gives a face as one SFNT that Skia builds from its tables, its `sfntVersion` set from CoreText's format; faces of format `kCTFontFormatTrueType` — Helvetica, Helvetica Neue, Menlo, Courier, Lucida Grande, Times, all held in `.ttc` files — start with `true` (0x74727565), which the library refuses, reading only `0x00010000` and `OTTO` (`REF-SKIA-MAC-STREAM-CODE`, `REF-APPLE-TT-SCALER`). Avenir, also in a `.ttc`, is of OpenType format and read already. An imported `.ttc`, and an installed one where the browser gives the file whole, starts with `ttcf` and is refused too (`REF-OT-OTFF`).
- **`TypeError`**: Chrome rejects `blob()` with "The font X could not be accessed." when it cannot find the face by its PostScript name, or "Font data for X could not be accessed." when Skia gives no stream; the page has no other way to read those bytes (`REF-CHROMIUM-FONT-BLOB-CODE`).

## Acceptance criteria

- Given a face file whose `sfntVersion` is `true` (0x74727565), when it is read, then it is read as TrueType (`REF-APPLE-TT-SCALER`): Helvetica, chosen in the installed fonts on macOS, gives "Helvetica: 6 faces read." (its `.ttc` holds 6 faces: 300, 400 and 700, upright and oblique).
- Given a collection file (`ttcf`, `REF-OT-OTFF`), imported or given whole by the browser, when it is read, then each face is read at its `tableDirectoryOffsets[i]`, its table offsets counted from the start of the file: Helvetica.ttc (version 2.0) gives 6 faces, at 48, 380, 696, 1028, 1344 and 1644; a face given by the browser keeps only the face of its PostScript name.
- Given a face read from a collection, when it is drawn, then the page gives `FontFace` that face alone, as a standalone SFNT whose tables are copied and directory offsets rewritten, and the text is measured and drawn with it.
- Given an installed face whose `blob()` is rejected with a `TypeError`, when its family is chosen, then it is refused with one line naming the face and Chrome's reason ("could not be accessed"), proposing to import its file, the other faces kept, and no error left uncaught.
- Given a collection holding faces of several families, when one family is chosen, then only that family's faces are kept.

## Product Owner test

| Step | Do                                                             | You should see                           |
| ---- | -------------------------------------------------------------- | ---------------------------------------- |
| 1    | List the installed fonts, choose Helvetica (a `.ttc` on macOS) | "Helvetica: 6 faces read.", none refused |
| 2    | Choose the family that failed with a `TypeError`               | its faces read, or one line saying why   |

## Tasks

One task = one commit, referenced as `US-046.Tn`.

- [x] T1 — The `true` tag read as TrueType, and a face read from a collection in the library (`ttcf` header, its offsets, a face written as a standalone SFNT), with tests on Helvetica.ttc's numbers (3 h)
- [x] T2 — The playground reads installed faces from collections by PostScript name, gives `FontFace` one face, and says the `TypeError` in one line (2 h)
- [x] T3 — `shapes.md`, the test card and the guided step (1 h)
