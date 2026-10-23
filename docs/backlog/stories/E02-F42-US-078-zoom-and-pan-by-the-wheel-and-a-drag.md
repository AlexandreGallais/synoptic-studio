---
id: US-078
epic: E02
feature: F42
title: Zoom and pan by the wheel, a pinch and a drag
status: ready
points: 3
---

# E02 · F42 · US-078 — Zoom and pan by the wheel, a pinch and a drag

As a symbol designer, I want to zoom about the pointer and move the view by the wheel, a pinch and a drag, so that I place a shape exactly and see the whole symbol without any shape's numbers changing.

After EN-148, whose view it keeps for the page. Both SVGs, the drawing and the layer above it, take the view's `viewBox` (ADR-0098); the pointer's point goes through the view; the export keeps the opening view's frame. The gestures as SP-030's answer 2 says; as proposed, Figma's, Inkscape's and Penpot's ([note 0031](../../research/notes/0031-f42-zoom-and-pan.md), §3): `Ctrl` (`Cmd` on macOS) with the wheel, and a pinch, zoom about the pointer; the wheel alone pans up and down, with `Shift` left and right; `Space` held with a drag, or a drag of the middle button, pans. SP-030's answers, given on 2026-10-23, each the option proposed: the `viewBox` of both SVGs (1), Figma's gestures (2), the wheel's zoom by its deltas, × (1 + 0.0057 a pixel) (4), the bounds 1 % to 25 600 % (5), the page opening at 100 % from (−40, −40) and Reset back to it (6), a click while the pointer stays within 3 pixels of its press at any zoom, the Selector's press likewise (8). The view changes only by the `viewBox` of both SVGs, never by the shapes' paths or coordinates.

## Acceptance criteria

The examples use a canvas of 1 200 × 800 CSS pixels, the opening view at 100 %, origin (−40, −40); A, a rectangle 100 × 50 at (10, 20); the focus outside the panel's fields.

- Given the opening view and the pointer at (440, 340) on the canvas, over (400, 300), when the wheel turns with `Ctrl` held, or `Cmd`, its `deltaY` −100 pixels, then the view zooms in about the pointer by × (1 + 0.0057 · 100) = 1.57, 157 % (SP-030's answer 4), its origin (400 − 440 / 1.57, 300 − 340 / 1.57) = (119.74522, 83.43949), the point under the pointer still (400, 300), the page itself not zoomed, the wheel's default prevented (`REF-MDN-WHEEL-EVENT`); a pinch, a `wheel` with `ctrlKey`, zooms the same; a `deltaMode` of lines counts 40 pixels a line, −2.5 lines zooming the same, of pages 800 (`REF-PENPOT-NORMALIZE-WHEEL`); the deltas sent before a frame zoom once by their sum, two of −50 giving 157 % (`REF-PENPOT-VIEWPORT-ACTIONS`); from 200 %, a `deltaY` of 100 zooms out to 2 / 1.57 = 127.389 %.
- Given the view at 200 %, origin (180, 130), when the wheel turns alone by a `deltaY` of 100 pixels, then the origin is (180, 180), 50 units down; with `Shift`, (230, 130), the vertical delta taken across, as is the horizontal one macOS sends instead (agent's choice).
- Given the view at 200 %, origin (180, 130), when the pointer is dragged by (100, 50) with `Space` held, or with the middle button, then the origin is (130, 105), nothing selected nor moved, nothing placed; `Space` held outside the panel's fields has its default prevented, so that neither the page scrolls nor a button takes it; typed in a field of the panel, it is the field's, and a drag then pans nothing.
- Given any change of the view, then the drawing's `<svg>` and the layer above have the same `viewBox`, A's outline on A; A's fields still read X 10, Y 20, W 100, H 50; no step is taken, Ctrl + Z undoing the last change of the drawing, the view staying.
- Given the view at 200 %, origin (180, 130), when the Rectangle tool clicks at (440, 340) on the canvas, over (400, 300), then the rectangle placed is 100 × 100 at (350, 250); at (240, 140), over (300, 200), at (250, 150). A press released while the pointer stayed within 3 pixels of it is a click at any zoom, the shape centred on the press (SP-030's answer 8): at 10 %, from (−40, −40), a press over (4 360, 3 360) and a tremble of 1 pixel, 10 units, place 100 × 100 at (4 310, 3 310); at 800 %, from (0, 0), a drag of 7 pixels across from (10, 10) draws 1 × 0 at (10, 10), one of 3 pixels places 100 × 100 at (−40, −40). The Selector's press likewise: at 100 %, A pressed and moved 3 pixels stays at (10, 20), selected, and moved 4 pixels lands at (14, 20).
- Given the view at 200 %, then a vertex's handle is drawn of radius 8 / 2 = 4 units, 8 pixels on screen, its halo 6.5, a frame's line 0.5, its dashes 2 in 3.5, a curve's handle square 4 units: every help in screen pixels (ADR-0098, decision 5); the bands of the layer above 2 pixels wide, their chords at most 2 pixels.
- Given A drawn, when it is exported at 100 % and at 200 %, then the two files are the same, their `viewBox` `-40 -40 1200 800` (ADR-0098, decision 4).
- Given A pressed at (50, 40) and moved to (60, 40), at (20, 20), when the wheel zooms to 157 % about the pointer meanwhile, then A stays under the pointer, at (20, 20), and its place is read through the new view at each move of the pointer: 15.7 pixels right, 10 units, A lands at (30, 20) (agent's choice).
- Given the pointer still over A's (50, 40), A outlined, when the wheel pans the view 100 units down, then (50, 140) is under the pointer and nothing is outlined; panned back, A is outlined again.
- Given the view changed, when Reset or a step of the guided test is clicked, then the view is the opening view, 100 % from (−40, −40), `-40 -40 1200 800` (SP-030's answer 6).

## Product Owner test

`npm run dev`, `http://localhost:5173/`. Written with T3, once SP-030's answers are given: zoom in and out with `Ctrl` and the wheel, and with a pinch, the point under the pointer staying; pan with the wheel, `Shift` and the wheel, `Space` and a drag; a shape placed at the zoom; the export at two zooms compared.

| Step | Do | You should see |
| --- | --- | --- |

## Tasks

- [x] T1 — The criteria in line with SP-030's answers; the page's view kept on the canvas, the wheel's and the drag's listeners (`playground/view/`), `createViewSvg` from the view, `readDrawingPoint` and `dispatchDrawingPointer` through it, the export's frame: interface and failing tests (1 h)
- [ ] T2 — Both SVGs from the view, the pointer through it, the export's fixed `viewBox`; the wheel with `Ctrl` and a pinch, the wheel alone and with `Shift`, `Space` and the middle button; one change of the view per frame (`scheduleRedraw`); the tests of T1 on them passing (2 h)
- [ ] T3 — `VIEW_SCALE` replaced by the view's scale; the frames, dashed boxes and handles of the drawing's SVG divided by it; Reset; the rules in `interaction.md` §1 and §7, the Q table, the Product Owner test card and F42's guide page; every test of T1 passing (2 h)

Files: `playground/view/` (new, the view of the page and its listeners), `playground/kept-shapes/` (`createViewSvg`, `VIEW_SCALE`), `playground/placing/read-drawing-point.ts`, `playground/user-actions/dispatch-drawing-pointer.ts`, `playground/document/read-exported-svg.ts`, `playground/points/create-handle-elements.ts`, `playground/handles/create-curve-handle-elements.ts`, `playground/drawing/`, `playground/hover/`, `playground/mount/`. `@see` ADR-0098, `docs/domain/interaction.md#_7-navigation-and-history`, `REF-MDN-WHEEL-EVENT`, `REF-MDN-WHEEL-DELTAMODE`, `REF-PENPOT-VIEWPORT-ACTIONS`, `REF-PENPOT-NORMALIZE-WHEEL`, `REF-FIGJAM-PAN-ZOOM`, `REF-SKETCH-SHORTCUTS`, `REF-INKSCAPE-KEYS`.

One task = one commit, referenced as `US-078.T1` in the commit footer.
