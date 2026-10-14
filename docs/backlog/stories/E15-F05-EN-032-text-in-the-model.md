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

- Given "Hello World" in Arial at size 16 without kerning, when it is measured, then its width is 82.6875 (10 584 × 16 / 2048).
- Given a letter spacing of 2 and a word spacing of 4, when "Hello World" is measured, then 2 is added after each character but the last and 4 after each space.
- Given a content with two lines, when it is measured, then its block is as wide as its longest line and as tall as two line heights, the line height being the font's ascender minus descender plus line gap, times the size over the units per em.
- Given each case transform (upper, lower, sentence, title, camel, Pascal, kebab, snake, constant), when "pump P-101 is running" is transformed, then it gives the expected string.
- Given an empty content, when it is measured, then its block is 0 wide; given a size outside 1 to 100 000 or not an integer, then the text is refused.

## Tasks

One task = one commit, referenced as `EN-032.Tn`.

- [ ] T1 — The text type, its validity and its case transforms; tests first (2 h)
- [ ] T2 — Measuring a line with kerning and spacing, and the block of its lines (2 h)
