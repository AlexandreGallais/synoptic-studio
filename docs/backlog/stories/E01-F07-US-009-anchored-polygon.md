---
id: US-009
epic: E01
feature: F07
title: Polygon anchored in its box
status: done
points: 2
---

# E01 · F07 · US-009 — Polygon anchored in its box

As a symbol designer, I want to anchor a regular polygon at one of nine places of its box so that a face lands on the edge I choose, on an integer coordinate.

## Acceptance criteria

- Given a polygon with a horizontal and a vertical alignment, when it is created, then both are stored with n, width, height and radius; any other value is refused (F07.AC1).
- Given center and center, when the contour is built, then it is exactly F02's (F07.AC1).
- Given n = 3 in 100 × 100 anchored bottom-center, when the contour is built, then its base is written at y = 100 and its apex at y = 13.39746 (F07.AC2).
- Given n = 3 in 100 × 100, when only the horizontal alignment changes, then the written contour does not change, and the alignment is still stored (F07.AC3).
- Given n = 4 in 100 × 50 anchored right and a radius of 10, when the contour is built, then its right side stays at x = 100; given n = 3 anchored top and a radius of 10, then its apex leaves y = 0 (F07.AC4).

## Product Owner test

None of its own: this story has no playground control yet; its cases are shown by US-010's card.

## Tasks

One task = one commit, referenced as `US-009.Tn`.

- [x] T1 — `anchor` (`{ horizontal, vertical }`) in `RegularPolygon` (note 0005), validity, contour with the alignments; tests of the cases above (2 h)
- [x] T2 — Domain (`shapes.md` §3, glossary: anchor, distinct from stroke alignment) and re-exports (0.5 h)
