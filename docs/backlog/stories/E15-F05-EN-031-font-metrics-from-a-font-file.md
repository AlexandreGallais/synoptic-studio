---
id: EN-031
epic: E15
feature: F05
title: Font metrics read from a TrueType or OpenType file
status: ready
points: 3
---

# E15 · F05 · EN-031 — Font metrics read from a TrueType or OpenType file

The library reads a font file once into its metrics: the advance of each character, its kerning pairs, its units per em and its vertical metrics, so that measuring a text is then a sum of numbers.

## Acceptance criteria

- Given Arial's file, when its metrics are read, then the advances of H, e, l, o, space, W, r, d are 1479, 1139, 455, 1139, 569, 1933, 682, 1139 for 2048 units per em.
- Given a TrueType (`glyf`) or an OpenType (`CFF`) file, when it is read, then `cmap` formats 4 and 12, `hmtx`, `hhea`, `head`, `OS/2` and the kerning pairs (`kern` format 0, `GPOS` pair adjustment) give the metrics; another table is ignored.
- Given a character missing from the font, when its advance is asked, then it is the advance of glyph 0 (`.notdef`).
- Given the default font's files, when a script runs, then their metrics are written to a data file of the repository, read by the playground and the tests without the font files.

## Tasks

One task = one commit, referenced as `EN-031.Tn`.

- [ ] T1 — The font file reader: directory, head, hhea, maxp, hmtx, cmap; tests first with hand-computed values (3 h)
- [ ] T2 — Kerning pairs from `kern` and `GPOS`; vertical metrics from `OS/2` (2 h)
- [ ] T3 — The default font's metrics as a data file, and the script that writes it (1 h)
