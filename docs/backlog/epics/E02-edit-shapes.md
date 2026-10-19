---
id: E02
title: Select and move shapes on one drawing
status: draft
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
| Several shapes on one drawing | the drawing as an ordered list of shapes, the last drawn on top, flat until E13 makes it a tree; a tool adds a new shape, Delete removes the selected ones (2 to 3) | to refine |
| Select shapes | a click selects, Shift + click or Ctrl + click adds or removes (`interaction.md` §2); a selection rectangle; the panel shows the selection's x, y, width and height, Mixed where they differ (3) | to refine |
| Move shapes on integers | dragged by the mouse, nudged by the arrow keys, x and y typed for several shapes, each landing on integers (2 to 3) | to refine |
| Undo and redo | a history of commands (Q7, to be confirmed), every change of the drawing undone and redone, by the keyboard and by buttons (2 to 3) | to refine |
| Zoom and pan | zoom about the pointer, pan, fit the drawing to the window; the shapes' numbers unchanged by the view (2 to 3) | to refine |

## Refinement

| Story | Status |
| --- | --- |
| [SP-025](../stories/E02-SP-025-cut-the-editing-work-into-small-features.md) — Cut the editing work into small features | in-progress |

## Working method for E02 and the epics cut from it

- **The method of RET-004 and ADR-0086** (Product Owner, 2026-10-18): features of 2 to 4 stories, one at a time, each refined before its stories; per story `T1` the interface and its failing tests, `T2` the code; one agent at a time, a fresh worker per story; a story merged into its feature and a feature into `develop` by merge commits; a review at each return of the Product Owner, no closing at E02's end. Tools that fail are cards, fixed at the next review; E17's were cut into E17's features F23 and F29 to F33 on 2026-10-18 (SP-024).
- **The model chosen per task** (Product Owner, 2026-10-16, at RET-003): a lighter model for logs, backlog edits, merges and step texts; the strongest for mathematics, audits and research. Measured during E02: tokens per story against E15's 24 M.
- **Checks by level** (ADR-0064): no independent review per story, a thorough audit per feature, a full audit beside the naming pass.
- **A trial of sessions** (Product Owner, 2026-10-16, at RET-003): the playground's specification written during the session from the requests, its code updated once before the validation; the heavy checks at the end of a session. Which checks move, and how a session stops in the middle, are specified before the trial starts.
