---
id: SP-031
epic: E02
feature: F43
title: Refine the corrections from the first try
status: ready
points: 2
---

# E02 · F43 · SP-031 — Refine the corrections from the first try

The spike that opens F43, its research done in full before its stories (Product Owner, 2026-10-22): the sources, the code each story will touch, every question at once, the feature's size known before any code.

## Acceptance criteria

- Given the corrections the Product Owner asked for on 2026-10-23 (E02's file, Remarks from the Product Owner's first try), when the spike ends, then each is traced to the playground's code it touches, what graphic editors do is read where the choice is not obvious, and F43's stories are written, 2 to 4 of at most 3 tasks each, criteria with numeric examples; a larger scope proposed as a split, with an example.
- Given what the sources and the code leave open, when the spike ends, then every question for the Product Owner is listed at once, each with options and a numeric example.

## The corrections asked

1. **The helps visible on any color**: the hover, the selection and the selection rectangle's outlines are blue, unseen on a blue shape; a double outline, dark and light, seen on every color (how Figma, Penpot and Inkscape draw them).
2. **The selection rectangle shows in advance** the shapes it would take, outlined as hovered, while it is dragged.
3. **A field showing Mixed** is stepped by the arrows as a number field (Up adds 1 to each shape, Shift + Up 10), so that several shapes' values move as quickly as one shape's.
4. **An arrow held in a number field** makes one history step once the value stops changing, not one per key repeat, as an arrow held on the drawing (SP-029's answer 3).
5. **Two arrows held at once** (Up and Right) move the selection diagonally.
6. **A text on the drawing is not selected as page text**: a press, a drag or a double click on it never highlights its characters (`user-select: none` on the drawing), the shape still picked.
7. **Shift while drawing a text's box** keeps it square, as every other tool's box.
8. **The history's list scrolls through every step** (Product Owner, 2026-10-23): today it shows a window of the steps near the present (`interaction.md` §7, The window of the list), so that a history of a million steps does not slow the page; the list should instead give access to all of them by a scroll that feels whole, only the rows in sight drawn, the others drawn as they scroll into sight (a virtual list), so that its length never slows it. AUD-044 fixed the History group split by the HTML parser: check whether it hid a scroll too.
9. **Shift with the wheel**: panning across is a mouse's need; with a trackpad, whose two fingers already pan both ways, Shift should not force the pan across (how Figma tells a trackpad from a mouse, `REF-MDN-WHEEL-DELTAMODE`, Penpot's `normalize_wheel`).
10. **Width and height typed for several shapes** were seen not always to compute right: reproduce, fix if wrong.

Placed elsewhere, not in F43: the playground made responsive with a Figma-like toolbar at the bottom, icons for the selector, the shapes and the multiple selection (E20, the interface designed last); settings for the pan's speed and the zoom's strength, an expert menu (E20).

And first in F43's order, a blocker (main thread, 2026-10-23): `playground/history/undo-and-redo.test.ts` takes 80 to 140 s under `check:all` and its test `[F40.AC2] takes a step at each key on the hue slider…` goes over Vitest's 5 s, failing pushes; made faster, not given a longer limit (`docs/conventions/testing.md`, Isolation and time).

## Questions

- Where does the slow test of the history spend its time, and what makes it faster without raising its limit?
- How do graphic editors draw their outlines so that they show on any color, and do they show what a selection rectangle will take while it is dragged?
- What does an arrow do in a field of several values, an arrow held in a number field, two arrows held at once?
- How is Shift with the wheel read on a trackpad and on a mouse?
- Do several shapes' width and height compute wrong, reproduced in the tests and in a browser?
- How is a list drawn as it scrolls, and how tall a box does a browser scroll?

The sources, the code read and the measures: [note 0032](../../research/notes/0032-f43-corrections-from-the-first-try.md). **The blocker** (§2): the page mounted in each test's `beforeEach` leaves its 14 listeners on the document, the same through a file, so that the 48th test hears each key 49 times, 7 undos redrawing the page 343 times in 1 863 ms against 14 times in 104 ms; and happy-dom's `querySelector`, which `selectElement` calls at each redraw, walks the tree after each change, 16.8 s of the file's 27.2 s profiled; not the redraws of F42, one per key. Both cures tried in place: the file from 29.2 s to 3.9 s alone, the hue slider's test from 2 447 ms to 116 ms, four other playground files from 2.3–4.1 s to 0.8–1.5 s (EN-150). **Settled by the sources**, two agreeing: the shapes a selection rectangle takes show while it is dragged (Figma, a blue box about each; Penpot, selected every 100 ms); `user-select: none` stops a selection started in the drawing (CSS UI 4, a specification). **Measured**: in Chromium, an arrow held in a number field fires a `change` at each repeat, 5 steps for 5 keys 40 ms apart; Chromium's tallest box is 33 554 428 pixels. **Correction 10 not reproduced**: every case computed by hand matched, in the tests and in Chromium; reproducing it found a value typed for one shape lost by a click on the drawing within the same frame (US-082). **Correction 8 reproduced**: the list never had a scroll, `#history-list` having no height bound nor `overflow`, and AUD-044 hid none (US-083). The stories: EN-150 (the blocker), US-080 (corrections 1, 2, 6), US-081 (3, 4, 5), US-082 (7, 9, 10), in F43; US-083 (8), drafted under For the main thread, a fifth story `tests/backlog.test.ts` refuses in F43 (ADR-0084), filed once question 1 places it; all draft until the questions below are answered.

## Questions for the Product Owner

The examples use the opening view, 100 %, and A, B and C, rectangles 100 × 100 at (0, 0), (150, 0) and (300, 0), drawn in the page's blue `#4a7bd0`, unless a question says otherwise.

1. **Where the fifth story goes** (ADR-0084: a feature holds 2 to 4 stories; the corrections and the blocker make 5).
   - (a) F43 keeps EN-150, US-080, US-081 and US-082, as filed; the list scrolled whole, US-083, becomes a feature of its own in E02, F44, after F43 and before the merge into `main`, with its audit: 3 tasks, 1 story, below the 2 a feature holds, so F44 would take a second story or the limit's lower bound passed once.
   - (b) EN-150 leaves F43, worked first on F43's branch as a fix of its own outside the count, and F43 holds US-080 to US-083.
   - (c) US-083 goes into E20 with the interface designed last, the list keeping its window of 15 rows until then.
2. **The outlines seen on any color** (US-080, correction 1). On A's own blue, today's outline, `#4a7bd0` 2 pixels wide, cannot be seen.
   - (a) Inkscape's: the accent's band, 2 pixels, over a white band 4 pixels wide, 1 pixel of white showing on each side; the dashed box's dashes in the accent, 4 long every 7, and the 3 between them white; the selection rectangle's band, 1 pixel, over a white one of 3.
   - (b) A dark band and a light one side by side, each 1 pixel: black `#000000` inside, white outside, 2 pixels in all as today, no blue; the dashes black and white; the hover and the selection then alike in color.
   - (c) As (a), the light band at 75 % white, as Inkscape's controls (`outline-opacity: 75%`), a white shape behind A showing a pale grey edge instead of a white one.
3. **What a selection rectangle shows while it is dragged** (US-080, correction 2). The rectangle pressed at (−10, −10), the pointer at (200, 110).
   - (a) A and B outlined as under the pointer, as Figma's blue box about each, selected at the release only, the panel unchanged until then.
   - (b) A and B selected as it goes, as Penpot every 100 ms: the panel shows `2 shapes selected` during the drag; the pointer back to (100, 50), B is taken out.
   - (c) As (a), with Alt the shapes enclosed outlined in another color, so that what Alt changes is seen: A alone.
4. **An arrow in a field showing Mixed** (US-081, correction 3). A 100 wide and B 40 selected, W Mixed.
   - (a) Up adds 1 to each shape's own value, Shift + Up 10, Down and Shift + Down take them away, as `+1` typed: Up gives 101 and 41, Shift + Up 110 and 50; Down with B 0 wide refuses it for both, as `-1` typed.
   - (b) Penpot's: Up sets every shape to one value, from the field's minimum or 0, so that Up in W Mixed makes both 1 wide.
   - (c) As (a), and Up in X and Y as well as in W and H, X going right and Y down by 1, as a number field's Up adds 1.
5. **An arrow held in a number field** (US-081, correction 4). A's W at 100, Up held for a second, 30 keys sent.
   - (a) One step at the arrow's release, as an arrow held on the drawing (SP-029's answer 3): A 130 wide, one Ctrl + Z gives 100; a second hold after the release is a step of its own.
   - (b) One step once the value has not changed for 500 ms: a hold, a pause of a second, then another hold make two steps; a hold, a pause of 300 ms, and another make one.
   - (c) No step at the arrows at all, the step taken when the field is left or Enter pressed, as a value typed: the hold, then another hold, then Tab, make one step.
6. **Two arrows held at once** (US-081, correction 5). A selected at (10, 20), Up held, then Right pressed while Up is still held, the browser repeating Right's key 29 times.
   - (a) Each arrow's key moves A by the sum of the arrows held: Up's first key to (10, 19), then (+1, −1) at each of the 29, to (39, −10); Left and Right held together cancel out; Shift × 10 on both axes.
   - (b) The arrows held move A at a pace of their own, 30 steps a second whatever the system's repeat, by the sum of the arrows held, so that the speed is the same on every system: a second of Up and Right, (+30, −30).
   - (c) As (a), and a step of 1 on each axis once every 2 keys when both are held, so that a diagonal moves at the speed of one arrow, about 1.41 units a key.
7. **Shift with the wheel** (US-082, correction 9). The view at 200 %, origin (180, 130).
   - (a) Penpot's: on macOS, the wheel's deltas taken as sent, Shift or not, macOS already sending a mouse's Shift + wheel across: a trackpad's (30, 40) with Shift pans by (15, 20) units, to (195, 150), and a mouse's Shift + wheel, (100, 0), to (230, 130); elsewhere, Shift turns the vertical delta across as today, (0, 100) panning to (230, 130).
   - (b) A device guessed from the deltas: a delta in lines, or a whole multiple of 100 or of 120 pixels with no horizontal one, read as a mouse's wheel, turned across with Shift; anything else as a trackpad's, taken as sent: a trackpad's (30, 40) with Shift pans to (195, 150), a mouse's (0, 120) to (240, 130).
   - (c) Shift never changes the pan, the deltas always taken as sent: a mouse's (0, 100) with Shift on Windows pans down, to (180, 180), the mouse then panning across only by a horizontal wheel or a drag.
8. **Several shapes' width and height** (US-082, correction 10). Not reproduced: the page's rectangle, a hexagon and a round shape, typed 150 in W, then `+10` in H, `-100` in W, 0 in H and 30 in W, each took every value; A turned by 30° kept its turned corner; W applied by Enter, Tab or a click on the drawing gave the same widths in Chromium.
   - (a) The steps that went wrong, if they can be told again: which shapes, which field, which value, and what was seen against what was expected; until then, the cases above as tests and the click within the same frame fixed.
   - (b) What may have looked wrong was right by the rules, and one of them is to change: a hexagon's W its box's, the hexagon drawn narrower, not stretched; a turned shape's W its own width, not the width it covers; a text that fits its content keeping its size, W staying Mixed.
   - (c) Nothing more than the click within the same frame and the tests.
9. **The list's height and its largest size** (US-083, correction 8). Rows of 28.8 pixels.
   - (a) A box of 15 rows, 432 pixels, scrolled, opened with the present as its last row in sight; past the browser's tallest box, Chromium's 33 554 428 pixels, 1 165 084 rows, the scroll scaled, a pixel of the scroll bar then more than a row; the history's largest bound, about 803 850 moves of A in 1 000 MB, 23.2 million pixels, fits without it in Chromium.
   - (b) A box as tall as the window allows below the toolbar less 40 pixels, 15 rows or more on a tall screen, the rest as (a).
   - (c) As (a), the scaling left out: a list over the browser's box shows its last 1 165 084 rows, the oldest reached by Undo alone.
10. **Anything else** about F43's corrections, and the selection rectangle taking a shape it touches by one pixel, which the first try left to discuss: open for remarks.

## Tasks

- [x] T1 — The code read, the sources read, the questions for the Product Owner, F43's stories drafted (2 h)

One task = one commit, referenced as `SP-031.T1`.

## For the main thread

**F43's feature file and the stories' index** were written in `SP-031.T1`, as SP-030 wrote F42's: `tests/backlog.test.ts` refuses a story file missing from its feature's table and from the index, and the pre-push refuses shared files on `US` and `EN` branches only (`tools/shared-files`, `WORKER_TYPES`). F43's file now holds its four criteria, its four stories, draft, and its plan; to check against the rows written meanwhile before the `done` commit.

**The corrections make five stories**, one over ADR-0084's four, which `tests/backlog.test.ts` refuses (`[F22.AC5]`): US-083's draft is below, to file where question 1 places it, with its criterion (the list of Start and 1 000 moves scrolled through all of them, 16 rows drawn whatever it holds, the present in sight as it opens) in its feature's file.

**EN-150 is ready to be worked once question 1 is answered**: it decides nothing for the Product Owner (the tests' cost alone), so the main thread may start it before the other answers, to unblock the pushes.

**The reproductions** used a Playwright 1.62.1 found in the machine's npx cache and a headless Chromium of the Playwright cache, driven by scripts of the spike's scratch folder against `vite` on port 5291, nothing committed and no package added: a browser check of the repository's own would need its row in `docs/tooling/dependencies.md` and an ADR; a point for the next review, since corrections 4 and 10 showed what happy-dom cannot (a key repeated a frame apart, a click within a frame).

**US-083, drafted by SP-031, filed once question 1 is answered**: `tests/backlog.test.ts` refuses a fifth story in F43 (`[F22.AC5]`, ADR-0084), so its file waits for the answer, in F44 by option (a), in F43 by option (c). Its draft, as `docs/backlog/stories/E02-F44-US-083-the-list-of-steps-scrolled-whole.md` or `E02-F43-…` would hold it, its tasks without their boxes here so that this file counts its own task alone (`[ ]` to put back when filed):

```markdown
---
id: US-083
epic: E02
feature: F43
title: The list of steps scrolled whole
status: draft
points: 3
---

# E02 · F43 · US-083 — The list of steps scrolled whole

As a symbol designer, I want to scroll the history's list through every step it holds, so that I can go back to any of them, the list staying fast however long the history is.

Correction 8 of the Product Owner's first try (2026-10-23), the window of 15 rows about the present (US-077, `interaction.md` §7, The window of the list) superseded: the list gives access to every state the history holds by a scroll that feels whole, only the rows in sight drawn, the others drawn as they scroll into sight, a virtual list, so that its length never slows it ([note 0032](../../research/notes/0032-f43-corrections-from-the-first-try.md), §11; `REF-WEBDEV-VIRTUALIZE`, `REF-MDN-SCROLLTOP`). The list never had a scroll: `#history-list` has no height bound nor `overflow`, and AUD-044's fix hid none. Placed in F43 or in a feature of its own by SP-031's question 1; its height and largest size wait for question 9; the criteria take the options proposed first.

## Acceptance criteria

The examples use rows of 28.8 CSS pixels (`1.8rem`) and a list box of 15 rows, 432 pixels (question 9).

- Given a history of Start and 1 000 moves, the last the present, when the list opens, then it is scrolled so that the present is the last row in sight, its box scrollable over 1 001 × 28.8 = 28 828.8 pixels and scrolled to 28 828.8 − 432 = 28 396.8, rows 986 to 1 000 in sight and row 985 above them, the overscan, drawn: 16 rows, whatever the history holds.
- Given that list scrolled to 10 000 pixels, then rows `floor(10 000 / 28.8)` = 347 to 362 are in sight, 16 rows, and 346 and 363 drawn beside them, the overscan of 1 on each side (`REF-WEBDEV-VIRTUALIZE`): 18 rows drawn, each named by its step (`stepName`), the names computed for them alone.
- Given a row clicked after a scroll, then the history goes to its state as today (`sizedHistoryAt`, US-077), the present marked and the steps to redo greyed, the list keeping its scroll.
- Given a step taken while the list is open, then the list grows by a row, the new present in sight; an undo moves the present's mark without scrolling while it stays in sight.
- Given the lines `6 earlier steps` and `6 later steps` of today, then they are gone, every state having its row.
- Given the history's largest bound, 1 000 MB, about 803 850 moves of A alone, then the rows are 23.2 million pixels tall, within Chromium's tallest box, 33 554 428 pixels, measured (note 0032 §11); a list taller than the browser's box is scaled (question 9).
- Given Reset or a step of the guided test, then the list shows `Start` alone, unscrolled.

## Product Owner test

`npm run dev`, `http://localhost:5173/`. The page opens with a rectangle 120 × 80 at (0, 0), selected.

| Step | Do                                                                                                               | You should see                                                   |
| ---- | ---------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------- |
| 1    | Set Steps kept to 100, then hold Right for a few seconds outside the fields, released and pressed again 40 times | 40 moves in the history                                          |
| 2    | Open `Steps`                                                                                                     | the list ends at the present, a scroll bar beside it             |
| 3    | Scroll up to `Start`                                                                                             | every step listed, the scroll smooth, no line of steps left out  |
| 4    | Click a step in the middle                                                                                       | the drawing as it was after that step, the steps after it greyed |

## Tasks

- T1 — The rows in sight in the model (`src/model/history-list/`, `domain`): from the number of states, the scroll, the box's and a row's heights and the overscan, the first and last rows drawn and the spacer's height, the list scaled past the browser's tallest box as answered; `shownHistoryWindow` replaced; `interaction.md` §7 (The window of the list) and its Q row; hand-computed examples and a property test, failing (2 h)
- T2 — The list's box scrolled in the page, its spacer and its rows drawn from T1 at each `scroll` and each change of the history, opened at the present, the rows named only when drawn (`historySteps` on a range) (2 h)
- T3 — The list's tests of US-077 brought to the scroll, a history of 100 000 states measured: opening and a scroll each drawn within a frame (16.7 ms, `REF-MDN-RAF`) in Chromium; the Product Owner test (1 h)

Files: `src/model/history-list/` (`shown-history-window.ts`, `history-steps.ts`), `playground/history-list/` (`show-history-list.ts`, `create-history-rows.ts`, `create-left-out-rows.ts`), `playground/index.html` (`#history-list`), `docs/domain/interaction.md` §7. `@see` `REF-WEBDEV-VIRTUALIZE`, `REF-MDN-SCROLLTOP`, `REF-GIMP-UNDO-DIALOG`, `docs/domain/interaction.md#_7-navigation-and-history`.

One task = one commit, referenced as `US-083.T1` in the commit footer.
```
