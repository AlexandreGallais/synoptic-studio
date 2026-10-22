---
id: US-063
epic: E02
feature: F35
title: A click with a tool adds a shape where it is clicked
status: ready
points: 3
---

# E02 · F35 · US-063 — A click with a tool adds a shape where it is clicked

As a symbol designer, I want a tool to place a new shape where I click, the shapes already drawn kept, so that a symbol is built of several shapes.

The drawing becomes an ordered list of shapes, the last drawn on top, flat until E13 makes it a tree; the panel edits the shape just added. The toolbar gets a selector, the tool the editor returns to once a shape is placed (agent's choice, as Figma).

## Acceptance criteria

- Given the rectangle tool chosen, when the drawing is clicked at (250, 140), then a rectangle 100 × 100 is added at x 200, y 90, drawn above the shapes already there, and selected; the tool returns to the selector.
- Given the polygon or round shape tool, when the drawing is clicked, then that shape is added the same way, in a box 100 × 100; given the text tool, a text fitting its content, centred on the point.
- Given a point clicked between two integers once the view's scale is applied, when the shape is placed, then its x and y are integers, rounded as the agent decides and writes in `interaction.md` §1.
- Given three shapes drawn, when the SVG is exported, then it holds the three, in the order drawn, each unchanged by the others.

## Product Owner test

`npm run dev`, `http://localhost:5173/`.

| Step | Do | You should see |
| --- | --- | --- |

## Tasks

- [ ] T1 — The drawing as a list of shapes and the placing of one by a click: interface and failing tests (1 h)
- [ ] T2 — The drawing, the placing and the export, the tests of T1 passing (2 h)
- [ ] T3 — The playground's tools place shapes on a click; the Product Owner test card (2 h)

One task = one commit, referenced as `US-063.Tn` in the commit footer.
