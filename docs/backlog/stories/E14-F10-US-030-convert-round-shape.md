---
id: US-030
epic: E14
feature: F10
title: Convert a round shape to a free shape, two ways
status: done
points: 1
---

# E14 · F10 · US-030 — Convert a round shape to a free shape, two ways

As a designer, I want to convert a round shape either to its 4 points on the curve with handles, or to the 4 rounded corners of its box, so that I can edit it as a free shape (Product Owner, 2026-10-12).

## Acceptance criteria

- Given a round shape 120 × 80 in Points mode, when **Convert to free shape** is pressed, then the shape is a free shape of 4 points in handles mode and the drawing does not move.
- Given the same, when **Convert to rounded rectangle** is pressed, then the shape is a free shape of 4 corners of radius 40: a capsule.
- Given a round shape of width 0, then both buttons are greyed.

## Product Owner test

| Step | Do                                                                     | You should see                                                               |
| ---- | ---------------------------------------------------------------------- | ---------------------------------------------------------------------------- |
| 1    | **Round** 120 × 80, **Points**, **Convert to free shape**              | nothing moves; the shape reads **Free shape**, 4 points                      |
| 2    | Click point 1                                                          | **Handles** pressed, Mirror angle and length: its handles can be typed (F12) |
| 3    | **Round** 120 × 80 again, **Points**, **Convert to rounded rectangle** | a capsule: the 4 corners of the box, radius 40                               |
| 4    | **Round**, width 0, **Points**                                         | a vertical line; both conversion buttons greyed                              |

## Tasks

One task = one commit, referenced as `US-030.Tn`.

- [x] T1 — The two conversion buttons; playground tests and test card (1 h)

## Decisions during the story

| Decision                                                                                               | Reason                                                                       |
| ------------------------------------------------------------------------------------------------------ | ---------------------------------------------------------------------------- |
| the converted shape writes its own radii in the Radius field: 0 for its points, 40 for the rounded box | as a converted polygon (F08): the free shape's radii are what is edited next |
| **Convert to free shape** converts a polygon or a round shape; the hint line says "This shape"         | one button, one rule for both shapes that keep their points                  |

## Review

Verifier, before the last commit, with probe tests: no bug. Tests added: a turned round shape converted both ways, a box of height 0, the guard behind the greyed buttons, the hint line, the radii after converting, the handles switch hidden on every point.
