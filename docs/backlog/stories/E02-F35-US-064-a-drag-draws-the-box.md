---
id: US-064
epic: E02
feature: F35
title: A drag with a tool draws the shape's box
status: ready
points: 3
---

# E02 · F35 · US-064 — A drag with a tool draws the shape's box

As a symbol designer, I want to drag with a tool to draw the new shape at the size I want, so that it needs no resizing afterwards.

## Acceptance criteria

- Given the rectangle tool, when the pointer is pressed at (10, 10), dragged and released at (130, 90), then a rectangle 120 × 80 at (10, 10) is added; while dragging, the shape follows the pointer.
- Given a drag up and to the left, from (130, 90) to (10, 10), then the same rectangle is added: the box runs from the smaller to the larger corner.
- Given the pointer released where it was pressed, on the same integer point, then it counts as a click (US-063).
- Given the text tool dragged from (10, 10) to (210, 70), then a text in a box 200 × 60 is added.
- Given the round shape tool and Shift held, when the pointer is dragged from (10, 10) to (130, 90), then a circle in a box 120 × 120 is added: Shift keeps the box square, its side the larger of the two distances dragged, for every tool but text; Shift pressed or released during the drag changes the shape at once (Product Owner, 2026-10-22; Shift as in Figma, the key editable later, E20).
- Given a drag leaving a width or a height of 0, then what is added is the agent's choice, written in `interaction.md` §1.

## Product Owner test

| Step | Do | You should see |
| --- | --- | --- |

## Tasks

- [ ] T1 — The box from a press and a release: interface and failing tests (1 h)
- [ ] T2 — The box computed, the tests of T1 passing (1 h)
- [ ] T3 — The playground draws the shape live while dragging; the Product Owner test card (2 h)

One task = one commit, referenced as `US-064.Tn` in the commit footer.
