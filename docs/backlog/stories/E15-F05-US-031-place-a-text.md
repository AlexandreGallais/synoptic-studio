---
id: US-031
epic: E15
feature: F05
title: Place a text with the Text tool
status: ready
points: 3
---

# E15 · F05 · US-031 — Place a text with the Text tool

As a designer, I want a Text tool with its fields so that a symbol shows a label in the font, size, weight and color chosen.

## Acceptance criteria

- Given the Text tool, when content, font, size, weight, italic, color, opacity, spacing, underline, strike-through, case and direction are typed, then the drawing shows them, and the SVG holds one `<text>` with one positioned `<tspan>` per line.
- Given a text aligned left, center or right, when it has two lines of different widths, then each line is placed in the block as the alignment says.
- Given the bold and italic buttons, when they are pressed, then the weight becomes 700 and the style italic; the weight list offers 100 to 900.

## Tasks

One task = one commit, referenced as `US-031.Tn`.

- [ ] T1 — The Text tool and its fields in the playground (3 h)
- [ ] T2 — The drawing of the text: positioned lines and their attributes (2 h)
- [ ] T3 — The Product Owner test card (30 min)
