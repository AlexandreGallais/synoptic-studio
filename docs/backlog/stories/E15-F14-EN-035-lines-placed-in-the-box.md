---
id: EN-035
epic: E15
feature: F14
title: "Lines placed in the box: line height, cap-height trim, vertical alignment"
status: ready
points: 3
---

# E15 · F14 · EN-035 — Lines placed in the box: line height, cap-height trim, vertical alignment

The library reads the font's cap height, and places each line's baseline from the line height, the trim and the vertical alignment in the box (`DERIV-text-box`, steps 7 and 8).

## Acceptance criteria

Arial 5.01 at size 16, kerning left out: ascender 1854, descender −434, line gap 67, `sCapHeight` 1467.

- Given Arial's file, when its metrics are read, then its cap height is 1467, read from `OS/2` version 3; given an `OS/2` table of version 0 or 1, or a cap height of 0, then it is 0.66 em, 1351.68 for 2048 units per em (`REF-OT-OS2`, `REF-CSS-INLINE-3`, A.2); Inter's metrics shipped with the library carry their cap height.
- Given "Hello" over "World" with an automatic line height, when placed, then the baselines are 14.48438 and 32.88281 below the block's top, as in F05 (F14.AC6).
- Given the same with a line height of 24, when placed, then the baselines are 17.54688 and 41.54688, half of the leading `24 − 17.875` above each line, and the block is 48 tall (`REF-CSS-INLINE-3`, §5.3).
- Given the same, trimmed, with an automatic line height, when placed, then the first baseline is 11.46094 below the block's top, the second 29.85938, and the block is 29.85938 tall; at size 32 the first baseline is 22.92188 below the top (`REF-CSS-INLINE-3`, §6.2).
- Given "Hello" over "World" in a box 60 tall, when aligned to the middle, then the block starts 11.60156 below the box's top and the first baseline is 26.08594; aligned to the bottom, 37.6875; trimmed and in the middle, 26.53125.
- Given lines in a box 60 wide, when aligned left, center or right, then each line's `x` follows `DERIV-text-width` step 5 with the box's width as the block's.
- Given a text in a box 60 × 40 at (0, 0) turned by 90°, when drawn, then it turns about (30, 20), the middle of its box (`DERIV-turned-shape`, step 4).

## Tasks

One task = one commit, referenced as `EN-035.Tn`.

- [ ] T1 — The cap height read from `OS/2` with its fallback, kept in the font metrics; Inter's data written again; tests first (2 h)
- [ ] T2 — The first baseline and the line height: automatic, chosen with half-leading, trimmed; `@see DERIV-text-box`; tests first, a property: consecutive baselines are one line height apart (2 h)
- [ ] T3 — The block placed in the box by its vertical alignment, the lines across the box's width, the turn about the box's middle; tests first (2 h)
