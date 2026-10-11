---
id: US-016
epic: E01
feature: F08
title: Delete points
status: ready
points: 2
---

# E01 · F08 · US-016 — Delete points

As a symbol designer, I want to delete the selected points of a shape, so that I can simplify it.

## Acceptance criteria

- Given selected points of a free shape or a rectangle, when Delete is clicked or the Delete key pressed, then they are removed and their neighbours joined by a straight edge (F08.AC4).
- Given a deletion that would leave fewer than 3 points, when it is asked, then it is refused with a message (F08.AC4).
- Given a regular polygon, when a deletion is asked, then it is refused with a message to convert it first (F08.AC4).
- Given no point selected, when the shape is shown, then Delete is disabled; given the first and the last points deleted together, then their neighbours wrap round the contour (F08.AC4).

## Product Owner test

Written with the story.

## Tasks

One task = one commit, referenced as `US-016.Tn`.

- [ ] T1 — Removing vertices and their radii (`REF-INKSCAPE-NODE-OPS`, note 0007), pure function; tests (1 h)
- [ ] T2 — Playground button and key, messages; Product Owner test card (1 h)
