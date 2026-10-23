---
id: SP-028
epic: E02
feature: F39
title: Refine the moves of shapes
status: done
points: 3
---

# E02 · F39 · SP-028 — Refine the moves of shapes

The spike that opens F39, its research done in full before its stories (Product Owner, 2026-10-22): the sources, the code each story will touch, every question at once, the feature's size known before any code.

## Acceptance criteria

- Given Figma, Inkscape, Penpot, Sketch and Affinity Designer, when the spike ends, then how each moves a selection by the mouse (the press on a shape, a shape not selected pressed, a threshold before a drag starts) and by the arrow keys (the step, with Shift; a key held) is recorded from read sources, beside the default keys of `interaction.md` §1.
- Given the rules of 2026-10-22 (a drag moves freely, the anchor lands on integers at the release, the top-left of the straight box by default, the shape's own fractions kept) and the playground's code after F38 (`playground/selection/`, `playground/selection-drag/`, `playground/kept-shapes/`, `src/model/panel-values/`, `straightBoxShift`), when the spike ends, then F39's stories are written, 2 to 4 of at most 3 tasks each, criteria with numeric examples, the size of each checked against the code; a larger scope cut into a further feature, proposed with an example.
- Given what the sources and the code leave open, when the spike ends, then every question for the Product Owner is listed at once, each with options and a numeric example.

## Questions

- What moves when a shape not selected is pressed and dragged, and when one of several selected is?
- What step do the arrow keys move by, with and without Shift, and how does a turned shape land on integers?

The sources, the code read and each story's size: [note 0029](../../research/notes/0029-f39-move-shapes-on-integers.md). The stories: EN-145 (the test of F02's fifth criterion made fast, and the band of a large rounded shape with answer 8 (a)), US-072 (a drag moves the shapes selected), US-073 (the arrows nudge them); EN-145 is ready, the two others draft until the questions below are answered. Settled by the sources, two agreeing: a press on a shape not selected, dragged, selects it alone and moves it (Inkscape, Penpot); a press on one of several selected moves them all (Inkscape, Sketch, Penpot). F39 holds three stories, four with answer 7 (a) if Shift's axis outgrows US-072's three tasks: no further feature is needed.

## Questions for the Product Owner

The examples draw, unless said otherwise, a rectangle A, 100 × 50 at (10, 20), straight, and a rectangle B, 100 × 50 at (200, 20) turned by 30° about its middle (250, 45), its straight box, the smallest upright box around it, at (194.19873, −1.65064). Every landing rounds to the nearest integer, a tie rounded up, as a click places a shape (Q45).

1. **Which point of a shape lands on integers** (US-072, US-073). B dragged by (10.6, 3.2).
   - (a) The top-left of its straight box, the X and Y the panel shows for one shape or several (Q55, Q57) and the point the integer grid of `interaction.md` §4 snaps, as Penpot rounds each shape's upright box: the box lands at (205, 2), the panel shows X 205, Y 2, and the stored position is (210.80127, 23.65064).
   - (b) Its stored position, the top-left of its box before it turns (ADR-0035): the position lands at (211, 23), and the panel shows X 205.19873, Y 1.34936.
   - Both give the same on a straight rectangle: A dragged by (30.6, 19.4) lands at (41, 39) either way.
2. **Several shapes landing** (US-072, US-073). A and B selected, A pressed and dragged by (10.6, 3.2), with 1 (a).
   - (a) Each by its own anchor, as Penpot snaps each shape to its pixel grid: A at (21, 23), B's straight box at (205, 2); the distance from A's left to B's, 184.19873, becomes 184.
   - (b) Together, by the top-left of their dashed box, from (10, −1.65064) to (21, 2): every shape goes by the same shift (11, 3.65064), A to (21, 23.65064), B's straight box to (205.19873, 2); their distances are kept, A's Y then showing 23.65064.
3. **A nudge from an anchor holding a fraction** (US-073). A′, A turned by 30° about its middle after it was placed, its straight box at x 4.19873.
   - (a) By exactly 1, or 10 with Shift, as Penpot, which never rounds a nudge: Right gives 5.19873, Left 3.19873, Shift + Left −5.80127; the fraction stays until a drag lands it.
   - (b) To the next integer that way, then by whole steps: Right gives 5, Left 4, Shift + Left −5.
   - (c) By 1 or 10, then to the nearest integer, as a drag lands: Right gives 5, Left 3, Shift + Left −6.
   - All three move A, at x 10, to 11 by Right, 9 by Left and 0 by Shift + Left.
4. **How far the pointer goes before a press on a shape is a move** (US-072). A selected, pressed at (50, 40), the hand trembling to (50.7, 40.2) before the release.
   - (a) Off the press's integer point, as a tool's drag and the selection rectangle today (F37): (50.7, 40.2) rounds to (51, 40), so it is a move, and A lands at (11, 20) instead of being clicked.
   - (b) 4 screen pixels from the press, the reach a shape without area is picked from (Q47): it is a click, A stays at (10, 20); released at (54.5, 40), A lands at (15, 20).
   - (c) 8 screen pixels, GTK's default, or 10, Penpot's: it is a click; released at (54.5, 40), still a click.
   - With (b) or (c), the selection rectangle and a tool's drag keep their threshold of today, or take the same.
5. **Shift held at the press on a shape** (US-072). A selected; B, not selected, pressed with Shift and dragged by (30.6, 19.4).
   - (a) B is added and both move, as Penpot: A at (41, 39), B's straight box at (225, 18); pressed with Shift on A instead, A stays selected and both move.
   - (b) A selection rectangle starts from the press, adding the shapes it touches, as Inkscape draws a rubberband "even if you start from an object".
   - (c) Nothing, as today: no move and no rectangle, and the release is no click.
6. **A press on an empty place inside the dashed box of several** (US-072). A and D, a straight rectangle 100 × 50 at (200, 20), selected, their dashed box from (10, 20) to (300, 70); pressed at (150, 40), between them, and dragged by (30.6, 0).
   - (a) A selection rectangle, as from any empty place (F37, Inkscape, Penpot): it runs from (150, 40) to (181, 40), takes no shape, and selects none, or keeps A and D with Shift.
   - (b) A and D move, as Sketch moves several "from anywhere inside the outer selection box": A to (41, 20), D to (231, 20).
7. **Shift held during a move** (US-072). A dragged by (30.6, 19.4) with Shift held.
   - (a) In F39, Shift keeps the axis of the larger displacement, as Sketch and Penpot (Inkscape uses Ctrl, which Q50 gives to snapping off with E19): A lands at (41, 20); Shift released during the drag frees the other axis again. A fourth story if it outgrows US-072's three tasks.
   - (b) Later, with E19's snapping or E20: in F39 Shift does nothing during a move, and A lands at (41, 39).
8. **The band about a large rounded shape** (EN-145). A rectangle 99 999 × 100 000 of radius 100 000, selected, takes 1 063 ms to redraw, its band following each corner by about 39 270 chords of at most 2 screen pixels (Q48); a drag redraws at each pointer move.
   - (a) In F39, the chords chosen by how far they stray from the curve, at most a tenth of a screen pixel: about 393 a corner at that size, while a corner of radius 10 gets chords of about 2.8 instead of 2; a derivation with the `math-expert`, EN-145's second task, Q48 changed.
   - (b) The band kept as it is: only the test is made fast (EN-145's first task); dragging so large a rounded shape takes about a second a move, accepted.
   - (c) As (b) in F39, the chords by their deviation an EN of E20 later.

## Tasks

One task = one commit, referenced as `SP-028.Tn`.

- [x] T1 — The sources found, read and recorded, the research note (2 h)
- [x] T2 — The code read, F39's stories written, the questions for the Product Owner listed (2 h)

## Answers of the Product Owner

Given on 2026-10-23:

1. (a) The top-left of the straight box lands on integers, the X and Y the panel shows: B's box at (205, 2).
2. (b) Several shapes land together, by the top-left of their dashed box, every shape moved by the same shift, their distances kept.
3. (a) A nudge moves by exactly 1, or 10 with Shift; a fraction stays until a drag lands it.
4. (a) A press becomes a move as soon as the pointer leaves the press's integer point, as a tool's drag and the selection rectangle.
5. (a) Shift held at the press on a shape not selected adds it, and every shape selected moves.
6. (a) A press on an empty place inside the dashed box starts a selection rectangle, as from any empty place.
7. (a) Shift held during a move keeps the axis of the larger displacement, released frees the other again; a fourth story if it outgrows US-072.
8. Neither option: the helps drawn in the layer above the drawing are drawn only within the area shown, its padding included, so that a band about a large rounded shape is computed only where it is seen. That needs the area shown, which E02's zoom and pan feature brings: placed there as an EN; in F39 only the test is made fast (EN-145's first task).
