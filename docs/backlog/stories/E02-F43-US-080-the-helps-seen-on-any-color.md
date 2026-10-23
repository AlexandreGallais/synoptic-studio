---
id: US-080
epic: E02
feature: F43
title: The helps seen on any color, and what a selection rectangle takes
status: ready
points: 3
---

# E02 · F43 · US-080 — The helps seen on any color, and what a selection rectangle takes

As a symbol designer, I want the outlines of the shape under the pointer, of the selection and of the selection rectangle to show on any color, the shapes a rectangle will take to show while I drag it, and the drawing's texts never selected as page text, so that I always see what I point at and what I take.

Corrections 1, 2 and 6 of the Product Owner's first try (2026-10-23). Today every help is filled with `--accent`, `#4a7bd0`, the color a shape is drawn with when it has none of its own, so that the selection of the page's own rectangle cannot be seen ([note 0032](../../research/notes/0032-f43-corrections-from-the-first-try.md), §3). Inkscape draws its rubberband in blue over a white outline and its boxes' dashes in two colors (`REF-INKSCAPE-NODE-HANDLES`, `REF-INKSCAPE-CANVAS-ITEM-RECT`, `REF-INKSCAPE-SELCUE`); Figma outlines each object a marquee takes while it is dragged, Penpot selects them as it goes (`REF-FIGMA-SELECT-LAYERS`, `REF-PENPOT-AREA-SELECTION`). The colors, the widths and what the rectangle shows wait for SP-031's questions 2 and 3; the criteria below take the options proposed first.

## Acceptance criteria

The examples use the opening view, 100 %, and A, B and C, rectangles 100 × 100 at (0, 0), (150, 0) and (300, 0), drawn in the page's `#4a7bd0`.

- Given A under the pointer with the Selector, then its outline is two bands about its contour, drawn in the layer above: a light one 4 screen pixels wide, white, then the accent's 2 pixels over it, so that 1 pixel of white shows on each side of the blue, on A's own blue as on white (question 2); A selected, the same; at 200 %, the bands are 2 and 1 units wide, 4 and 2 pixels on screen (ADR-0098, decision 5).
- Given A and B selected, then their dashed box, from (0, 0) to (250, 100), shows its dashes 4 long every 7 in the accent, the 3 between them white, both 1 pixel thick (question 2, as Inkscape's box, `REF-INKSCAPE-SELCUE`).
- Given a selection rectangle dragged from (−10, −10) to (200, 110), then its band of 1 pixel in the accent lies over a white band of 3 pixels, its tint unchanged (question 2).
- Given that rectangle while it is dragged, its press at (−10, −10) and the pointer at (200, 110), then A and B, the shapes it touches (`shapesTouchedByBox`), are outlined as under the pointer, C not; with Alt held, A alone, the one it encloses; released, A and B selected as today; the pointer back to (100, 50), B is no longer outlined (question 3).
- Given Shift held during that drag with C selected, then C keeps its selection's outline and A and B are outlined as under the pointer.
- Given a text on the drawing, when it is pressed, dragged across or double-clicked with the Selector, then none of its characters is selected as page text, the shape still picked or moved (`user-select: none` on the canvas and the layer above, `REF-CSS-UI4-USER-SELECT`); the panel's fields still select their text.
- Given any of these helps, then none is in the export: the file of A, B and C is the same as before.

## Product Owner test

`npm run dev`, `http://localhost:5173/`. The page opens with a blue rectangle 120 × 80 at (0, 0), selected.

| Step | Do | You should see |
| --- | --- | --- |
| 1 | Look at the rectangle the page opens with | its outline shows on its own blue: a blue line with a thin white edge on each side |
| 2 | Click the empty drawing, then point at the rectangle | the same outline, the shape under the pointer |
| 3 | Add two rectangles with the Rectangle tool, then drag a selection rectangle from an empty place across two of them, without releasing | the two shapes it touches are outlined before the release; the rectangle's own line shows over the blue |
| 4 | Release | the two are selected, a dashed box around them, its dashes blue and white |
| 5 | Add a text, type `Pump`, then double-click on it and drag across it with the Selector | the text is selected as a shape, its letters never highlighted as page text |

## Tasks

- [ ] T1 — The two bands of each outline, the dashes of two colors and the rectangle's band over a light one, the colors and widths as answered, in `playground/helps-frame/`, `playground/hover/` and the page's style; `interaction.md` §2 (The outline, The selection shown, Shown) and its Q row; tests of the criteria first, failing (2 h)
- [ ] T2 — The shapes a selection rectangle takes outlined while it is dragged, Alt and Shift read at each move, in `playground/hover/show-hover.ts`; `interaction.md` §2 (The selection rectangle, Shown); tests first (2 h)
- [ ] T3 — `user-select: none` on the canvas and the layer above, a test reading the computed style and a double click on a text; the Product Owner test (1 h)

Files: `playground/index.html` (`#overlay` and `#canvas` styles), `playground/helps-frame/create-outline-element.ts`, `playground/hover/create-hover-elements.ts`, `create-selection-elements.ts`, `create-selection-rectangle-elements.ts`, `show-hover.ts`, `docs/domain/interaction.md` §2. `@see` `REF-INKSCAPE-NODE-HANDLES`, `REF-INKSCAPE-CANVAS-ITEM-RECT`, `REF-INKSCAPE-SELCUE`, `REF-FIGMA-SELECT-LAYERS`, `REF-PENPOT-AREA-SELECTION`, `REF-CSS-UI4-USER-SELECT`, `docs/domain/interaction.md#_2-selector`.

One task = one commit, referenced as `US-080.T1` in the commit footer.
