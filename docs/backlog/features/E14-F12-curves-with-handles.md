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

| ID                                                                  | Type       | Title                                                               | Status      |
| ------------------------------------------------------------------- | ---------- | ------------------------------------------------------------------- | ----------- |
| [SP-011](../stories/E14-F12-SP-011-research-f12.md)                 | Spike      | Research for F12: curves in symbols, ways to type a handle, strokes | done        |
| [EN-024](../stories/E14-F12-EN-024-handles-model.md)                | Enabler    | A point's handles in the free shape model                           | done        |
| [EN-025](../stories/E14-F12-EN-025-cubic-sides.md)                  | Enabler    | Cubic sides in contours, path data and the straight box             | done        |
| [US-022](../stories/E14-F12-US-022-handles-in-points-mode.md)       | User story | Switch a point to handles and type them as x and y                  | done        |
| [US-023](../stories/E14-F12-US-023-handles-relative-to-point.md)    | User story | Type a handle as dx and dy from its point                           | done        |
| [CHK-011](../stories/E14-F12-CHK-011-checkpoint-f12.md)             | Checkpoint | Checkpoint in the middle of F12                                     | done        |
| [US-024](../stories/E14-F12-US-024-handles-as-angle-and-length.md)  | User story | Type a handle as an angle and a length                              | done        |
| [US-025](../stories/E14-F12-US-025-handles-as-share-or-data.md)     | User story | Type a side's handles as shares of the side, or paste curve data    | done        |
| [US-026](../stories/E14-F12-US-026-handles-from-a-tension.md)       | User story | Smooth a point by a tension, its handles computed                   | done        |
| [AUD-011](../stories/E14-F12-AUD-011-audit-f12.md)                  | Audit      | Audit F12                                                           | done        |
| [EN-026](../stories/E14-F12-EN-026-ways-kept-at-validation.md)      | Enabler    | Keep only the ways and modes chosen at the validation               | done        |
| [SP-012](../stories/E14-F12-SP-012-research-fillet-beside-curve.md) | Spike      | Research: a corner radius beside a curved side                      | done        |
| [US-027](../stories/E14-F12-US-027-radius-beside-curve.md)          | User story | A corner beside a curved side keeps its radius                      | ready       |
| [US-028](../stories/E14-F12-US-028-handles-on-a-polygon.md)         | User story | Give handles to a point of a regular polygon                        | done        |
| [AUD-012](../stories/E14-F12-AUD-012-audit-after-validation.md)     | Audit      | Audit the changes asked at the validation                           | draft       |
| [VAL-011](../stories/E14-F12-VAL-011-validate-f12.md)               | Validation | Validate F12                                                        | in-progress |

Written by SP-011 from [research note 0013](../../research/notes/0013-f12-curves-with-handles.md); set ready by the run (ADR-0045).

## Acceptance criteria

1. A point of a free shape switches between its corner radius and its handles; the radius comes back as it was.
2. Handles follow Figma's three modes, angle and length by default.
3. A side with a handle at either end is drawn as a cubic Bézier, its straight box exact (ADR-0046).
4. A handle is typed as x and y on the page, or as an angle and a length from its side — the two methods kept at VAL-011 out of the seven of research note 0013 (Q32).
5. Every shape of E01 draws as before.
6. A shape's box is the box of its curve (Product Owner, 2026-10-13).
7. A corner in radius mode beside a curved side keeps its rounding (Product Owner, VAL-011, 2026-10-13).
8. A point of a regular polygon switches to handles, the polygon converted first without moving (Product Owner, VAL-011, 2026-10-13).

## Feature plan

| Acceptance criterion | Realized by            | Verified by         |
| -------------------- | ---------------------- | ------------------- |
| F12.AC1              | EN-024, US-022         | AUD-011             |
| F12.AC2              | EN-024, US-022         | AUD-011             |
| F12.AC3              | EN-025                 | AUD-011             |
| F12.AC4              | US-022, US-024, EN-026 | AUD-011, AUD-012    |
| F12.AC5              | every story            | `npm run check:all` |
| F12.AC6              | EN-025                 | AUD-011             |
| F12.AC7              | SP-012, US-027         | AUD-012             |
| F12.AC8              | US-028                 | AUD-012             |
