---
id: EN-011
epic: E01
feature: F08
title: A free shape in the model
status: ready
points: 2
---

# E01 · F08 · EN-011 — A free shape in the model

A free shape stores its vertices and their requested radii, in contour order; its width and height are its bounding box. A regular polygon gives a free shape with the same vertices and radii; a rectangle too.

## Acceptance criteria

- Given a regular polygon, when it is turned into a free shape, then its vertices, radii and drawing are the same, with no number of corners nor anchor (F08.AC1).
- Given a rectangle, when it is turned into a free shape, then it has its 4 vertices and its radii (F08.AC2).
- Given a free shape, when it is checked, then it has at least 3 vertices and one radius per vertex, integers ≥ 0, otherwise it is refused (F08.AC4).
- Given a free shape, when its size is read, then width and height are those of its bounding box (F08.AC2).

## Product Owner test

None of its own: the free shape is shown by US-014's card.

## Tasks

One task = one commit, referenced as `EN-011.Tn`.

- [ ] T1 — `FreeShape` (ADR-0031, `REF-INKSCAPE-OBJECT-TO-PATH`, note 0007), validity, corners, bounding box, conversion from a polygon and a rectangle; tests (2 h)
- [ ] T2 — Domain (`shapes.md`), re-exports (0.5 h)
