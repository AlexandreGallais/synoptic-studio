---
id: F14
epic: E15
title: A box of text
status: draft
---

# E15 · F14 — A box of text

**Benefit hypothesis**: the designer puts a long text in a box of given width and height, wrapped, cut with an ellipsis, aligned in both directions, as in Figma and a word processor.

Refined with F05 on 2026-10-14 (its rounds 1 and 2); its stories are written by its spike, once F05 is built.

## Acceptance criteria

1. A text either fits its content or sits in a box of a given width and height; in a box, its lines wrap at spaces, and a word too long for the box either breaks at a character or moves whole to the next line, as chosen.
2. A text too long for its box, or for a maximum number of lines, ends with an ellipsis.
3. The lines are aligned horizontally and vertically in the box (top, middle, bottom), with a line height chosen; a balanced wrap style evens the lines' lengths.
4. A background color is drawn behind the text, as a highlighter, as a `<path>`.
5. A vertical trim from the cap height to the baseline lines up texts of different sizes on where their letters start.
6. Every shape and text of E01, E14 and F05 draws as before.

## Stories

Written by its research spike.

## Left for later

- Scrolling a long text in its box on a touch screen: with the synoptic views (E08), through the HTML layer (F15).
