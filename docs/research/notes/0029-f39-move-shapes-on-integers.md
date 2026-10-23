# 0029 — Sources of F39: moving shapes by the mouse and the arrow keys, landing on integers

Spike SP-028, 2026-10-23, for F39's stories. Every page cited was read through the local library the same day; the code F39 touches was read at F38's head (`a1586d6a`, F39's branch opened on it).

## 1. What F39 needs

| Story | What it needs | Where it comes from |
| --- | --- | --- |
| EN-145, the playground's tests fast on a large rounded shape | why `[F02.AC5] keeps 100 000 and smaller values exactly` takes 6.5 s under the pre-push's load | §6 |
| US-072, a drag moves the shapes selected | what a press on a shape selects; how far the pointer goes before a press is a drag; which point of a shape lands on integers, one shape and several; Shift during the drag | §2, §3, §4, §5 |
| US-073, the arrow keys nudge the shapes selected | the step, with Shift; a shape whose anchor holds a fraction; a key held | §2, §4 |

The rules F39 starts from: a move lands its anchor on integers, the top-left of its box by default, the shape's own fractions kept (Product Owner, 2026-10-22, `interaction.md` §2); the arrows nudge by 1, by 10 with Shift (Q50, `interaction.md` §1); X and Y, for one shape or several, are the top-left of the straight box (Q55, Q57, ADR-0035).

## 2. How each editor moves a selection

Ctrl stands for Cmd on macOS. "—" is what the pages read do not state.

| Behavior | Figma | Inkscape | Sketch | Penpot | Affinity Designer |
| --- | --- | --- | --- | --- | --- |
| a press on a shape not selected, dragged | — | selects it, then moves the selection | moves it | it becomes the selection alone, then moves | — |
| a press on one of several selected, dragged | — | moves the selection | moves them, from anywhere inside their box, the empty space between them included | moves the selection | — |
| Shift at the press, on a shape | — | a rubberband "even if you start from an object" | — | the shape toggled in the selection, then moved | — |
| how far before a drag starts | — | — | — | 10 screen pixels (`10 / zoom`) | — |
| one axis kept while dragging | — | `Ctrl` | `⇧` | Shift, the axis of the larger displacement | a modifier, unreadable |
| snapping off for a while | `Control` (snapping to objects) | `Shift` | `⌃` | — | — |
| nudge | 1, `Shift` 10, "resolution-independent points" | 2 px in the drawing's units, `Shift` 10 times; `Alt` 1 screen pixel | 1, `⇧` 10, set in Settings | 1, `Shift` 10, set in the profile | a nudge and a "modified nudge", set in Settings |
| whole units after a move | Snap to pixel grid, an option, always for frames; the point aligned not stated | — | a command, "Round layer to pixel" | Snap to pixel grid: each shape's upright box's top-left rounded, one shape at a time; a nudge never rounded | — |

Sources: `REF-INKSCAPE-KEYS`, `REF-INKSCAPE-TUTORIAL-BASIC`, `REF-SKETCH-MOVING-LAYERS`, `REF-SKETCH-SHORTCUTS`, `REF-SKETCH-CANVAS-SETTINGS`, `REF-PENPOT-TRANSFORMS`, `REF-PENPOT-PIXEL-PRECISION`, `REF-PENPOT-VIEWPORT-ACTIONS` (Penpot's code, level 5, read only, never copied), `REF-FIGMA-ADJUST`, `REF-FIGMA-PIXEL-GRID`, `REF-AFFINITY-SHORTCUTS`, `REF-AFFINITY-MOVE-TOOL`.

Not read: Figma's moves by the mouse, which none of its help pages read describes (a press on a shape not selected, Shift while dragging), and its keyboard shortcuts panel, built in the application; Affinity Designer's page on moving objects, which a search did not find, and its modifier keys, images the library does not keep; Illustrator, Adobe's help answering 403 to the library, as at SP-026 and SP-027. None is cited.

What the sources settle, two of them agreeing (`docs/research/README.md` §2):

- **A press on a shape not selected, dragged, selects it alone and moves it**: Inkscape ("Dragging an object selects it if it was not selected, then moves selection") and Penpot's `start-move-selected` (the shape pressed becomes the selection alone, `d/toggle-selection` without Shift); Sketch's "click and drag it" says the same of one layer. Taken as a rule for US-072, the agent's reading.
- **A press on one of several selected moves them all**: Inkscape ("then moves selection"), Sketch ("click and drag anywhere inside the outer selection box") and Penpot (`on-move-selected`, the selection as it is). Taken as a rule for US-072, the agent's reading.
- **The nudge is 1 and 10 with Shift**, already settled (Q50); Sketch and Penpot agree with Figma, and the three make it a setting, as `interaction.md` §1 makes every key (Keys editable, E20).

What they leave open, the questions of SP-028:

- **Shift at the press on a shape** (question 5): Inkscape draws a rubberband from it, Penpot toggles the shape and moves; F37 draws nothing today.
- **A press on an empty place inside the dashed box of several** (question 6): Sketch moves them; Inkscape and Penpot draw a rectangle from an empty place, as F37 does.
- **One axis kept while dragging** (question 7): Sketch and Penpot by Shift, Inkscape by Ctrl, Affinity by a modifier; Figma's key not read. Shift already adds at a press (F37), and Ctrl turns snapping off for a while with E19 (Q50), as Figma's.

## 3. How far the pointer goes before a press is a drag

A system keeps a click from becoming a drag by a threshold: Windows' `SM_CXDRAG` and `SM_CYDRAG`, "the number of pixels on either side of a mouse-down point that the mouse pointer can move before a drag operation begins. This allows the user to click and release the mouse button easily without unintentionally starting a drag operation" (`REF-WIN32-GETSYSTEMMETRICS`, its default value not on the page); GTK's `gtk-dnd-drag-threshold`, 8 pixels by default (`REF-GTK-DND-DRAG-THRESHOLD`); Penpot starts a move 10 screen pixels from the press (`REF-PENPOT-TRANSFORMS`).

The project's threshold is the press's integer point (F37, `interaction.md` §2, A drag is never a click), about 1 screen pixel at scale 1, set for a tool's drag and the selection rectangle, where a release on it is a click. A move is not alike: its landing turns a jitter into a move of a whole unit. A pressed at (50, 40) and released at (50.7, 40.2), the pointer having trembled by 0.73: the release's integer point (51, 40) differs from the press's, so it is a drag, and A's left at 10 goes to 10.7, landing at 11. With a threshold of a few screen pixels the same gesture is a click and A stays where it is. Question 4.

## 4. Which point lands on integers

The rule of 2026-10-22 lands "the top-left of its box". Two boxes exist (ADR-0035): the **straight box**, the smallest upright box around the contour as drawn, which X and Y show for one shape and several (Q55, Q57) and which the integer grid of `interaction.md` §4 snaps ("for a turned shape, the x and y of its straight box, its stored position staying fractional"); and the **stored position**, the top-left of the box before it turns. They differ on a turned shape, and on a straight shape whose contour does not reach its box's corners (a regular polygon not stretched). Penpot rounds the top-left of each shape's upright bounds (`REF-PENPOT-PIXEL-PRECISION`); Figma's X and Y of a rotated layer are its turned corner (`REF-FIGMA-ADJUST`), the point its pixel grid aligns not stated. Question 1.

Several shapes can land each by its own box, as Penpot does, one shape at a time over its tree of changes, or together by the box around them all, their gaps then kept exactly but only the box's top-left on integers. Question 2.

The arithmetic is the one a click already uses (`nearestInteger`, a tie rounded up, as CSS rounds, Q45): the shift a shape moves by is `nearestInteger(s + d) − s` on each axis, `s` where its anchor starts and `d` the pointer's displacement, the shape's position going by that shift (`straightBoxShift`, `shapeMovedOnAxis`, F38). A at (10, 20), dragged by (30.6, 19.4), goes to (41, 39); A turned by 30° about its middle, its straight box at (4.19873, −1.65064), dragged by (10.6, 3.2), has its straight box at (15, 2) and its position at (20.80127, 23.65064). No new derivation: one sum and one rounding per axis, checked by hand-computed tests.

A **nudge** is a move too: by 1 from an anchor on an integer it gives an integer whichever way is taken. From an anchor holding a fraction, which a shape turned after it was placed has (A's 4.19873), the ways split: Penpot moves by exactly 1, never rounding a nudge (`:ignore-snap-pixel true`); landing it as a drag lands gives either the next integer that way or the nearest integer to the anchor moved by 1. Question 3.

A **key held** sends the key again and again; Penpot gathers the repeats into one change for its history, ended 1 s after the last key or 250 ms after its release (`REF-PENPOT-TRANSFORMS`). The history comes with E02's next feature (Q7): F39 moves by one step per key sent, the browser's repeats included, and leaves to that feature whether a key held is one step of the history.

## 5. The code F39 touches

Read: `playground/selection/`, `playground/selection-drag/`, `playground/hover/`, `playground/kept-shapes/`, `playground/position/`, `playground/mount/update-page.ts`, `src/model/panel-values/`, `src/model/placement/straight-box-shift.ts`, `src/model/selection/`.

What it shows:

- **A press on a shape is already told apart.** `startSelectionDrag` keeps every press with the Selector in `SELECTION_DRAGS`; one on an empty place in Shape mode draws a rectangle (`isRectangle`), one on a shape is "left for moving it", kept so that its release is no click (`stopSelectionDrag`, `PRESSED_TOOLS`). US-072 gives that press its move: the drag's kind beside `isRectangle`, its shapes at the press kept with it.
- **One shape is edited from the fields, several are kept as JSON.** With one selected, the drawing is split around it and its position is the page's (`writeShapePosition`, a free shape's stored JSON, a text's drawn record); with several, every shape is kept below and the selection by indices (`writeShapeSelection`). The simplest move works on the drawing as kept (`readKeptDrawing`, the shape edited as last drawn), each shape selected moved by the model, then the selection written again as a click writes it (`writeShapeSelection`, `loadPickedShape`), so that one shape is loaded into the panel at its new place and several stay kept. `applySeveralValue` does this for X and Y typed for several (F38): the move reuses `shapeMovedOnAxis` and `straightBoxShift`, and the straight box of each (`shapePanelValues`, `textPanelValues`, a text by its measured block, `readTextBlock`).
- **A text never measured** is left as it is by a value typed for several (F38), having no straight box; a move does the same.
- **The keys are heard in one place.** `listenToSelectionKeys` maps Ctrl + A, Escape, Delete and Backspace outside the panel's fields (`FIELDS`, the anchor grid's radios included, which keep their arrows); the arrows go beside them, the page's own scrolling prevented only when a shape is selected.
- **A redraw is the whole page** (`updatePage`): the drawing, the panel and the layer above it. While a shape is dragged, every pointer move redraws; the cost of the band about a large rounded shape (§6) makes that redraw slow.
- **Folder sizes** (15 modules a signal, tests and barrel not counted, `docs/conventions/files.md`): `src/model/selection/` holds 12, `src/model/placement/` 9, `src/model/panel-values/` 12, `playground/selection/` 15, `playground/selection-drag/` 6. The move's model functions go in a folder of their own, `src/model/moving/`; its playground in `playground/selection-drag/`, beside the rectangle's drag; the arrows in a module of `playground/selection-drag/` too, `playground/selection/` being full.

## 6. Why `[F02.AC5] keeps 100 000 and smaller values exactly` is slow

The test types 100 000 in Height and 99 999 in Width on the rectangle that the test before it left with a radius of 100 000 (`[F02.AC5] caps a width or a radius above 100 000…`, which runs in 559 ms). The rectangle is selected, so each redraw outlines it by its band (`createOutlineElement`), and the band follows a curve by chords of at most 2 screen pixels (`contourBand`, `arcInnerSamples`, Q48): each of its four corners, a quarter circle of radius 49 999.5 (the radius reduced to fit), 78 539 long, is cut into about 39 270 chords, about 157 000 points along the contour, at each of the two values typed.

Measured on 2026-10-23, the test alone (`npx vitest run playground/mount/mount-playground.test.ts -t "100 000"`): 1 063 ms; with the radius typed 0 first, 11 ms; under the pre-push's load, 6.5 s three times against Vitest's 5 s.

Two ways out, which can be taken together:

- **The test**: type a radius of 0 before the sizes, since the criterion is that a size of 100 000 is kept exactly, which the radius does not change; a comment gives the timings. EN-145's T1.
- **The band**: chords chosen by how far they stray from the curve rather than by their length. A chord of length `c` on a circle of radius `r` strays from it by `r − √(r² − c²/4)`; a deviation `s` of a tenth of a screen pixel allows `c = 2√(2rs − s²)`, about 200 at radius 49 999.5, so about 393 chords a corner instead of 39 270, while a corner of radius 10 gets chords of about 2.8, against 2 today. This changes the rule of the outline (Q48) and `contourBand`, a `geometry` function, with a derivation by the `math-expert`. It matters for F39: a drag redraws at each pointer move. Question 8.

## 7. What remains open

The questions for the Product Owner are listed in full in SP-028, each with options and an example: the anchor (1), several shapes landing (2), a nudge from a fraction (3), the threshold of a move (4), Shift at the press (5), a press inside the dashed box (6), Shift during the drag (7), the band of a large rounded shape (8).

Left for later, not asked now: whether a key held is one step of the history, for E02's next feature (Q7); duplicating by Alt + drag (Figma, Sketch, Penpot), already for later (`interaction.md` §1); moving the points selected by the arrows in Points mode (Inkscape's node tool), for later with the points (`interaction.md` §3, For later).
