---
id: F14
epic: E15
title: A box of text
status: in-progress
---

# E15 · F14 — A box of text

**Benefit hypothesis**: the designer puts a long text in a box of given width and height, wrapped, cut with an ellipsis, aligned in both directions, as in Figma and a word processor.

Refined with F05 on 2026-10-14 (its rounds 1 and 2); its stories were written by SP-016 (research note 0018).

## Acceptance criteria

1. A text either fits its content or sits in a box of a given width and height; in a box, its lines wrap at spaces, and a word too long for the box either breaks at a character or moves whole to the next line, as chosen.
2. A text too long for its box, or for a maximum number of lines, ends with an ellipsis: "Hello World" in Arial 16, in a box 60 wide with at most 1 line, shows "Hello…", 52.46094 wide ("Hell…" in a box 50 wide); "Hello World Hello World" in a box 60 wide with at most 2 lines shows "Hello" over "World…", 57.78125 wide (`DERIV-text-box`).
3. The lines are aligned horizontally and vertically in the box (top, middle, bottom), with a line height chosen; a balanced wrap style evens the lines' lengths.
4. A background color is drawn behind the text, as a highlighter, as a `<path>`.
5. A vertical trim from the cap height to the baseline lines up texts of different sizes on where their letters start.
6. Every shape and text of E01, E14 and F05 draws as before.

## Stories

In delivery order.

| ID                                                                  | Type       | Title                                                                     | Status |
| ------------------------------------------------------------------- | ---------- | ------------------------------------------------------------------------- | ------ |
| [SP-016](../stories/E15-F14-SP-016-research-f14-text-box.md)        | Spike      | Research for F14: a box of text wrapped, cut with an ellipsis and aligned | done   |
| [EN-033](../stories/E15-F14-EN-033-lines-wrapped-in-a-box.md)       | Enabler    | A text in a box: its options and its lines wrapped at spaces              | done   |
| [EN-034](../stories/E15-F14-EN-034-lines-shown-and-the-ellipsis.md) | Enabler    | The lines a box shows, and the ellipsis                                   | done   |
| [EN-035](../stories/E15-F14-EN-035-lines-placed-in-the-box.md)      | Enabler    | Lines placed in the box: line height, cap-height trim, vertical alignment | done   |
| [EN-036](../stories/E15-F14-EN-036-balanced-wrap.md)                | Enabler    | A balanced wrap that evens the lines' lengths                             | done   |
| [CHK-015](../stories/E15-F14-CHK-015-checkpoint-f14.md)             | Checkpoint | Checkpoint in the middle of F14                                           | done   |
| [US-033](../stories/E15-F14-US-033-put-a-text-in-a-box.md)          | User story | Put a text in a box with the Text tool                                    | done   |
| [US-034](../stories/E15-F14-US-034-background-behind-a-text.md)     | User story | A background color behind a text, as a highlighter                        | done   |
| [AUD-016](../stories/E15-F14-AUD-016-audit-f14.md)                  | Audit      | Audit F14                                                                 | ready  |
| [VAL-015](../stories/E15-F14-VAL-015-validate-f14.md)               | Validation | Validate F14                                                              | ready  |

## Feature plan

| Acceptance criterion | Realized by            | Verified by         |
| -------------------- | ---------------------- | ------------------- |
| F14.AC1              | EN-033, US-033         | AUD-016             |
| F14.AC2              | EN-034, US-033         | AUD-016             |
| F14.AC3              | EN-035, EN-036, US-033 | AUD-016             |
| F14.AC4              | US-034                 | AUD-016             |
| F14.AC5              | EN-035, US-033         | AUD-016             |
| F14.AC6              | every story            | `npm run check:all` |

## Left for later

- Scrolling a long text in its box on a touch screen: with the synoptic views (E08), through the HTML layer (F15).
