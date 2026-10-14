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

- Given the Text tool, when content, font, size, weight, italic, color, opacity, spacing, underline, strike-through, case and direction are typed, then the drawing shows them, and the SVG holds one `<text>` with one `<tspan>` per line, each with its `x` and `y`, without `text-anchor` nor `dominant-baseline` (ADR-0052, `REF-SVG2-TEXT`, `REF-RESVG-SUPPORT-TABLE`).
- Given a text aligned left, center or right, when it has two lines of different widths, then each line's `x` places it in the block as the alignment says, and each `y` is the line's top plus the ascender times the size over the units per em (`DERIV-text-width`, step 4).
- Given underline or strike-through, when drawn, then each is a `<path>` rectangle from the font's `post` underline or `OS/2` strike-out position and thickness, not `text-decoration` (ADR-0052, `REF-OT-POST`, `REF-OT-OS2`).
- Given the bold and italic buttons, when they are pressed, then the weight becomes 700 and the style italic; the weight list offers 100 to 900 (`REF-CSS-FONTS-4`, §2.2).

## Tasks

One task = one commit, referenced as `US-031.Tn`.

- [ ] T1 — The Text tool and its fields in the playground (3 h)
- [ ] T2 — The drawing of the text: positioned lines and their attributes (2 h)
- [ ] T3 — The Product Owner test card (30 min)

## Notes from EN-032's review (2026-10-15)

- **Spaces kept.** The library measures every space of a line, runs of spaces and spaces at its ends included; the SVG writer must keep them all, with `xml:space="preserve"` or a `white-space` that preserves them, or a reader collapses them and draws a narrower line than the measured block.
- **Letter spacing after the last character.** `DERIV-text-width` adds letter spacing between units only, `u − 1` times, as CSS Text 3 says; the CSS editors' draft now spreads it on both sides of each unit (`REF-CSSWG-10193`) and browsers differ. Measure in Chrome, Firefox and Safari whether a line with letter spacing ends with a trailing space, and record the result before the text's block is trusted for alignment.
