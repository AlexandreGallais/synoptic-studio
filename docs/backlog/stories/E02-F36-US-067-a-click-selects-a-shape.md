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

| Step | Do | You should see |
| --- | --- | --- |

## Tasks

- [x] T1 — The drawing split around the edited shape, the selection: interface and failing tests (1 h)
- [ ] T2 — The click, the selection outline, none selected, the tests of T1 passing (2 h)
- [ ] T3 — The panel loaded per kind, a rectangle as a Rectangle, a text with its whole style; the Product Owner test card (2 h)

One task = one commit, referenced as `US-067.Tn` in the commit footer.
