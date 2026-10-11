---
id: US-010
epic: E01
feature: F07
title: Choose the anchor in the playground
status: ready
points: 2
---

# E01 · F07 · US-010 — Choose the anchor in the playground

As a symbol designer, I want a 3 × 3 grid next to the polygon's settings so that I see and change where the polygon sits in its box.

## Acceptance criteria

- Given the polygon shape, when the playground opens, then a 3 × 3 anchor grid is shown with the center cell selected; it is hidden for the rectangle (F07.AC5).
- Given the grid, when a cell is clicked or chosen with the arrow keys (radio group, reading order), then the polygon moves to that place of its box and the cell is shown selected (F07.AC5).
- Given a triangle in 100 × 100, when left, center or right is chosen on the same row, then the drawing does not move but the selected cell changes (F07.AC3).

## Product Owner test

Written with the story.

| Step | Do  | You should see |
| ---- | --- | -------------- |

## Tasks

One task = one commit, referenced as `US-010.Tn`.

- [ ] T1 — Anchor grid of nine native radio buttons (`REF-FIGMA-AUTO-LAYOUT` as functional reference, keyboard of `REF-WAI-APG-RADIO`, note 0005), hidden for the rectangle; playground tests (2 h)
- [ ] T2 — Product Owner test card; playground help text (0.5 h)
