---
id: US-029
epic: E14
feature: F10
title: Draw a circle or an ellipse with the Round tool
status: ready
points: 2
---

# E14 · F10 · US-029 — Draw a circle or an ellipse with the Round tool

As a designer, I want a Round tool that draws a circle in a square box and an ellipse otherwise, so that I never type a radius that must stay at its maximum.

## Acceptance criteria

- Given the playground, when **Round** is chosen with 100 × 100, then a circle of diameter 100 is drawn, its path four `C` commands from (50, 0).
- Given **Round** with 120 × 80, then an ellipse touching the four sides of its box is drawn; its straight box reads 120 × 80.
- Given a round shape, when the panel is shown, then it has width, height, angle and position, and no radius nor anchor; in Points mode its 4 points are listed, not editable.
- Given a round shape turned by 30°, then it turns about the centre of its box, as every shape.

## Product Owner test

Written with the story.

## Tasks

One task = one commit, referenced as `US-029.Tn`.

- [ ] T1 — The Round tool and the round shape's panel in the playground; playground tests and test card (2 h)
