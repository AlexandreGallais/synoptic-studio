---
id: SP-007
epic: E01
feature: F04
title: Research for F04
status: ready
points: 2
---

# E01 · F04 · SP-007 — Research for F04

Find, read and record the sources of every story of F04 before its implementation (ADR-0020).

## Acceptance criteria

- Given each story of F04, when the spike ends, then every formula and behavior it needs has a verified `REF-*` or a `DERIV-*`.
- Given the sources found, when the spike ends, then the stories cite them, and a research note records what was found and what remains open.

## Questions

- Turning a point about a center, clockwise on screen in the SVG frame (y down): the formula and a source (SVG transforms, `rotate(a, cx, cy)`); exact values at 0, 90, 180, 270 (cos and sin of 90° are not exact in floating point).
- An angle kept from 0 to 359: the remainder of a negative number in ECMAScript (`%` keeps the sign), stated in TSDoc (ADR-0025).
- The straight box of a contour of segments and circular arcs: where an arc reaches its extreme x and y (the axis directions it sweeps); how browsers define `getBBox`.
- The model: what the position stores (the straight box's top-left, or a translation from which it is derived), how it stays without loss when the shape turns back and forth; an ADR (every shape gets an angle and a position, ADR-0003 and ADR-0034).
- Editing a turned shape: which point stays fixed when the width changes (Figma's behavior as a functional reference), a point typed in the SVG's coordinates turned back into the shape's box.

## Tasks

One task = one commit, referenced as `SP-007.Tn`.

- [ ] T1 — Sources, derivations if no source states them, ADR (2 h)
- [ ] T2 — Research note, stories updated (1 h)
