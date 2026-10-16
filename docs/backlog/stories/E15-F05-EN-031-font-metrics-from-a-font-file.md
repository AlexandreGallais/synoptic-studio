---
id: EN-031
epic: E15
feature: F05
title: Font metrics read from a TrueType or OpenType file
status: done
points: 3
---

# E15 · F05 · EN-031 — Font metrics read from a TrueType or OpenType file

The library reads a font file once into its metrics: the advance of each character, its kerning pairs, its units per em and its vertical metrics, so that measuring a text is then a sum of numbers.

## Acceptance criteria

- Given a font built byte by byte in a test (table directory, `head`, `hhea`, `maxp`, `hmtx`, `cmap` format 4), when it is read, then its units per em, advances, ascender, descender and line gap come back as written (`REF-OT-OTFF`, `REF-OT-HEAD`, `REF-OT-HHEA`, `REF-OT-HMTX`, `REF-OT-CMAP`).
- Given Arial's file, where the machine has it, when its metrics are read, then the advances of H, e, l, o, space, W, r, d are 1479, 1139, 455, 1139, 569, 1933, 682, 1139 for 2048 units per em, and the pair W o kerns −37 (`DERIV-text-width`, cross-check).
- Given a TrueType (`0x00010000`) or an OpenType (`OTTO`) file, when it is read, then `cmap` formats 4 and 12, `hmtx`, `hhea`, `head`, `post`, `OS/2` and the kerning pairs (`kern` format 0, `GPOS` lookup type 2) give the metrics; another table is ignored (`REF-OT-KERN`, `REF-OT-GPOS`, `REF-OT-POST`, `REF-OT-OS2`).
- Given a character missing from the font, when its advance is asked, then it is the advance of glyph 0, `.notdef` (`REF-OT-CMAP`); given a glyph past `numberOfHMetrics`, then it is the advance of the last record (`REF-OT-HMTX`).
- Given Inter 4.1's files, when a script runs, then their metrics are written to a data file of the repository with Inter's copyright notice and the OFL beside it, read by the tests without the font files (`REF-INTER-LICENSE`, `docs/tooling/dependencies.md`). Reading it in the playground comes with US-031, which first draws a text there (deferred at the EN-031 review, 2026-10-14).

## Tasks

One task = one commit, referenced as `EN-031.Tn`.

- [x] T1 — The font file reader: directory, head, hhea, maxp, hmtx, cmap; tests first with hand-computed values (3 h)
- [x] T2 — Kerning pairs from `kern` and `GPOS`; underline and strike-out from `post` and `OS/2`, and `fsType` (2 h)
- [x] T3 — The default font's metrics as a data file, and the script that writes it (1 h)

## Evidence

- Criterion 2 (Arial) is proven where the machine has macOS's `Arial.ttf`: the test runs in the local checks and the `pre-push` hook on the developer's Mac, and skips elsewhere, the font not being redistributable (`docs/tooling/dependencies.md`).
- Every other criterion is proven by fonts built byte by byte in the tests, and by the default font's data.
