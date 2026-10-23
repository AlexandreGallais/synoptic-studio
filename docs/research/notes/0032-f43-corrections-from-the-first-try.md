# 0032 — Sources of F43: the corrections from the Product Owner's first try, and the slow test of the history

Spike SP-031, 2026-10-23, for F43's stories. Every page cited was read through the local library the same day; the code was read at F43's branch, opened on `59ddb71c`. The measures were taken on the author's machine, Node 24.18, happy-dom 21.0.3, and a headless Chromium 151 driven by a Playwright found in the machine's cache, outside the repository, against the page served by `vite` (none of it committed: a test file and a script, deleted after).

## 1. What F43 needs

| Story | Correction | What it needs | Where it comes from |
| --- | --- | --- | --- |
| EN-150, the playground's tests fast again | the blocker | where the time of `playground/history/undo-and-redo.test.ts` goes, and a cure that does not raise its limit | §2 |
| US-080, the helps seen on any color, and what a selection rectangle takes | 1, 2, 6 | how editors draw their outlines so that they show on any color; whether they show, during the drag, what a rectangle will take; `user-select` | §3, §4, §8 |
| US-081, the arrows in the panel's fields and on the drawing | 3, 4, 5 | what an arrow does in a field showing Mixed; the events of a number field's arrow held; two arrows held | §5, §6, §7 |
| US-082, a text's box, the wheel with Shift, several shapes' sizes | 7, 9, 10 | Shift for a text's box; Shift with the wheel on macOS and elsewhere; correction 10 reproduced | §8, §9, §10 |
| US-083, the list of steps scrolled whole | 8 | a list whose rows are drawn as they come into sight; the tallest box a browser scrolls | §11 |

## 2. The slow test of the history (the blocker)

**Measured** (the file alone, `npx vitest run playground/history/undo-and-redo.test.ts`): 62 tests in 29.2 s, the test `[F40.AC2] takes a step at each key on the hue slider, the plane's drag and the color typed` 2.45 s, 0.45 s when run alone (`-t "hue slider"`). Its tests grow slower down the file: about 150 ms for the first, about 750 ms for the last ones. Loaded by `check:all`, 80 to 140 s for the file and over 5 s for that test, as reported.

**Where the time goes**, read from a CPU profile of the file inside its worker (`node:inspector`, sampled every 0.5 ms, 27.2 s): 21.4 s in `updatePage`, the page's redraw; inside it, 16.8 s in happy-dom's `QuerySelector.findFirst`, called by `selectElement` (`playground/page/select-element.ts`, `document.querySelector("#id")`). Two causes:

- **The listeners of every mount stay on the document.** `openServedPage` mounts the page again in each test's `beforeEach`; `loadPage` replaces the head and the body, so the listeners on elements go with them, but the 14 listeners each mount adds to `document` (`keydown`, `keyup`, `input`, `change`, `focusout`, `pointerdown`, in 9 modules) and those on `window` (`resize`, `blur`) stay, the document being the same through a file. After 48 mounts, 672 listeners are on the document; 7 undos by Ctrl + Z redraw the page 343 times, 49 per key, and take 1 863 ms, against 14 redraws and 104 ms after one mount. A test's keys cost more the further down the file it is. In the browser the page is mounted once, so this is the tests' cost alone. It is not the redraws per key of F42: 5 keys on the hue slider redraw 5 times, after 1 mount as after 48.
- **happy-dom's `querySelector` walks the tree after each change.** It keeps a cache of the selectors asked, which any change of the document drops (`affectsCache`), and a redraw changes the document; `getElementById` reads the document's map of ids instead (`ParentNodeUtility.getElementById`, `elementIdMap`), in constant time (happy-dom 21.0.3, read in `node_modules`). `selectElement` asks an id only, so `document.getElementById(id)` gives the same element.

**Measured cures**, each tried in place and taken back:

| Cure | The file | The hue slider's test | 4 other playground files |
| --- | --- | --- | --- |
| none | 29.2 s | 2 447 ms | 2.3, 2.6, 2.9 and 4.1 s |
| the document's listeners of the mount before removed at each load | 9.1 s | 356 ms | 1.8, 2.1, 2.8, 2.8 s |
| `selectElement` by `getElementById` | 9.1 s | 791 ms | 0.8, 1.2, 1.6, 1.7 s |
| both | 3.9 s | 116 ms | 0.8, 1.0, 1.2, 1.5 s |

The four other files: `view/zoom-and-pan`, `selection/write-several-values`, `installed-fonts/installed-fonts`, `selection/select-shapes`. A listener given an `AbortSignal` is removed "when the `abort()` method of the `AbortController` which owns the `AbortSignal` is called" (`REF-MDN-ADD-EVENT-LISTENER`): the page's mount keeps one controller per document and aborts the one of the mount before, so that mounting again leaves one set of listeners; the tests then need no change. The convention asks a merely slow test to be made faster (`docs/conventions/testing.md`, Isolation and time): the limit stays 5 s.

## 3. The helps seen on any color (correction 1)

**The cause.** The outlines, the dashed box and the selection rectangle's band are filled with `--accent`, `#4a7bd0`, the color a shape is drawn with when it has none of its own (`#overlay path.hover`, `#canvas path:not([fill])`, `playground/index.html`): on the page's own rectangle the outline of the selection is the shape's own color, and cannot be seen.

**How editors draw them:**

| Editor | The outline of a shape or a rectangle | Source |
| --- | --- | --- |
| Inkscape | the rubberband: a stroke of `#277fff` 2 px over an outline of white 1 px, `outline: white` at 75 % for every control; the controls (handles) black over white; a control rectangle draws its outline, then its stroke over it | `REF-INKSCAPE-NODE-HANDLES`, `REF-INKSCAPE-CANVAS-ITEM-RECT`, `REF-INKSCAPE-RUBBERBAND` |
| Inkscape | the box of each object selected (the `BBOX` cue): dashed, white dashes `0xffffffa0` alternating with dark blue ones `0x0000c0a0`, the second color drawn in the gaps of the first | `REF-INKSCAPE-SELCUE`, `REF-INKSCAPE-CANVAS-ITEM-RECT` |
| Penpot | one stroke of 1 px divided by the zoom, `--color-accent-tertiary`, a component's in its own color; no second color | `REF-PENPOT-OUTLINE` |
| Figma | "a blue box" around each object in a marquee, a solid purple box in view-only mode; a second color not stated | `REF-FIGMA-SELECT-LAYERS` |

Inkscape alone draws a second, light color under the first; Penpot and Figma draw one. What settles it is the Product Owner's ask, a double outline seen on every color: a light band under the accent's, as Inkscape's controls, needs no new kind of element (a band is a fill, ADR-0098 decision 5 keeps it in screen pixels), only a second band of the same contour, wider, drawn first: the cache of bands (`readBandPathData`, keyed by the contour and the band's size) keeps both. Question 2 asks the colors and widths; a band of 2 px of the accent over one of 4 px of white shows 1 px of white on each side.

## 4. What a selection rectangle takes, shown while it is dragged (correction 2)

- **Figma**: dragging the marquee across objects, "A blue box will appear around each object are in the selection", as the page writes it (`REF-FIGMA-SELECT-LAYERS`).
- **Penpot**: the area selects while it is dragged: its shapes are selected every 100 ms of the drag (`rx/buffer-time 100`), and once more at its last move, so that the selection follows the rectangle (`handle-area-selection`, `REF-PENPOT-AREA-SELECTION`).

Two sources agree that the shapes a rectangle takes show during the drag; they differ in how: Figma outlines them, Penpot selects them. The page already computes the shapes taken (`shapesTouchedByBox`, `shapesEnclosedByBox`, EN-144) and draws the rectangle at each move (`createSelectionRectangleElements`, `playground/hover/show-hover.ts`); the outlines of the shapes it would take are the hover's band (`createHoverElements`), so that they read as "under the pointer" (question 3).

## 5. An arrow in a field showing Mixed (correction 3)

- **Figma**: a field accepts an equation, "`Mixed+100` to add 100 to two or more selected objects with different values"; "When an equation is entered into the position, dimension or rotation fields, the change will apply to all selected layers" (`REF-FIGMA-ADJUST`, Equations). What an arrow does in such a field is not stated in the pages read.
- **Penpot**: its numeric field is a text field (`type "text"`) whose Up and Down add a step, 10 with Shift, 0.1 with Alt, the value applied at each key; a field of several different values (`:multiple`) has no value, so that an arrow starts from the field's minimum, or 0, and sets every shape to it (`REF-PENPOT-NUMERIC-INPUT`).

So Penpot steps a Mixed field, but to one value for all; the Product Owner asks each shape's own value to move by the step, as `+1` typed does (F38, `parsePanelValue`, `panelValueAfterEdit`): A 100 wide and B 40, Up in W gives 101 and 41, Shift + Up 110 and 50. The fields stay text fields, which keep a typed `+50` (`interaction.md` §2, The panel for several shapes, Shown); the arrow is read on the field (`keydown`, `ArrowUp`, `ArrowDown`), its default prevented so that the caret does not move. A step below 0 in W or H refuses the value for all, as `-1` typed does (US-071).

## 6. An arrow held in a number field (correction 4)

**Measured** in Chromium 151: Up pressed in W, then repeated 4 times 40 ms apart, as a key held repeats, fires `keydown input change` five times, the repeats marked `repeat`, then one `keyup`; W goes from 120 to 125 and the history takes 5 steps, the list showing Start and 5 rows. The same keys sent within one frame take one step, the step being taken at the redraw that follows (`schedulePanelStep`): a test sending its keys at once never sees it. MDN says a `change` fires "When the user commits the change explicitly" (`REF-MDN-CHANGE-EVENT`); Chromium counts each step of the arrow as one. The page asks a step at every `change` in the panel (`listenToPanelSteps`, `PANEL_ENDS`), hence one per repeat.

SP-029's answer 3 makes a key held one step from its press to its release on the drawing (`interaction.md` §7, A key held); the same in a field means: a `change` that follows an arrow key still held takes no step, and the arrow's `keyup` takes it, as the color picker's sliders take theirs at their `keyup` already (`PANEL_ENDS`, `[role="slider"]`). Question 5 asks whether the step waits for the release or for a pause.

## 7. Two arrows held at once (correction 5)

A key held repeats: its `keydown` comes again, marked `repeat`, as the measure of §6 shows on each `keydown` after the first. With Up held, then Right pressed, the repeats that follow are Right's alone: the page nudges by each key it receives (`listenToNudgeKeys`, `nudgeShift`), so A goes up once, then right. That the systems repeat only the last key pressed was not read in a source; it is what the Product Owner saw (Up and Left do not move a shape diagonally) and what the story's first task checks by hand in a browser. The cure the page can take alone: keep the arrows held (`keydown` adds, `keyup` takes out, the window's `blur` empties, as `Space` held, AUD-044), and nudge at each arrow's `keydown`, first or repeated, by the sum of the arrows held: Up and Right held, (+1, −1) at each key, Left and Right cancel out. No editor's page read states a diagonal nudge; it is the Product Owner's ask (question 6).

## 8. A text not selected as page text, and a text's box square (corrections 6 and 7)

- **`user-select: none`** on the canvas and the layer above: "The UA must not allow selections to be started in this element", and "Attempting to start a selection in an element where user-select is none, such as by clicking in it or starting a drag in it, must not cause a pre-existing selection to become unselected"; the value is not inherited but an element whose parent's used value is `none` takes `none` too, so one rule on `#canvas` covers its SVG and its texts (`REF-CSS-UI4-USER-SELECT`, §6.1). A double click on a text then selects no characters. The fields of the panel are other elements, still selectable.
- **Shift with the Text tool's drag**: today a text never stays square (`interaction.md` §1, the box dragged, agent's choice of US-064; `isSquare = drawing.isShift && drawing.tool !== "text"`, `playground/shape-drawing/show-shape-drawing.ts`). The Product Owner asks it square, as every other tool's box (2026-10-23): (10, 10) to (130, 90) with Shift gives a box 120 × 120 at (10, 10), the text set in it. The agent's choice is superseded; no source needed.

## 9. Shift with the wheel (correction 9)

- **Penpot**: on any platform but macOS, Shift with the wheel turns the vertical delta into a pan across; on macOS the deltas are taken as sent, "macOS sends delta-x automatically" (`schedule-scroll!`, `REF-PENPOT-VIEWPORT-ACTIONS`).
- **Penpot's `normalize_wheel`**, adapted from Facebook's: "spinX is a 2-finger side drag on the trackpad, and a shift + wheel turn with a mouse"; a slow step of a mouse on macOS and Chrome is 4 pixels with a `wheelDelta` of −120, a trackpad's 2 pixels with −6 (`REF-PENPOT-NORMALIZE-WHEEL`).
- **Figma**: its pages read state the trackpad's pinch and the mouse's `Cmd` + wheel for the zoom (`REF-FIGMA-PIXEL-GRID`), not how it tells a trackpad from a mouse; not found.

Today, with Shift, the page pans across by the sum of both deltas, the vertical one included (`readWheelPan`, `playground/view/listen-to-wheel.ts`): a trackpad's diagonal swipe of (30, 40) with Shift pans (70, 0). Penpot's rule needs no guess of the device: on macOS, where the system already turns a mouse's Shift + wheel across, the deltas are taken as sent, (30, 40) panning (30, 40); elsewhere a mouse's `deltaY` of 100 with Shift pans (100, 0). Telling macOS needs `navigator.userAgentData.platform` or `navigator.platform`, not read yet (a source for the story's first task). Question 7.

## 10. Several shapes' width and height (correction 10)

**Reproduced in the page's tests** (happy-dom, a test file deleted after), each value computed by hand: the page's rectangle 120 × 80 at (0, 0), a hexagon and a round shape, W 150, then H `+10`, `-100` in W, 0 in H, 30 in W: every shape took each value, the fields showing 150 or Mixed as they should; A turned by 30° and B by 45°, W 150 keeps each turned top-left corner (A's position (−2.00962, 7.5), computed by hand from its corner (28.03848, −24.64102)); a text in a box 100 × 20 turned by 23°, W 150 then H 60, its box taken. **In Chromium**, W 150 applied by Enter, by Tab, by a click on the empty drawing, on A and on B: each time both shapes 150 wide. Not reproduced: the values typed compute as `interaction.md` §2 says.

**Found while reproducing**: with one shape edited, a value typed then the drawing clicked within the same frame is lost, the click leaving the shape as last redrawn: B placed at (200, −5), 20 typed in Y, then Tab and a click on the empty drawing at once, B stays at y −5; 20 ms later, it is at y 20. A person rarely clicks within 16 ms of a key, but a slow frame lengthens the window; the cure is to draw the redraw asked before the panel leaves its shape.

**What may have looked wrong**, each as the rules say: a hexagon's W is its box's, its drawn width smaller (not stretched); a turned shape's W is its own width, not the width it covers on screen; a text that fits its content keeps its size, so W stays Mixed. Question 8 asks the Product Owner's steps.

## 11. The list of steps scrolled whole (correction 8)

**The code.** The list shows a window of at most 15 rows about the present (`shownHistoryWindow`, `historySteps`, `interaction.md` §7, The window of the list), its states left out counted on a line of each side; `#history-list` has no height bound nor `overflow`: no scroll was ever written, and AUD-044's fix of the History group split by the HTML parser hid none.

**A virtual list.** "List virtualization, or 'windowing', is the concept of only rendering what is visible to the user", the rows leaving the window replaced by the rows coming into it, a list of rows of one size the simple case, a few rows drawn beyond the window ("overscan", 1 by default) so that a quick scroll flashes no blank (`REF-WEBDEV-VIRTUALIZE`). The box scrolled holds a spacer as tall as every row, its `scrollTop` "the number of pixels by which an element's content is scrolled from its top edge", a float (`REF-MDN-SCROLLTOP`): the first row in sight is `floor(scrollTop / h)`, a box of height `H` showing `ceil(H / h) + 1` rows. With `h` = 28.8 px (`min-height: 1.8rem`) and a box of 15 rows, 432 px, scrolled to 10 000 px: rows 347 to 362 drawn, whatever the history holds; each row named from its two states (`stepName`), only those drawn.

**The tallest box.** Measured in Chromium 151: a box asked 34 000 000 px tall is 33 554 428 px, its `scrollTop` at most 33 554 128 in a box of 300 px; 28 000 000 px is kept. Rows of 28.8 px reach that height at 1 165 084 rows. The history holds at most what its bound allows: 1 000 MB, the largest the Memory field takes, over the 1 244 bytes of a move of A alone, about 803 850 moves (`movesWithinSize`), 23.2 million px of rows: within Chromium's height; Firefox's and Safari's were not measured (question 9 asks whether a larger list matters).

## 12. The stories and their size

| Story | Tasks | What it holds |
| --- | --- | --- |
| EN-150, the playground's tests fast again | 2 | the mount's listeners removed at the next mount; `selectElement` by its id; the file and the hue slider's test measured |
| US-080, the helps seen on any color, what a rectangle takes, no page text selected | 3 | the double outline of the hover, the selection, the dashed box and the rectangle; the shapes a rectangle takes outlined while it is dragged; `user-select: none` on the drawing |
| US-081, the arrows in the fields and on the drawing | 3 | Up and Down in the fields of several shapes; one step for an arrow held in a field; two arrows held move diagonally |
| US-082, a text's box square, the wheel with Shift, several shapes' sizes | 3 | Shift with the Text tool; Shift with the wheel on macOS; a value typed then a click in the same frame, and the cases of §10 as tests |
| US-083, the list of steps scrolled whole | 3 | the rows in sight in the model; the list's box scrolled, its rows drawn as they come into sight, opened at the present; the names of the rows drawn alone |

Five stories: one over the four a feature holds (ADR-0084), which `tests/backlog.test.ts` checks (`[F22.AC5]`): F43 holds the first four, and US-083 is drafted in SP-031, filed once its question 1 places it.

## 13. What remains open

The questions for the Product Owner, in SP-031, each with options and an example: the split (1), the outlines' colors (2), what a rectangle shows (3), the arrows in a Mixed field (4), an arrow held in a field (5), two arrows held (6), the wheel with Shift (7), correction 10's steps (8), the list's height and its largest size (9). Not read and not needed now: Figma's key for an arrow in a field, its trackpad's detection, Illustrator's outlines (Adobe's help answering 403, note 0031).
