---
id: SP-032
epic: E02
feature: F44
title: Refine the corrections from the second try
status: done
points: 1
---

# E02 · F44 · SP-032 — Refine the corrections from the second try

A short spike that opens F44: the code each correction touches read, the bug reproduced, F44's stories written, the questions asked only where the code and the Product Owner's words leave a choice open.

## Acceptance criteria

- Given the three corrections below, when the spike ends, then each is traced to the playground's code it touches, the bug of correction 3 is reproduced with its steps and its cause, and F44's stories are written, 2 to 4 of at most 3 tasks each, criteria with numeric examples.
- Given what the code and the Product Owner's words leave open, when the spike ends, then the questions left are listed at once, each with options and a numeric example; none when the Product Owner's words settle it.

## The corrections asked (Product Owner, 2026-10-24)

1. **The helps as one dashed line, white and black**: the outline of the shape under the pointer, of each shape selected, the dashed box of several shapes and the selection rectangle are each one line of white and black dashes alternating, seen on every color, white and black included; US-080's accent over a white band is unseen on a white shape. In the layer above the drawing the line may be drawn by a native dashed stroke (`stroke-dasharray`), since the helps are not shapes: the shapes and texts never take a stroke, their edges drawn as paths (ADR-0002). That exception is a structural decision: an ADR, and the band computed about a contour (`contourBand`, EN-149) replaced for the helps by the shape's own path stroked, where the code allows (the two layers' row of the improvement log).
2. **The panel's fields the same for one shape as for several**: Shift with an arrow steps by 10 in one shape's X, Y, W and H as in several shapes' fields (US-081), today 1; the fields of one shape behave as the fields of several (a text field acting as a number field, its arrows, its bursts, its relative values), so that one rule holds everywhere.
3. **The view after a resize**: after the window is resized, or the pipeline panel shown or hidden, the hover is drawn offset from its shape, the drawing seems to shrink, and a tool's click sometimes places nothing (seen twice; once also a click on the work area after a text doing nothing): the view's canvas size, its `viewBox` and the pointer's point should follow the work area's size (`playground/view/read-canvas-size.ts`, `listenToView`, ADR-0098, decision on a resize).

Placed elsewhere, not in F44: the box of several shapes' position shown (E18); a group resized proportionally (E13, once groups exist), several shapes keeping each its own size as now.

## Questions

- What does the view read the canvas's size from, when is it read again, and what changes the canvas's size without a window's `resize`?
- How does a page hear an element's size change, and how do the playground's tests stand in for it?
- Where do one shape's X, Y, W and H differ from several shapes' fields, in the code?
- What does ADR-0002 cover, which rules carry it over to the helps, and what goes once the helps are strokes?
- What does a dashed line 1 or 2 pixels wide look like on a contour on whole units?

The code read, the measures and the sources: [note 0033](../../research/notes/0033-f44-corrections-from-the-second-try.md). **Correction 3 reproduced** in Chromium (§2): Pipeline hides or shows its column without a redraw (`listenToToolbar`), and the checklist is shown after the page's first draw (`playground/index.ts`), so that the canvas changes size by 300 or 360 pixels while both SVGs keep their `viewBox`, which `xMidYMid meet` then fits (`REF-SVG2-COORDS`): the drawing shrunk to 0.816 with the checklist, the rectangle 98 × 65 at (33, 124), or centred 150 pixels right with the column hidden, while the layer above, drawn again at each move of the pointer at the new size, puts the hover 150 pixels from its shape; a click on the shape as seen reads a point outside it. The window's own `resize` redraws both, measured. Cure tried in place: a `ResizeObserver` on the canvas (`REF-RESIZE-OBSERVER-1`). On the way, `#stage`, `overflow: hidden`, was scrolled 266 pixels by a focus outside it; `overflow: clip` stops it. **Correction 2**: one shape's fields are number fields applied at each key, several's text fields applied on Enter, the same values and computations behind (§4). **Correction 1**: [ADR-0099](../../adr/0099-a-native-stroke-for-the-helps-of-the-layer-above.md), proposed, a native stroke for the helps of the layer above, never for shapes; measured, a black line 1 pixel wide on a contour on whole units is two grey columns of 128, 2 pixels wide two black ones (§6). The stories: US-084 (correction 3), US-085 (correction 2) and US-086 (correction 1), all ready, US-086 once ADR-0099 is accepted.

## The agent's choices

The Product Owner left the details to the agent on 2026-10-24, no question asked, to be tuned once seen. The examples use the opening view, 100 %, and A and B, rectangles 100 × 100 at (0, 0) and (150, 0).

1. **The dashed line's width and dashes** (US-086): 2 screen pixels for the outlines of shapes, the hover's, the selection's and those a selection rectangle takes; 1 pixel for the dashed box of several shapes and the selection rectangle, laid half a pixel inside their sides; dashes 4 black and 4 white everywhere, Inkscape's length (`REF-INKSCAPE-CANVAS-ITEM-RECT`). Reason: measured in Chromium, a line 1 pixel wide on A's side, on the edge between two screen pixels, is two columns of grey 128, a line 2 pixels wide two black ones (note 0033, §6); a box's sides can be put on half pixels, a shape's contour not; 2 and 1 pixels are today's widths of the outline and of the box's dashes. The box of A and B, from (0, 0) to (250, 100), carries 31 dashes and a quarter along its top side. Left aside: 1 pixel everywhere, grey on whole units; 2 pixels and dashes of 6 everywhere, the box then as heavy as the outlines.
2. **Which of one shape's fields follow several's** (US-085): its X, Y, Width and Height, and a text's box Width and Height, made text fields acting as number fields, applied on Enter or when the field is left as several's: Shift + Up in the rectangle 120 × 80's Width gives 130, `+10` gives 130, typing 150 draws nothing until Enter, where today it draws 1, 15 then 150. Reason: these are the fields the correction names and the only ones several shapes have, so that one rule covers the fields that exist for both; a relative value cannot be applied as it is typed (`+1` before `+10`). The other number fields, Corners, Radius, Angle, a text's Size and spacings, a point's X and Y, keep the browser's step of 1: a Shift step of 10 for them all would make a fourth story, beyond the correction. Left aside: one set of fields shown for one shape and several, a change of the panel's layout the correction does not ask.

## Tasks

- [x] T1 — The code read, the bug reproduced, F44's stories drafted, the choices left to the agent made (1 h)

One task = one commit, referenced as `SP-032.T1`.

## For the main thread

**F44's feature file and the stories' index** were written in `SP-032.T1`, as SP-031 wrote F43's: `tests/backlog.test.ts` refuses a story file missing from its feature's table and from the index, and the pre-push refuses shared files on `US` and `EN` branches only. F44 now holds its three criteria, its three stories and its plan; to check against the rows written meanwhile before the `done` commit.

**The three stories are ready**, the details the Product Owner left to the agent chosen under The agent's choices (2026-10-24); US-084 first, US-086 once ADR-0099 is accepted.

**ADR-0099 is Proposed**: the Product Owner asked for it in words on 2026-10-24; its acceptance, recorded in its status and in `docs/adr/README.md`, comes before US-086, whose T1 then changes `.claude/rules/library.md` (frames and dashes are fills), `AGENTS.md`'s line on strokes (Geometry) and the `project-rules` digest that follows it, to say that the layer above's helps may stroke.

**`src/geometry/contour-band/`** is called by the playground alone today; once US-086 strokes the helps, nothing calls it. The library's call, not the Product Owner's: (a) kept, exported and tested, for E04's strokes; (b) taken out in US-086's T3 with its tests and `DERIV-band-within-box`, a change of the library's exports; (c) kept until E04 decides how a shape's stroke is offset.

**The reproductions** used the Playwright 1.62.1 of the machine's npx cache and its headless Chromium 151, driven by scripts of the spike's scratch folder against `vite` on port 5292, nothing committed, no package added, as SP-031 did; happy-dom's `ResizeObserver` does nothing, so US-084's tests use a stand-in, its behavior in a browser measured here only.

**The toolbar**, 1 632 pixels wide in the headless browser's fonts, overflows the stage below a window of about 2 270 pixels with both columns shown; US-084 stops the stage from scrolling, the responsive toolbar being E20's.

**No domain rule changed** by the spike: for the footer, `Rule-docs: none affected, a spike writing stories, a note, two references and a proposed ADR`.

## The main thread's decisions

Made on 2026-10-24, the Product Owner having said there was nothing to ask and that the details are the agent's, to be tuned later:

- ADR-0099 accepted, the Product Owner having asked for a native dashed stroke in the layer above in their own words; US-086's T1 brings `AGENTS.md`'s Geometry line, `.claude/rules/library.md` and the `project-rules` digest in line with it.
- `src/geometry/contour-band/` and `DERIV-band-within-box` kept, exported, when US-086 leaves them without a caller in the playground: a shape's stroke computed by offset (ADR-0002) is E04's, which will decide whether the band serves it; the option (c) of the spike.
