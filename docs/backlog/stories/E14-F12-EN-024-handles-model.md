---
id: EN-024
epic: E14
feature: F12
title: "A point's handles in the free shape model"
status: ready
points: 3
---

# E14 · F12 · EN-024 — A point's handles in the free shape model

Give a free shape's corner its handles beside its radius (ADR-0046, note 0013): a switch between the radius mode and the handles mode, Figma's three modes, two vectors stored exactly.

## Acceptance criteria

- Given a corner of a free shape, when it is created or converted (rectangle, polygon), then its handles are off, in mode angle and length, both vectors (0, 0), and its drawing is unchanged.
- Given a corner with radius 30, when its handles are switched on, then off, then its radius is still 30; with radius 0, still 0.
- Given mode angle and length, when the handle after is set to (30, −15), then the handle before is (−30, 15); in mode angle, the handle before keeps its length on the opposite direction; in mode none, it is unchanged.
- Given a stored free shape, when it is read back, then its handles are validated (finite numbers, a known mode) and a shape without handles from E01 still reads, its handles off.

## Tasks

One task = one commit, referenced as `EN-024.Tn`.

- [ ] T1 — Types and validity of handles, defaults on creation and conversion
- [ ] T2 — Setting one handle by the mode, switching the mode on and off, the radius kept
