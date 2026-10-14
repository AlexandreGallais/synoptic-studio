---
id: US-030
epic: E14
feature: F10
title: Convert a round shape to a free shape, two ways
status: ready
points: 1
---

# E14 · F10 · US-030 — Convert a round shape to a free shape, two ways

As a designer, I want to convert a round shape either to its 4 points on the curve with handles, or to the 4 rounded corners of its box, so that I can edit it as a free shape (Product Owner, 2026-10-12).

## Acceptance criteria

- Given a round shape 120 × 80 in Points mode, when **Convert to free shape** is pressed, then the shape is a free shape of 4 points in handles mode and the drawing does not move.
- Given the same, when **Convert to rounded rectangle** is pressed, then the shape is a free shape of 4 corners of radius 40: a capsule.
- Given a round shape of width 0, then both buttons are greyed.

## Product Owner test

Written with the story.

## Tasks

One task = one commit, referenced as `US-030.Tn`.

- [ ] T1 — The two conversion buttons; playground tests and test card (1 h)
