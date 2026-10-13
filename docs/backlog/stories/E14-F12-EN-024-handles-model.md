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
- Given mode angle and the handle typed of length 0, when it is set, then the other handle is unchanged.
- Given a free shape stretched from its stored box to 200 × 100, when it is drawn, then its handles are stretched like its points (they live in the stored points' frame).
- Given a free shape read back from the page, when its handles are not valid (an unknown mode, a number not finite, a switch not boolean), then it is refused (`isValidFreeShape`).

## Tasks

One task = one commit, referenced as `EN-024.Tn`.

- [x] T1 — Types and validity of handles, defaults on creation and conversion, handles stretched with the points
- [x] T2 — Setting one handle by the mode; the radius kept beside the handles (done with T1: one commit)

## Decisions during the story

| Decision                                                                                                                   | Reason                                                                                                                                            |
| -------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Handles`, `CurveCorner` and `mirroredHandle` in `geometry/handles/`; the default, validity and typing in `model/handles/` | the contour (EN-025) needs the type in the geometry layer; the business rules belong to the model (ADR-0043: the same domain name in both layers) |
| a free shape's corners are `CurveCorner`s; rectangles and polygons keep plain `Corner`s                                    | only a free shape has handles; a conversion gives them off (`curveCornerOf`)                                                                      |
| `turnedCorners` keeps the handles as they are                                                                              | turning them is part of the cubic sides (EN-025)                                                                                                  |
| the page's model text shows the handles                                                                                    | it shows the model as stored                                                                                                                      |
