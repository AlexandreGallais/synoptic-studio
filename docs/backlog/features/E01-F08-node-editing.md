---
id: F08
epic: E01
title: Node editing
status: draft
---

# E01 · F08 — Node editing

**Benefit hypothesis**: the designer reshapes any contour vertex by vertex.

Split from F03 (Product Owner, 2026-10-11).

## Decided in refinement (Product Owner, 2026-10-11)

- **A rectangle or a regular polygon becomes a free contour** when its vertices are edited, as in Figma and Inkscape: it loses its number of corners and its anchor. This happens by a **voluntary action**, never silently: « on la transforme par une action volontaire, oui ».
- The points mode of F03 is the way in (no double-click needed).
- **Moving a vertex by dragging it** on the drawing, as well as by typing x and y: « je veux pouvoir cliquer sur le point et le drag and drop pour pouvoir le bouger en X et Y » (Product Owner, 2026-10-11).
- **Free contour and x, y in the list of points** (Product Owner, VAL-004, 2026-10-11): « Il n'y a pas encore le passage en freeform, parce que je ne peux pas encore modifier le X, Y dans la liste » — confirmed for F08: the list of points of F03 gets editable x and y here.

## Acceptance criteria

1. Vertices can be selected, added on an edge, moved by integers and deleted.
2. Editing the vertices of a rectangle or a regular polygon first turns it into a free contour, by a voluntary action.

## Stories

To be written in refinement, after F03.
