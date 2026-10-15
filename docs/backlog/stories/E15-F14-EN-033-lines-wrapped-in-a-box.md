---
id: EN-033
epic: E15
feature: F14
title: "A text in a box: its options and its lines wrapped at spaces"
status: ready
points: 3
---

# E15 · F14 · EN-033 — A text in a box: its options and its lines wrapped at spaces

A text holds the options of its box, and the library wraps its lines at spaces in the box's width, a word too long broken at a character or moved whole (`DERIV-text-box`, steps 1 to 4; `docs/domain/shapes.md`, Text, Box).

## Acceptance criteria

Arial 5.01 at size 16, kerning left out, as in `DERIV-text-width` (one font unit is 1 / 128 user unit); every box is tall enough for all its lines.

- Given a text, when it is checked (`isValidText`), then it holds: whether it fits its content or sits in a box; the box's width and height, integers from 0 to 100 000; the long-word option, break or move; a maximum number of lines, an integer from 0 (none) to 100 000; a line height, an integer from 0 (automatic) to 100 000; a vertical alignment, top, middle or bottom; a wrap style, automatic or balanced; a trim, on or off; the ellipsis, on or off (on by default); a background on or off, its color in hexadecimal and its opacity from 0 to 100 %; a value outside these, or not an integer, is refused (provisional bounds, VAL-015).
- Given "Hello World" in a box 50 wide, when its lines are wrapped, then they are "Hello" (36.46094) and "World" (41.78125): 10 584 units, 82.6875, do not fit, and the break comes after the space (`REF-UAX14`, LB18; `REF-WIKI-LINE-WRAP`).
- Given "Hello", two spaces and "World" in a box 50 wide, when wrapped, then the break comes after the second space and the first line is "Hello", 36.46094, its spaces neither measured nor drawn (`REF-UAX14`, SP; `REF-CSS-TEXT-3`, §4.1.2).
- Given "Hello" and "World" joined by a no-break space U+00A0 in a box 50 wide, when wrapped, then they stay one word: no break at the no-break space (`REF-UAX14`, LB12).
- Given "Pump P-101 is running" in a box 50 wide with long words broken at a character, when wrapped, then the lines are "Pump", "P-101", "is", "runnin" (44.47656) and "g": "running" (53.375) starts its own line, then breaks after its longest run of characters that fits, without a hyphen (`REF-CSS-TEXT-3`, §5.4, `overflow-wrap: anywhere`).
- Given the same text with long words moved whole, when wrapped, then the lines are "Pump", "P-101", "is" and "running", the last 53.375 wide, wider than the box (`overflow-wrap: normal`; EN-034 cuts it when the ellipsis is on).
- Given "Hello World", a line break and "Hello" in a box 50 wide, when wrapped, then each paragraph wraps on its own: "Hello", "World", "Hello".
- Given a text that fits its content, when measured, then its lines and block are those of F05, unchanged (F14.AC6).

## Tasks

One task = one commit, referenced as `EN-033.Tn`.

- [x] T1 — The options of the box in the text type, their validity and their bounds; F05's texts given the defaults (fit content, line height automatic, no trim, no background, ellipsis on); tests first (2 h)
- [ ] T2 — The paragraphs cut into tokens at spaces and wrapped greedily, the spaces at a wrap dropped, `@see DERIV-text-box`; tests first, a property: no line but a long word's is wider than the box (3 h)
- [ ] T3 — A long word broken at a character or moved whole; tests first, a property: the lines joined give back the paragraph without its dropped spaces (2 h)
