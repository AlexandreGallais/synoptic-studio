# 0028 — Sources of F37: selecting several shapes, and the usual keys of graphic editors

Spike SP-027, 2026-10-22, for F37's stories and for the default keys of E02, E18 and E19 (`interaction.md` §1, The default keys). Every page cited was read through the local library the same day; the code F37 touches was read at F36's head (`ecd9a1b3`).

## 1. What F37 needs

| Story | What it needs | Where it comes from |
| --- | --- | --- |
| US-068, several shapes by Shift + click | the keys adding and removing a shape, selecting all and none; how several selected shapes show | §2, §5; `selectionAfterHandleClick` reused (§6) |
| US-066, Delete | the keys deleting; what a key does while a field has the focus | §2; F08's `listenToDeleteKey` |
| US-069, the selection rectangle | where it starts; whether it takes what it touches or what it encloses; whether by the painted area or the box | §3, questions 1 and 2 of SP-027 |
| US-070, the panel for several shapes | what x, y, width and height show for several, Mixed, what a value typed does | §4, questions 4 to 6 of SP-027 |

## 2. The keys each editor gives

Ctrl stands for Cmd on macOS. "—" is a key the pages read do not state.

| Action | Figma | Inkscape | Penpot | Sketch | Affinity Designer | macOS, Windows |
| --- | --- | --- | --- | --- | --- | --- |
| add or remove a shape | Shift + click | Shift + click | Shift + click | Shift + click | a modifier + click, unreadable | — |
| select by a rectangle | a drag from an empty place | a drag around the objects | a drag around the layers | a drag from an empty place, X over a layer | a drag, two modifiers | — |
| select all | Ctrl + A | Ctrl + A (current layer) | Ctrl + A | Ctrl + A in the layer list | unreadable | Ctrl + A |
| select none | Escape, a click on the empty canvas | Escape | Escape (Cancel) | — | unreadable | — |
| delete | — | Delete, Backspace | Delete; Backspace on macOS | — | — | — |
| nudge | arrows, 1 | arrows, 2 px | arrows | — | arrows, customisable | — |
| nudge further | Shift + arrows, 10 | Shift + arrows, 10 nudges | Shift + arrows, "fast" | — | a modifier + arrows | — |
| undo | Ctrl + Z | Ctrl + Z | Ctrl + Z | — | unreadable | Ctrl + Z |
| redo | — | Ctrl + Y | Ctrl + Shift + Z | — | unreadable | Shift + Cmd + Z (macOS), Ctrl + Y (Windows) |

Sources: `REF-FIGMA-SELECT-LAYERS`, `REF-FIGMA-ADJUST`, `REF-INKSCAPE-KEYS`, `REF-INKSCAPE-TUTORIAL-BASIC`, `REF-INKSCAPE-SELECTOR`, `REF-PENPOT-SHORTCUTS`, `REF-PENPOT-LAYERS`, `REF-SKETCH-SELECTING`, `REF-AFFINITY-SHORTCUTS` (its keys are images the library does not keep), `REF-APPLE-SHORTCUTS`, `REF-WINDOWS-SHORTCUTS`.

Not read: Illustrator's and Photoshop's keys, Adobe's help answering 403 to the library (`helpx.adobe.com`, the shortcuts pages and the page on selecting inside the marquee), as at SP-026; Sketch's shortcuts page, built in the browser and empty in the library; Figma's keys for deleting and redoing, which its help pages read do not state. None is cited.

The choice, written in `interaction.md` §1 (Q50): Figma's keys where its pages state them, the Product Owner's rule (2026-10-22); where they do not, the keys two other sources agree on — Delete or Backspace (Inkscape, Penpot, and the project's own for points since F08), Ctrl + Shift + Z (Penpot, macOS) and Ctrl + Y (Inkscape, Windows) for redo, both kept since each is a system's habit. Ctrl + click adds or removes too, the Product Owner's rule of 2026-10-11, while Figma's Ctrl + click picks a layer inside a group: no group exists before E13, so the two do not meet yet (question 7). For E18 and E19, Figma's Shift keeps the proportions while resizing and turns by steps of 15°, its Ctrl turns snapping off for a while; Inkscape uses Ctrl for the first two and Shift for the third (`REF-FIGMA-ADJUST`, `REF-INKSCAPE-KEYS`).

## 3. What a selection rectangle takes

The editors split:

- **What it touches**: Figma, by its forum (`REF-FIGMA-FORUM-MARQUEE`, level 6: a drag "will select whatever touches the bounding box", and users ask for Sketch's way of taking only what is enclosed), its help saying only "Drag the cursor across any objects" (`REF-FIGMA-SELECT-LAYERS`); Sketch by default, `⌥` taking only "layers and frames that are completely within your selection area" (`REF-SKETCH-SELECTING`).
- **What it encloses**: Inkscape by default, "By default, selecting with a box will only select those objects that were completely enclosed by it", a button switching to what it touches, and `Alt` drawing a path that selects what it touches (`REF-INKSCAPE-SELECTOR`, `REF-INKSCAPE-KEYS`).
- Affinity Designer offers both, each with a modifier, its default not readable (`REF-AFFINITY-SHORTCUTS`); Penpot's "drag around the layers" does not say (`REF-PENPOT-LAYERS`).

Every editor read starts the rectangle from an empty place of the drawing (Figma, Sketch, Inkscape), a press on a shape being kept for moving it, which E02's next feature brings. Shift + drag adds to the selection in Inkscape and Sketch; Figma's pages do not say.

**What is tested** is a second choice the sources do not settle: a shape's painted area, as a click picks it (`interaction.md` §2, Under the pointer), or its box. Enclosing gives the same answer both ways: a contour lies in an upright rectangle exactly when its tight box does (`contourBoundingBox`), the rectangle being convex. Touching does not: a circle of diameter 100 in the box from (0, 0) to (100, 100) and a rectangle from (90, 90) to (120, 120) overlap by their boxes, but the circle's nearest point to (90, 90), its center (50, 50) at 56.6 from it, lies 6.6 beyond its radius of 50, so its painted area is not touched. Touching by the painted area needs the contour's segments, arcs and cubic sides cut against the rectangle's sides, which no derivation of the project states yet: an EN of its own, with the `math-expert`, before the rectangle's story.

## 4. The panel for several shapes

Two ways come out:

- **Each shape's values, Mixed where they differ** (Figma, Penpot): Figma's fields show `Mixed` for "two or more selected objects with different values", and "the change will apply to all selected layers" (`REF-FIGMA-ADJUST`); Penpot shows the placeholder Mixed and sends a value typed with the ids of every shape selected (`REF-PENPOT-MEASURES`, level 5, read only). The project's radius field already shows Mixed for vertices of different radii (`REF-FIGMA-CR`, F03).
- **The selection's box** (Inkscape, Affinity Designer): the transformation arrows sit "around the entire selection" (`REF-INKSCAPE-SELECTOR`), and Affinity transforms "the bounding box" of several objects unless Transform Objects Separately is chosen (`REF-AFFINITY-MOVE-TOOL`); a width typed then scales every shape.

Example, A at (10, 20), 100 × 50, and B at (200, 20), 40 × 40, both straight:

|  | Each shape's values (Figma) | The selection's box (Inkscape) |
| --- | --- | --- |
| shown | X Mixed, Y 20, W Mixed, H Mixed | X 10, Y 20, W 230, H 50 |
| X typed 50 | A at (50, 20), B at (50, 20): both start at 50 | A at (50, 20), B at (240, 20): both move by 40 |
| W typed 460 | A and B each 460 wide | both scaled by 2: A 200 × 100 at (10, 20), B 80 × 80 at (390, 20) |

The second way scales every shape, which gives fractions for most factors (W 300 scales by 300 / 230 = 1.304…), against the integers of ADR-0003, and is resizing, which E18 holds. Which way, and what is typed in F37 rather than with E02's next feature (moving, "x and y typed for several shapes") and E18 (resizing), are questions 4 and 5; a text without a box has no width to type (question 6).

## 5. How several selected shapes show

Figma marks each object — "A blue box will appear around each object" in the selection (`REF-FIGMA-SELECT-LAYERS`); Inkscape frames each one, "Each individual object within a selection displays a selection cue — by default, a dashed rectangular frame" (`REF-INKSCAPE-TUTORIAL-BASIC`), and puts its arrows around the whole selection (`REF-INKSCAPE-SELECTOR`). The project outlines the shape selected by the hover's band (`interaction.md` §2, The selection shown, Q49); whether several get a band each, and a box around them all, is question 3.

## 6. The code F37 touches, and the size of each story

Read: `playground/selection/` (the click, the panel loaded per kind), `playground/kept-shapes/` (the drawing kept on the canvas as `below`, `edited` and `above`), `playground/hover/` (the layer above the drawing, the outlines), `playground/mount/update-page.ts`, `playground/placing/start-new-shape.ts`, `playground/points/listen-to-delete-key.ts`, `playground/position/`, the panel of `playground/index.html`, `src/model/drawing/`, `src/model/selection/`.

What it shows:

- **One shape is edited from the panel's fields.** The shape the panel edits is not held as a value: each redraw reads it from the fields (`readShape`, `readTextModel`) and writes it back to `edited` (`showShape`, `showText`); the others are kept as JSON on the canvas. Several shapes selected cannot all be edited from the fields: with two or more, every shape must be kept, none edited, and the selection held beside them — a new key on the canvas holding their indices, and a kind `several` beside `none` (`writeEditedKind`, `updatePage`). Back to one, the shape is loaded into the panel as a click loads it (`splitDrawing`, `loadPickedShape`); going from one to two, the shape edited is kept with its settings saved as the last of its kind, as `addDrawingBelow` already does when a shape is placed.
- **The selection's rule exists.** `selectionAfterHandleClick` (`src/model/selection/`) is exactly the rule of `interaction.md` §2: Shift or Ctrl adds or removes, a plain click selects alone. US-068 reuses it on the shapes' indices rather than writing it again (its names say vertex; a renaming is the naming pass's).
- **Delete is already heard.** `listenToDeleteKey` deletes points on Delete or Backspace, outside an `input`; it does not leave a `textarea` (the text's Content), which US-066 must, and with no point selected it deletes nothing, so the shape's removal comes beside it, in Shape mode only.
- **Escape is already heard** by the color picker, which closes on it (`listenToPickerClosing`): Escape selects none only when no picker is open.
- **The outlines are one per shape.** `createSelectionElements` maps the `edited` shapes to their bands; with the selection's indices it maps them instead, unchanged otherwise.
- **The boxes exist.** A shape's straight box is `contourBoundingBox(curveContour(shapeCorners(shape)))`, a text's `textStraightBox`; a selection rectangle by boxes, and the panel's values for several shapes, need no new geometry. Common values reuse the logic of `commonRadius`.
- **A drag with the Selector is new.** A press with the Selector only picks today (`listenToPicking`); a drag from an empty place needs its own listener, a release on the press's integer point being a click as for a tool (`interaction.md` §1).

Each story's tasks, from that reading:

| Story | Tasks | Files |
| --- | --- | --- |
| US-068 | T1 interface and failing tests; T2 several shapes kept and outlined, back to one loaded; T3 Escape and Ctrl + A, the Product Owner test | `playground/selection/`, `playground/hover/create-selection-elements.ts`, `playground/kept-shapes/drawing-keys.ts`, `playground/mount/update-page.ts`, `playground/placing/start-new-shape.ts`, `playground/page/` |
| US-066 | T1 `drawingWithoutShapes` and failing tests; T2 the key and the Product Owner test | `src/model/drawing/`, `playground/selection/`, `playground/points/listen-to-delete-key.ts` |
| US-069 | T1 the shapes a rectangle takes, by their boxes, and failing tests; T2 the drag from an empty place, the rectangle drawn in the layer above; T3 Shift + drag, the Product Owner test | `src/model/drawing/`, `playground/selection/`, `playground/hover/` |
| US-070 | T1 the values of several shapes, common or Mixed, and failing tests; T2 the panel shows them; T3 the Product Owner test | `src/model/drawing/`, `playground/selection/`, `playground/index.html` |

Four stories fit F37 when the rectangle tests the boxes and the panel only shows several shapes' values. Touching by the painted area adds an EN (§3), and typing values for several shapes a story or two (moving each, resizing each with a turned shape's corner kept, a text's box): either outgrows four, and the panel then goes to a feature of its own (question 8).

## 7. What remains open

The questions for the Product Owner are listed in full in SP-027, each with its options and an example: what the rectangle takes (1), by what (2), how several shapes show (3), the panel's way (4) and what is typed in F37 (5), a text among several (6), Ctrl + click (7), and the cut of the feature (8).

Left for later, not asked now: a press on a shape already selected among several, which E02's next feature settles when a press may start a move (Figma and Inkscape keep the selection to drag it); Ctrl + A in Points mode, which Inkscape's node tool uses to select every node, left as it is (it selects no shape there).
