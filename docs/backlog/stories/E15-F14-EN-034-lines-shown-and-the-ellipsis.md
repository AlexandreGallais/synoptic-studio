---
id: EN-034
epic: E15
feature: F14
title: The lines a box shows, and the ellipsis
status: ready
points: 2
---

# E15 · F14 · EN-034 — The lines a box shows, and the ellipsis

The library keeps the lines that fit in the box's height and its maximum number of lines, and ends a text cut, or a word too long moved whole, with an ellipsis (`DERIV-text-box`, steps 5 and 6).

## Acceptance criteria

Arial 5.01 at size 16, kerning left out; automatic line height 18.39844; "…" 2048 units, 16 wide.

- Given "Hello World" in a box 60 wide with at most 1 line, when its lines are shown, then the line is "Hello…", 52.46094 wide (`REF-CSS-OVERFLOW-4`, `line-clamp`, `block-ellipsis`).
- Given "Hello World" in a box 50 wide with at most 1 line, when shown, then the line is "Hell…", 43.5625 wide: "Hello…" (52.46094) does not fit, so the last character goes (`REF-CSS-OVERFLOW-3`, `text-overflow: ellipsis`).
- Given "Hello World Hello World" in a box 60 wide with at most 2 lines, when shown, then the lines are "Hello" and "World…", 57.78125 wide.
- Given the same text in a box 60 × 40 with no maximum, when shown, then 2 lines show, "Hello" and "World…": 2 lines take 36.79688 ≤ 40, 3 take 55.19531.
- Given a box 10 tall, when shown, then one line shows, the box being shorter than a line (provisional, VAL-015).
- Given "Pump P-101 is running" in a box 50 wide with long words moved whole, when shown, then the last line is "runn…", 48.02344 wide ("runni…" is 51.57813); with at most 2 lines, the lines are "Pump" and "P-10…", 49.79688.
- Given "Hello World" in a box 10 wide with at most 1 line, with either long-word option, when shown, then the line is "H…", 27.55469 wide: the first character stays and the line overflows (`REF-CSS-OVERFLOW-3`).
- Given a face without a glyph for U+2026, when a line is cut, then it ends with three full stops "...", 13.33594 wide in Arial 16 (`REF-CSS-OVERFLOW-3`).
- Given a text whose lines all fit, when shown, then no ellipsis is added.

## Tasks

One task = one commit, referenced as `EN-034.Tn`.

- [ ] T1 — The number of lines shown from the box's height, the line height, the trim and the maximum, at least one; tests first, a property: the lines shown never exceed the maximum (2 h)
- [ ] T2 — The ellipsis: characters removed from the end until the line fits, the first kept, three full stops when the face lacks U+2026; `@see DERIV-text-box`, `REF-CSS-OVERFLOW-3`; tests first, a property: a cut line is no wider than the box unless one character is left (2 h)
