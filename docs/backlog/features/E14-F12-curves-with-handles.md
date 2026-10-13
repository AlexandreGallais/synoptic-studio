---
id: F12
epic: E14
title: Points on a curve with handles
status: in-progress
---

# E14 · F12 — Points on a curve with handles

**Benefit hypothesis**: the designer bends a side of a free shape into a curve by typing the two handles of a point, as in Figma, instead of being limited to rounded corners.

**Product Owner's idea (2026-10-12, recalled at VAL-010 on 2026-10-13)**: a free shape gets points on the curve, each with two handles, as in Figma. Figma offers three handle modes — no mirroring, mirrored angle, mirrored angle and length — and keeps the corner radius beside them; the Product Owner wants the same. A round shape converted to a free shape gets 4 such points, at its top, right, bottom and left (F10).

Already in the scope of E14 (handle modes, Bézier curves in symbols); this feature makes it a deliverable of its own. Refined with the Product Owner before any story: how handles are typed, the modes, how a corner radius and handles live on one point, and the ADR that supersedes ADR-0001 (segments and circular arcs only).

## Refinement

Started with the Product Owner on 2026-10-13:

| Question                                 | Decision                                                                                                                                                                                                                                                                                                            |
| ---------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Order in E14                             | F12 before F10: the round shape converted to a free shape needs points with handles                                                                                                                                                                                                                                 |
| How a handle is typed                    | researched in the spike: x and y of its end (the Product Owner's choice, the simplest), an angle and a length relative to its point (the same values give the same curve anywhere), pasted Bézier data, other ways used by tools; every method found is built, each tried in the playground at the end of F12 (Q32) |
| A corner radius and handles on one point | the radius is kept when handles are added and comes back when they are removed; a clear switch between a radius mode and a handles mode; its final layout at E14's UX pass                                                                                                                                          |
| Default handle mode                      | mirrored angle and length, as Figma's smooth point; mirrored angle and no mirroring chosen per point                                                                                                                                                                                                                |

## Stories

| ID                                                      | Type       | Title                                                               | Status |
| ------------------------------------------------------- | ---------- | ------------------------------------------------------------------- | ------ |
| [SP-011](../stories/E14-F12-SP-011-research-f12.md)     | Spike      | Research for F12: curves in symbols, ways to type a handle, strokes | ready  |
| [CHK-011](../stories/E14-F12-CHK-011-checkpoint-f12.md) | Checkpoint | Checkpoint in the middle of F12                                     | draft  |
| [AUD-011](../stories/E14-F12-AUD-011-audit-f12.md)      | Audit      | Audit F12                                                           | draft  |
| [VAL-011](../stories/E14-F12-VAL-011-validate-f12.md)   | Validation | Validate F12                                                        | draft  |

The stories between SP-011 and the checkpoint, and after it, are written by SP-011; the run sets them ready (ADR-0045).

## Acceptance criteria

To write after SP-011.
