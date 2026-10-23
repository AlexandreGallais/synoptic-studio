---
id: E02
title: Select and move shapes on one drawing
status: in-progress
---

# E02 — Select and move shapes on one drawing

A list of features grouped by theme, with no closing of its own (ADR-0086). Cut again on 2026-10-19 (SP-025), at the Product Owner's request, into small features: E02 holds the drawing of several shapes, their selection, their moves, the history and the view; resizing, turning and dragging points went to [E18](./E18-transform-shapes-by-hand.md), snapping and alignment to [E19](./E19-snap-align-and-distribute.md), the shapes' colors to [E04](./E04-outline-shapes.md), the ideas on ease of use to [E20](./E20-make-the-editor-easy-to-use.md).

| Field | Content |
| --- | --- |
| For | symbol designers |
| who | need to work on several shapes at once, exactly |
| the solution | a drawing of several shapes, a selector, a history and a view that zooms |
| is a | the editors' first interaction layer |
| that | every move lands on an integer, and every change can be undone |
| unlike | one shape at a time, typed in fields only, as in the playground today |
| our solution | shapes are added, selected by a click or a rectangle, moved by the mouse or the keys on integers, undone and redone; the view zooms and pans |
| Business outcomes | a symbol of several shapes is built, selected, moved, undone and redone in the playground |
| Leading indicators | features accepted |
| Non-functional requirements | integer positions after every move (ADR-0003) |
| In scope | several shapes on one drawing and their stacking order as drawn, selection by click and by rectangle, moves by the mouse and the keys, undo / redo (Q7), zoom and pan |
| Out of scope | resizing and turning by handles, dragged points (E18); snapping, align and distribute (E19); the shapes' colors (E04); the tree and the layers panel (E13) |

## Features

Ordered by the agent at SP-025 (2026-10-19), each needing the one before it. A feature gets its id when it starts, its questions in its own short spike (ADR-0086).

| Feature | Expected stories | Status |
| --- | --- | --- |
| [F35](../features/E02-F35-several-shapes-on-one-drawing.md) — Several shapes on one drawing | the drawing as an ordered list of shapes, the last drawn on top, flat until E13 makes it a tree; a click with a tool adds a shape 100 × 100 centred on the pointer, the playground migrated to Figma's way, a new shape's starting values researched (3) | in-progress |
| [F36](../features/E02-F36-draw-pick-and-remove-shapes.md) — Draw and pick shapes | a drag draws the box, Shift keeps it square; the shape at a point by its painted area, shown in a layer above the drawing, a click selects it and the panel loads it; split from F35, then Delete moved to F37, on 2026-10-22 (4) | in-progress |
| [F37](../features/E02-F37-select-shapes.md) — Select shapes | opened by SP-027, the usual keys of graphic editors and the default keys (`interaction.md` §1); Shift + click adds or removes, Ctrl + A, Escape; Delete (US-066, from F36); a selection rectangle taking what its painted area touches, Alt what it encloses (EN-144) (4) | done |
| [F38](../features/E02-F38-the-panel-for-several-shapes.md) — The panel for several shapes | X, Y, W and H of several shapes, Mixed where they differ; a value typed relative (`+50`) or fixed; cut from F37 on 2026-10-23 (2) | done |
| [F39](../features/E02-F39-move-shapes-on-integers.md) — Move shapes on integers | dragged by the mouse, nudged by the arrow keys (x and y typed for several shapes came with F38); a drag moves freely and at the release the shape's anchor lands on integers, its top-left by default, its own fractions kept (`interaction.md` §2) (2 to 3) | done |
| Undo and redo | one step per finished action, the selection alone never a step, the history bounded by a memory size (Q7, settled on 2026-10-22), by the keyboard and by buttons; opened by a short spike comparing a history of commands and one of states from read sources, made an epic of its own if it outgrows 4 stories (2 to 4) | to refine |
| Zoom and pan | zoom about the pointer, pan, fit the drawing to the window; the shapes' numbers unchanged by the view; the helps of the layer above the drawing drawn only within the area shown and its padding, so that a large rounded shape's band stays fast (Product Owner, 2026-10-23, SP-028's answer 8) (2 to 4) | to refine |

## Refinement

| Story | Status |
| --- | --- |
| [SP-025](../stories/E02-SP-025-cut-the-editing-work-into-small-features.md) — Cut the editing work into small features | done |

## Working method for E02 and the epics cut from it

- **The method of RET-004 and ADR-0086** (Product Owner, 2026-10-18): features of 2 to 4 stories, one at a time, each refined before its stories; per story `T1` the interface and its failing tests, `T2` the code; one agent at a time, a fresh worker per story; a story merged into its feature and a feature into `develop` by merge commits; a review at each return of the Product Owner, no closing at E02's end. Tools that fail are cards, fixed at the next review; E17's were cut into E17's features F23 and F29 to F33 on 2026-10-18 (SP-024).
- **The model chosen per task** (Product Owner, 2026-10-16, at RET-003): a lighter model for logs, backlog edits, merges and step texts; the strongest for mathematics, audits and research. Measured during E02: tokens per story against E15's 24 M.
- **Checks by level** (ADR-0064): no independent review per story, a thorough audit per feature, a full audit beside the naming pass.
- **A trial of sessions** (Product Owner, 2026-10-16, at RET-003): the playground's specification written during the session from the requests, its code updated once before the validation; the heavy checks at the end of a session. Which checks move, and how a session stops in the middle, are specified before the trial starts.
