---
id: EN-032
epic: E15
feature: F05
title: "A text in the model: its characters, its lines and its measured block"
status: ready
points: 3
---

# E15 · F05 · EN-032 — A text in the model: its characters, its lines and its measured block

A text holds its content and the options of its characters; the library transforms its case, measures its lines and gives its block, from the font's metrics.

## Acceptance criteria

- Given "Hello World" in Arial at size 16, when it is measured, then its width is 82.6875 without kerning (10 584 × 16 / 2048) and 82.39844 with it (`DERIV-text-width`, steps 1 and 2).
- Given a letter spacing of 2 and a word spacing of 4, when "Hello World" is measured, then 2 is added between adjacent characters and 4 for each space: 106.6875; a single character gets no letter spacing (`DERIV-text-width`, step 3; `REF-CSS-TEXT-3`).
- Given the content "Hello" and "World" on two lines, when it is measured, then its block is 41.78125 wide, its longest line, and 36.79688 tall, two line heights of `(A − D + G) × s / U` (`DERIV-text-width`, step 4).
- Given each case transform (upper, lower, sentence, title, camel, Pascal, kebab, snake, constant), when "pump P-101 is running" is transformed, then it gives the string of `docs/domain/shapes.md` (Text), once Q36 is answered.
- Given an empty content, when it is measured, then its block is 0 wide; given a size outside 1 to 100 000 or not an integer, then the text is refused.

## Tasks

One task = one commit, referenced as `EN-032.Tn`.

- [ ] T1 — The text type, its validity and its case transforms; tests first (2 h)
- [ ] T2 — Measuring a line with kerning and spacing, and the block of its lines (2 h)
