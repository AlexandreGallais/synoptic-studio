---
id: SP-027
epic: E02
feature: F37
title: Refine the selection of several shapes
status: done
points: 3
---

# E02 · F37 · SP-027 — Refine the selection of several shapes

The short spike that opens F37 (ADR-0086), longer than F35's and F36's: the Product Owner asked on 2026-10-22 for a feature's research to be done in full before its stories, reading the code each will touch, so that every question is asked at once and the feature's size is known before any code (`docs/process/improvements.md`). It also holds the research on the usual shortcuts of graphic editors that E02's table set for this feature (Product Owner, 2026-10-22).

## Acceptance criteria

- Given Figma, Inkscape, Illustrator, Photoshop, Penpot, Sketch and Affinity Designer, when the spike ends, then the shortcuts for selecting, adding and removing from a selection, the selection rectangle (touching or enclosing), selecting all, deselecting, deleting, nudging by the arrow keys (and with Shift), undo and redo are recorded from read sources, and a table of defaults is written in `interaction.md` for E02, E18 and E19, Figma's as the default where they differ (`interaction.md` §1, Keys editable).
- Given the playground's code after F36 (`playground/selection/`, `playground/kept-shapes/`, the panel), when the spike ends, then F37's stories are written, 2 to 4 of at most 3 tasks each with US-066 among them, their criteria with numeric examples, the size of each checked against the code each will touch; a larger scope is cut into a further feature, proposed with an example.
- Given what the sources and the code leave open — the panel for several shapes (x, y, width, height, Mixed), what the rectangle selects, how the selection shows — when the spike ends, then every question for the Product Owner is listed at once, each with options and a numeric example.

## Questions

- Which keys and gestures select, add, remove, select all and deselect in each editor, and which default the editor takes?
- Does a selection rectangle take the shapes it touches or those it encloses, and does a modifier change that?
- What does the panel show and accept for several shapes, and how is Mixed typed over?

The sources, the code read and each story's size: [note 0028](../../research/notes/0028-f37-select-shapes.md). The default keys: `interaction.md` §1, The default keys (Q50). The stories: US-068 (Shift + click, Ctrl + A, Escape), US-066 (Delete), US-069 (the rectangle), US-070 (the panel for several shapes); US-066 is ready, the three others draft until the questions below are answered.

## Questions for the Product Owner

The examples draw, unless said otherwise, a rectangle A from (0, 0) to (100, 100), a rectangle B from (150, 0) to (250, 100) and a round shape C, a circle of diameter 100 in the box from (300, 0) to (400, 100), in that order.

1. **What a selection rectangle takes** (US-069). A rectangle dragged from (−10, −10) to (200, 110) holds A whole and half of B.
   - (a) The shapes it touches, as Figma and Sketch do by default: A and B. Alt held while dragging takes only those it encloses, as Sketch's `⌥`.
   - (b) The shapes it encloses, as Inkscape does by default: A alone. Alt held takes those it touches, as Inkscape's `Alt`.
   - (c) The shapes it touches, with no key for the other way: A and B, as Figma, whose users ask for Sketch's way on its forum.
2. **By what a shape is touched** (US-069; for touching only, enclosing giving the same answer both ways). A rectangle dragged from (390, 90) to (420, 120) covers the corner of C's box but not the circle, whose nearest point to (390, 90) lies 56.6 from its center (350, 50), beyond its radius of 50.
   - (a) By its painted area, as a click picks it: C is not selected. Needs an EN before US-069 computing whether a contour of segments, arcs and cubic sides touches a rectangle, with the `math-expert` (question 8).
   - (b) By its straight box, the box of its contour as drawn: C is selected. No new geometry.
3. **How several selected shapes show** (US-068). A and B selected.
   - (a) Each by its band, 2 screen pixels about its contour as drawn, as one shape is shown today (Q49): two outlines, nothing around them.
   - (b) Each by its band, and a dashed box around them all, from (0, 0) to (250, 100), as Inkscape frames each object and sets its arrows around the whole selection: three lines.
   - (c) A box around them all only, from (0, 0) to (250, 100): one line, the shapes themselves not outlined.
4. **What the panel shows for several shapes** (US-070). A at (10, 20), 100 × 50, and B at (200, 20), 40 × 40, both selected.
   - (a) Each shape's values, Mixed where they differ, as Figma and Penpot: X Mixed, Y 20, W Mixed, H Mixed.
   - (b) The box of the whole selection, as Inkscape and Affinity Designer: X 10, Y 20, W 230, H 50.
5. **What is typed for several shapes in F37** (US-070). The same A and B, 50 typed in X, then 460 in W.
   - (a) Nothing yet: the fields show the values, disabled; X and Y are typed with E02's next feature, which moves shapes, W and H with E18, which resizes them.
   - (b) Each shape set to the value, as Figma and Penpot: X 50 puts A at (50, 20) and B at (50, 20), one over the other; W 460 makes each 460 wide.
   - (c) The selection moved or scaled as one, as Inkscape: X 50 moves both by 40, A to (50, 20), B to (240, 20); W 460 scales both by 2, A 200 × 100 at (10, 20), B 80 × 80 at (390, 20), and most widths give fractions (300 scales by 1.304…), against the integers of ADR-0003.
6. **A text among several shapes** (US-070, with 4 (a) and 5 (b)). A text Pump, its block 87.3 × 38.4, not in a box, selected with A.
   - (a) Its W and H are its block's, 87.3 and 38.4; a width typed leaves it unchanged, a text without a box having no width to set.
   - (b) A width typed puts it in a box of that width, Fit set to Box, as a drag with the Text tool does: 460 typed gives a box 460 × 38.4.
   - (c) W and H are disabled while a text is among the shapes selected.
7. **Ctrl + click on the drawing** (US-068). A selected, C clicked with Ctrl held.
   - (a) Ctrl + click adds or removes, as Shift + click (the rule of 2026-10-11): A and C selected. When E13 brings groups, Figma's Ctrl + click, which picks a shape inside a group, needs another key.
   - (b) Shift + click alone adds or removes, Ctrl + click kept for picking inside groups with E13, as Figma: C alone selected until then.
8. **The cut of F37** (ADR-0084: 2 to 4 stories, each of at most 3 tasks). Its stories hold as written when 2 is (b) and 5 is (a).
   - (a) F37 holds US-068, US-066, US-069 and US-070, four stories, the panel showing several shapes' values only.
   - (b) F37 holds US-068, US-066, an EN for the painted area (2 (a)) and US-069; a feature F38, "The panel for several shapes", holds US-070 and the typing of 5 (b) or (c), two stories: one shows the values, one gives a value typed to the shapes, each shape moved and resized with a turned shape's corner kept (US-019) and a text as 6 decides.
   - (c) F37 holds US-068, US-066 and US-069; F38 holds US-070 alone, its typing left to E02's next feature and E18 (5 (a)).

## Tasks

One task = one commit, referenced as `SP-027.Tn`.

- [x] T1 — The sources found, read and recorded, the research note, the table of shortcuts (2 h)
- [x] T2 — The code read, F37's stories written, the questions for the Product Owner listed (2 h)

## Answers of the Product Owner

Given on 2026-10-22 and 2026-10-23:

1. (a) A selection rectangle takes the shapes it touches, as Figma; Alt held takes only those it encloses. Groups, which can hold shapes, come with E13, where hovering shows the group and a modifier reaches inside it: nothing to decide before.
2. (a) Touched by the painted area, as a click picks: C is not selected. An EN before US-069, with the `math-expert`.
3. (b) Each shape by its band, and a dashed box around the whole selection.
4. Mixed where the values differ; a relative value typed, `+50` in W, adds 50 to each shape's own width (100 gives 150, 50 gives 100).
5. A value typed without a sign sets each shape to it, as Figma: 460 in W makes A and B 460 wide.
6. (a) A text fitting its content keeps its size when W or H is typed for several shapes.
7. (b) Shift + click alone adds or removes; Ctrl + click is kept for picking inside a group with E13, as Figma: C alone selected until then. This supersedes the rule of 2026-10-11 for the drawing (the list of points keeps its Ctrl + click).
8. (b) F37 holds US-068, US-066, EN-144 (a shape touched by a rectangle, by its painted area) and US-069; F38, "The panel for several shapes", holds US-070 (the values shown, Mixed) and US-071 (a value typed, relative or fixed).
