# Note 0004 — Sources of F02, regular polygon with corner radius

**Date**: 2026-10-09 — research spike SP-002, done by Claude Code (WebFetch, WebSearch), during the autonomous run of F02.
**Resulting documents**: `DERIV-regular-polygon-fit` completed (steps 2, 5, 6); six references read or added; request 0004 (optional); EN-008, US-005, US-006 updated.

## Per story

| Story  | Found                                                                                                                                 | Source                                                                                                                                    |
| ------ | ------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| EN-008 | the nth roots of unity are the vertices of a regular n-gon on the unit circle; a vertex at angle `α` is `(cos α, sin α)`              | `REF-MATHWORLD-ROOT-OF-UNITY`, `REF-MATHWORLD-EULER-FORMULA`, `REF-OPENSTAX-UNIT-CIRCLE`                                                  |
| EN-008 | flat base, decreasing angles for clockwise on screen, starting vertex by its index `⌊n/2⌋` (no float comparison)                      | derived: `DERIV-regular-polygon-fit` steps 1–2 (Q11, Q18)                                                                                 |
| US-005 | unit bounding box, uniform scale, centering and y flip; a size of 0 collapses to the center                                           | `DERIV-regular-polygon-fit` steps 3–5                                                                                                     |
| US-006 | at the maximal radius each fillet is an arc of the incircle, radius `½ a cot(π/n) = R cos(π/n)`                                       | `DERIV-regular-polygon-fit` step 6, `REF-MATHWORLD-REGULAR-POLYGON` (3), (4), `REF-MATHWORLD-INCIRCLE`, `REF-EUCLID-I47`, `REF-EUCLID-I8` |
| US-006 | the F01 clamp and fillet need no change: they already work on any point, the polygon's vertices are derived and fractional (ADR-0003) | `src/geometry/` (read); `Corner.point` TSDoc and `shapes.md` §1 still say "integer coordinates": to correct                               |
| US-007 | Inkscape's tool: a polygon mode with a "Corners" field; its rounding uses curves and is not kept (ADR-0001)                           | `REF-INKSCAPE-POLYGON` (now verified)                                                                                                     |

## Q19 — largest number of corners

What could be read:

- an ISA page names a "diamond" symbol of ISA-5.1, defined in 2009 for safety instrumented systems and drawn inside a box (`REF-ISA-INTECH-2019`); its geometry is not stated — reading it as a square standing on a vertex is an assumption;
- search summaries, not readings (pages refused): circle, square, circle in a square and hexagon (computer function) in ISA-5.1, triangles in valve symbols;
- the symbol standards themselves (ISA-5.1, ISO 10628-2, ISO 14617, IEC 60617) and ISA-101 are paid: not read, not cited — written before research 0004, which then read the public IEC 60617 symbol pages (below).

What it costs: one segment and one arc per corner in the path data, about 50 to 60 characters each with 5 decimals (measured by the auditor on the format of `src/io/`); 100 corners make about 6 kB. The limit is a matter of usefulness, not of performance.

Findings for the Product Owner:

- no source read or summarized shows a symbol polygon with more than 6 sides; 8 (octagon) is a margin;
- the diamond is needed; if it is a square standing on a vertex (assumption), F02 always draws a flat base, so the diamond comes with rotation (F04), or with an orientation option not planned;
- request 0004 holds the precise question if a normative confirmation is wanted.

## Research 0004 (claude.ai, Research mode, run by the Product Owner, 2026-10-09)

Summary in our own words:

- Editions: ISA-5.1-2009 is historical, replaced by ISA-5.1-2022 then ANSI/ISA-5.1-2024; request 0004 named a "2009 (R2022)" edition that does not exist. ISO 14617-2:2025 replaces parts 2 to 15 of 2002 and drops the measurement and control symbols.
- Polygons found: in ISA-5.1-2009 (through secondary drawings) a diamond in a square, a square with a circle, a flat-based hexagon (computer function), small diamonds (purge, reset, interlock); in the IEC 60617 database (read directly) equilateral triangles, squares, two hexagons — one standing on a vertex — and an octagon (connection box) whose regularity is not shown.
- The diamond is a square standing on a vertex in the secondary drawings, i.e. a square turned by 45° if its vertices are at the middles of the box's sides — not confirmed by the dimension tables.
- None with more than 8 sides; the IEC survey stopped at S00162 (anti-robot check): not exhaustive.
- Alarm displays (ISA-18.2, ISA-101): no normative shape found; priority shapes are vendor practice.
- Read directly: IEC symbol pages, ANSI and ISO previews (cover, contents, informative parts); not read: the standards' tables. Unauthorized full copies were found and not used.

Claude Code tried to re-read the IEC pages and the ANSI preview: HTTP 403. `REF-IEC-60617` is recorded as read by the research, with that qualifier.

## Decision

Q19 settled by the Product Owner on 2026-10-09: **3 ≤ n ≤ 12**. Symbols use up to 8 sides; beyond 12 a polygon differs from its circle by less than 3.5 % (`1 − cos(π/12)`), and the circle is already a rounded square; 12 is unchanged by a quarter turn (ADR-0008); raising the limit later invalidates no symbol, lowering it would. Other orientations (diamond 45°, IEC hexagon 30°) come from rotation, F04.

## Remaining open

- None for F02.
