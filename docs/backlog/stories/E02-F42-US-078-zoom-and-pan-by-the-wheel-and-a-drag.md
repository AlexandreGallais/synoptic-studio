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

After EN-148, whose view it keeps for the page. Both SVGs, the drawing and the layer above it, take the view's `viewBox` (ADR-0098); the pointer's point goes through the view; the export keeps the opening view's frame. The gestures as SP-030's answer 2 says; as proposed, Figma's, Inkscape's and Penpot's ([note 0031](../../research/notes/0031-f42-zoom-and-pan.md), §3): `Ctrl` (`Cmd` on macOS) with the wheel, and a pinch, zoom about the pointer; the wheel alone pans up and down, with `Shift` left and right; `Space` held with a drag, or a drag of the middle button, pans. Draft until SP-030's answers 1, 2, 4, 5 and 8.

## Acceptance criteria

The examples use a canvas of 1 200 × 800 CSS pixels, the opening view at 100 %, origin (−40, −40); A, a rectangle 100 × 50 at (10, 20); the focus outside the panel's fields.

- Given the opening view and the pointer at (440, 340) on the canvas, over (400, 300), when the wheel turns with `Ctrl` held, its `deltaY` −100 pixels, then the view zooms in about the pointer as SP-030's answer 4 says (157 % by Penpot's deltas), the point under the pointer still (400, 300), the page itself not zoomed (`REF-MDN-WHEEL-EVENT`); a pinch, a `wheel` with `ctrlKey`, zooms the same; a `deltaMode` of lines counts 40 pixels a line, of pages 800 (`REF-PENPOT-NORMALIZE-WHEEL`).
- Given the view at 200 %, origin (180, 130), when the wheel turns alone by a `deltaY` of 100 pixels, then the origin is (180, 180), 50 units down; with `Shift`, (230, 130).
- Given the view at 200 %, origin (180, 130), when the pointer is dragged by (100, 50) with `Space` held, or with the middle button, then the origin is (130, 105), nothing selected nor moved; `Space` typed in a field of the panel is the field's.
- Given any change of the view, then the drawing's `<svg>` and the layer above have the same `viewBox`, A's outline on A; A's fields still read X 10, Y 20, W 100, H 50; no step is taken, Ctrl + Z undoing the last change of the drawing, the view staying.
- Given the view at 200 %, origin (180, 130), when the Rectangle tool clicks at (440, 340) on the canvas, over (400, 300), then the rectangle placed is 100 × 100 at (350, 250); a click at a low zoom as SP-030's answer 8 says.
- Given the view at 200 %, then a vertex's handle is drawn of radius 8 / 2 = 4 units, 8 pixels on screen, its halo 6.5, a frame's line 0.5, its dashes 2 in 3.5, a curve's handle square 4 units: every help in screen pixels (ADR-0098, decision 5); the bands of the layer above 2 pixels wide, their chords at most 2 pixels.
- Given A drawn, when it is exported at 100 % and at 200 %, then the two files are the same, their `viewBox` `-40 -40 1200 800` (ADR-0098, decision 4).
- Given shapes moved by a drag, when the wheel turns meanwhile, then the view changes and the shapes stay under the pointer, their place read through the view at each move (agent's choice).
- Given the view changed, then the shape outlined as under the pointer is the one under it in the new view.
- Given Reset, or a step of the guided test, then the view is the page's opening view, as SP-030's answer 6 says.

## Product Owner test

`npm run dev`, `http://localhost:5173/`. Written with T3, once SP-030's answers are given: zoom in and out with `Ctrl` and the wheel, and with a pinch, the point under the pointer staying; pan with the wheel, `Shift` and the wheel, `Space` and a drag; a shape placed at the zoom; the export at two zooms compared.

| Step | Do | You should see |
| --- | --- | --- |

## Tasks

- [ ] T1 — The criteria in line with SP-030's answers; the page's view kept on the canvas, the wheel's and the drag's listeners (`playground/view/`), `createViewSvg` from the view, `readDrawingPoint` and `dispatchDrawingPointer` through it, the export's frame: interface and failing tests (1 h)
- [ ] T2 — Both SVGs from the view, the pointer through it, the export's fixed `viewBox`; the wheel with `Ctrl` and a pinch, the wheel alone and with `Shift`, `Space` and the middle button; one change of the view per frame (`scheduleRedraw`); the tests of T1 on them passing (2 h)
- [ ] T3 — `VIEW_SCALE` replaced by the view's scale; the frames, dashed boxes and handles of the drawing's SVG divided by it; Reset; the rules in `interaction.md` §1 and §7, the Q table, the Product Owner test card and F42's guide page; every test of T1 passing (2 h)

Files: `playground/view/` (new, the view of the page and its listeners), `playground/kept-shapes/` (`createViewSvg`, `VIEW_SCALE`), `playground/placing/read-drawing-point.ts`, `playground/user-actions/dispatch-drawing-pointer.ts`, `playground/document/read-exported-svg.ts`, `playground/points/create-handle-elements.ts`, `playground/handles/create-curve-handle-elements.ts`, `playground/drawing/`, `playground/hover/`, `playground/mount/`. `@see` ADR-0098, `docs/domain/interaction.md#_7-navigation-and-history`, `REF-MDN-WHEEL-EVENT`, `REF-MDN-WHEEL-DELTAMODE`, `REF-PENPOT-VIEWPORT-ACTIONS`, `REF-PENPOT-NORMALIZE-WHEEL`, `REF-FIGJAM-PAN-ZOOM`, `REF-SKETCH-SHORTCUTS`, `REF-INKSCAPE-KEYS`.

One task = one commit, referenced as `US-078.T1` in the commit footer.
