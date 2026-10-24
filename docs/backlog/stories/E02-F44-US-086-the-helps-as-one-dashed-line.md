---
id: US-086
epic: E02
feature: F44
title: The helps as one dashed line of white and black
status: ready
points: 3
---

# E02 · F44 · US-086 — The helps as one dashed line of white and black

As a symbol designer, I want the outlines of the shape under the pointer and of the selection, the dashed box of several shapes and the selection rectangle drawn as one line of white and black dashes, so that I see them on every color, white and black included.

Correction 1 of the Product Owner's second try (2026-10-24): US-080's accent over a white band cannot be seen on a white shape. Each help becomes the path of the contour or box it outlines, stroked white, then stroked black and dashed over it, so that its dashes alternate black and white, as Inkscape draws a control's dashes again in a second color in the first's gaps (`REF-INKSCAPE-CANVAS-ITEM-RECT`, `REF-INKSCAPE-SELCUE`); in the layer above alone, by a native stroke, which [ADR-0099](../../adr/0099-a-native-stroke-for-the-helps-of-the-layer-above.md), proposed by SP-032, allows for the helps and never for shapes (ADR-0002). The bands computed about each contour (`contourBand`, `contourBandWithin`) and the frame they are reused in (`playground/helps-frame/`, EN-149) go ([note 0033](../../research/notes/0033-f44-corrections-from-the-second-try.md), §5). Worked once ADR-0099 is accepted; the widths and dashes are the agent's choice (SP-032, The agent's choices, 1), left to the agent by the Product Owner on 2026-10-24, to tune once seen: 2 pixels on a shape's contour, sharp on whole units where 1 pixel is grey, 1 pixel on the box and the rectangle laid on half pixels, dashes 4 and 4 as Inkscape's.

## Acceptance criteria

The examples use the opening view, 100 %, and A and B, rectangles 100 × 100 at (0, 0) and (150, 0), drawn in the page's blue `#4a7bd0`, and two texts `Pump` at (0, 150) and (150, 150), one white `#ffffff`, one black `#000000` (the shapes take a color of their own with E04, the texts today, `interaction.md` §5).

- Given A under the pointer with the Selector, then its outline is two `<path>` of A's contour as drawn (`contourPiecesToPathData`), `fill: none`, in the layer above: a white stroke 2 screen pixels wide, then a black one of the same width dashed `4 4`, its dashes from A's top-left corner clockwise, black from 0 to 4 along the top side, white from 4 to 8; on A's blue, over the white text's letters and over the black text's alike, each text outlined on its frame. A selected, and each shape a selection rectangle would take, the same.
- Given the view at 200 %, then the strokes are 1 unit wide and dashed `2 2`, 2 and 4 pixels on screen; at 25 %, 8 units and `16 16` (`screenToUser`, ADR-0098, decision 5).
- Given A and B selected, then their dashed box, from (0, 0) to (250, 100), is one line 1 screen pixel wide, white then black dashed `4 4`, on the rectangle from (0.5, 0.5) to (249.5, 99.5), half a pixel inside the box, so that it lies on whole pixels (note 0033, §6); each shape keeps its own outline.
- Given a selection rectangle dragged from (−10, −10) to (200, 110), then its line is 1 screen pixel wide, white then black dashed `4 4`, on the rectangle from (−9.5, −9.5) to (199.5, 109.5), its tint over its area unchanged.
- Given any shape or text of the drawing, then none has a `stroke` attribute nor style, its edges still paths (ADR-0002); the file exported is the same as before, the layer above never in it.
- Given R, 99 999 × 100 000 of radius 100 000, selected at 100 %, then its outline is the two strokes of its contour's path, whatever part of it the view shows, no band computed (`contourBand` no longer called by the playground).

## Product Owner test

`npm run dev`, `http://localhost:5173/`. The page opens with a blue rectangle 120 × 80 at (0, 0), selected.

| Step | Do | You should see |
| --- | --- | --- |
| 1 | Look at the rectangle the page opens with | its outline a line 2 pixels wide of black and white dashes, 4 pixels each, seen on its blue |
| 2 | Choose the Text tool, click below the rectangle, type `Pump`, type `#ffffff` in the text's Color and press Enter; click an empty place, then point at the text's letters | the same dashed line around the text's frame, seen over the white letters |
| 3 | Do the same with `#000000` beside it | the dashed line seen over the black letters |
| 4 | Choose the Rectangle tool and click an empty place to add a second rectangle, then Shift + click the first | each rectangle outlined by the dashed line, and a thinner dashed box, 1 pixel wide, just around both |
| 5 | Press an empty place and drag across both rectangles, without releasing | the selection rectangle a thin dashed line over its light tint, both rectangles outlined by the dashed line |
| 6 | Release; open the zoom menu (`100 %`), type `400` in Zoom and press Enter, then `100` | the lines, the dashes and the box the same size on screen at each zoom |
| 7 | Click Export SVG and open the file | the shapes and the texts alone, without any outline, box or dash |

## Tasks

- [ ] T1 — The helps as strokes in the layer above, the widths and dashes as chosen: `createOutlineElements`, the dashed box of `createSelectionElements`, `createSelectionRectangleElements`, their classes and the page's style; `interaction.md` §2 (The outline, Seen on any color, The selection shown, Shown for several shapes and for the rectangle) and its Q row, `.claude/rules/library.md` and `AGENTS.md` on strokes, as ADR-0099 accepted says; tests of the criteria first, failing (2 h)
- [ ] T2 — The code that passes them, the strokes' width and dashes divided by the view's scale (2 h)
- [ ] T3 — The bands and their frame taken out of the playground (`playground/helps-frame/`, `readBandPathData`, `startHelpsFrame`, `HELPS_FRAMES`), what the drawing's SVG still uses kept (`readBoxBandPathData` for a text's frame), `interaction.md` §2 (Computed where it is seen) superseded, EN-149's tests brought to the strokes; the Product Owner test (1 h)

Files: `playground/helps-frame/`, `playground/hover/` (`create-selection-elements.ts`, `create-selection-rectangle-elements.ts`, `create-hover-elements.ts`, `create-taken-elements.ts`, `show-hover.ts`, `helps-on-any-color.test.ts`), `playground/index.html` (`#overlay` styles), `docs/domain/interaction.md` §2, `.claude/rules/library.md`, `AGENTS.md`. `@see` `REF-SVG2-PAINT`, `REF-INKSCAPE-CANVAS-ITEM-RECT`, `REF-INKSCAPE-SELCUE`, `docs/adr/0099-a-native-stroke-for-the-helps-of-the-layer-above.md`, `docs/domain/interaction.md#_2-selector`.

One task = one commit, referenced as `US-086.T1` in the commit footer.
