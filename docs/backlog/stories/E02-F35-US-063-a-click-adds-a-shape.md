---
id: US-063
epic: E02
feature: F35
title: A click with a tool adds a shape where it is clicked
status: parked
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

## Question for the Product Owner

Asked on 2026-10-22, after T2 (the drawing as a list of shapes, the box centred on a click, in the library, pushed on `feat/us-063-a-click-adds-a-shape`). T3, the playground's tools placing shapes as Figma does, makes 234 of the playground's tests fail in 31 files: since E01 a tool button turns the edited shape into another kind and keeps its values (120 × 80 at (0, 0)), and the E15 checklist lines say "choose the Text tool" with no click on the drawing; US-032's memory of X, Y and angle per tool becomes dead code. The work no longer fits US-063's three tasks.

- **A**: Figma's way in the one playground page: the tests and checklists of E01 to E15 migrated (choose the tool, click the drawing, type the values), the tool memory removed; F35 split into two features, the first the migration and the click, the second the drag, the hover and click selection, and Delete.
- **B**: a tool button still turns the edited shape into its kind, and also arms the placing: few tests change, but choosing Polygon with a rectangle selected turns it into a polygon, against Figma's way.
- **D**: a new playground page for the drawing of several shapes (`?drawing`), Figma's way, the current page kept as it is for the checklists of E01 to E15; the new page reuses the panel, no old test migrated.
