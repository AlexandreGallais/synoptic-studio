---
id: US-050
epic: E15
feature: F17
title: A turned text's frames drawn as a turned shape's
status: done
points: 2
---

# E15 · F17 · US-050 — A turned text's frames drawn as a turned shape's

As a symbol designer, I want a turned text to show its frames as a turned shape does so that both read the same way.

From the Product Owner's validation of E15 (2026-10-15): a turned text's turned frame is drawn in blue, in the style of a shape's turned box, and its straight box dashed (`shapes.md`, Text, Turning).

## Acceptance criteria

- Given a text in a box 60 × 40 turned by 30, when it is selected, then its turned box is drawn in blue, in the style of a turned shape's box, and its straight box dashed, as for a rectangle 60 × 40 turned by 30.
- Given a text at 0°, when it is selected, then its frame is dashed, as before.
- Given an empty text or a box 0 wide, when selected, then no frame is drawn, as before.

## Product Owner test

| Step | Do                                             | You should see                                  |
| ---- | ---------------------------------------------- | ----------------------------------------------- |
| 1    | A rectangle 60 × 40 turned by 30, selected     | its blue turned box and its dashed straight box |
| 2    | A text in a box 60 × 40 turned by 30, selected | the same two frames, in the same styles         |

## Tasks

One task = one commit, referenced as `US-050.Tn`.

- [x] T1 — The turned text's frames drawn with the shapes' styles, with tests (2 h)
- [x] T2 — Test card and guided step (1 h)
