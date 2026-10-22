---
id: US-067
epic: E02
feature: F36
title: A click selects a shape, the panel loads it
status: ready
points: 3
---

# E02 · F36 · US-067 — A click selects a shape, the panel loads it

As a symbol designer, I want to pick a shape by a click and edit it in the panel, so that any shape of the drawing can be changed again.

Cut from US-065 on 2026-10-22. A shape picked from the middle of the drawing keeps its place in the drawing order, so the page holds the shapes below and above the edited one. A picked rectangle shows as a Rectangle with its radius, and a picked text loads its whole style (Product Owner, 2026-10-22, US-065's answer).

## Acceptance criteria

- Given shapes A, B and C drawn in that order, C edited, when A is clicked with the Selector, then A is edited, shown selected in the layer above, and drawn under B and C; a shape placed next goes on top of all three.
- Given a rectangle of radius 10 placed, then a polygon placed, when the rectangle is clicked, then the panel shows Rectangle, its x, y, width, height and radius 10.
- Given a text in Roboto 700 italic, struck through, the current family Inter, when it is clicked, then the panel shows Roboto, 700, italic and the strike-through, and its content.
- Given a click on the empty drawing, then none is selected and the panel says so.
- Given the selection shown, when the SVG is exported, then it holds no selection outline.

## Product Owner test

`npm run dev`, `http://localhost:5173/`.

| Step | Do | You should see |
| --- | --- | --- |
| 1 | Open the page, then Reset | the rectangle 120 × 80 the page opens with, outlined in blue as selected; the Selector button pressed |
| 2 | Click Polygon and click the drawing so that the polygon covers the rectangle's right end; then Round shape and click further right | three shapes, the polygon over the rectangle; the round shape outlined as selected |
| 3 | Click the left part of the rectangle | the rectangle outlined as selected, the panel titled Rectangle, width 120, height 80, radius 10; the polygon still drawn over it |
| 4 | Type 200 in Width | the rectangle widens under the polygon, the polygon still on top, the outline following |
| 5 | Click Rectangle, then an empty place of the drawing | the new rectangle selected, drawn last, on top of all |
| 6 | Click an empty place of the drawing | no outline; the panel says no shape is selected and how to select one; every shape still drawn |
| 7 | Click Text and click the drawing; choose another family than Inter (Installed fonts, or Import a font), then Italic, weight 700, Strike-through, and type Pump in Content | the text Pump in that family, bold, italic, struck through |
| 8 | Click Text and click elsewhere; choose Inter, Italic off, weight 400, Strike-through off | a second text, in Inter regular |
| 9 | Click the text Pump | the panel shows its family, 700, Italic pressed, Strike-through ticked and Pump; the font list shows its family |
| 10 | Click Rectangle, press an empty place and drag until the pointer is over another shape, then release | the new rectangle selected, not the shape under the pointer at the release |
| 11 | Click a shape, then Export SVG and open the file | every shape, and no outline |

## Tasks

- [x] T1 — The drawing split around the edited shape, the selection: interface and failing tests (1 h)
- [x] T2 — The click, the selection outline, none selected, the tests of T1 passing (2 h)
- [x] T3 — The panel loaded per kind, a rectangle as a Rectangle, a text with its whole style; the Product Owner test card (2 h)

One task = one commit, referenced as `US-067.Tn` in the commit footer.
