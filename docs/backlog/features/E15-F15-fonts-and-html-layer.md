---
id: F15
epic: E15
title: Fonts from files, embedded fonts and the HTML layer
status: in-progress
---

# E15 · F15 — Fonts from files, embedded fonts and the HTML layer

**Benefit hypothesis**: any font of the computer can be used and measured, a symbol or a view can carry its fonts, and browsers lay text out live.

Refined with F05 on 2026-10-14 (its rounds 1 and 2); its stories are written by its spike, once F05 is built.

## Acceptance criteria

1. A TrueType or OpenType font file imported, or a font installed on the computer (read by the browser with permission), is read into its metrics once and kept.
2. An option embeds a font in a symbol, and in a synoptic view once for all its symbols (`@font-face` in `<defs>`), its licence permitting it (OS/2 `fsType`).
3. An HTML layer in a `<foreignObject>` inside a `<switch>` lets a browser lay the text out live with its own fonts; other readers show the positioned lines (ADR-0053).
4. Measuring stays a sum of stored numbers: a thousand texts measured in a few milliseconds.

## Stories

In delivery order.

| ID                                                                       | Type  | Title                                                                 | Status      |
| ------------------------------------------------------------------------ | ----- | --------------------------------------------------------------------- | ----------- |
| [SP-018](../stories/E15-F15-SP-018-research-f15-fonts-and-html-layer.md) | Spike | Research for F15: fonts from files, embedded fonts and the HTML layer | in-progress |

## Left for later

- Arabic and Indian scripts measured by the library: they need a shaping engine; until then they are laid out by the browser's HTML layer.
