# 0015 — Sources of F10 in E14: an ellipse from four cubic sides

Spike SP-013, 2026-10-14. Claude Code's own research; every source below was read in the local library. Formulas in `DERIV-round-shape-curve`; decision in ADR-0048. F10 as built in E01 is in [note 0010](./0010-f10-round-shape.md).

## What was found

| Question | Answer | Source |
| --- | --- | --- |
| A quarter circle as a cubic | handles `4/3 · tan(θ/4)` of the radius, 0.55228 for a quarter; error about 0.00027 of the radius | `REF-POMAX-BEZIER` |
| A whole circle | four cubics, handles `4(√2 − 1)/3`, radial error under one part in a thousand | `REF-WIKI-BEZIER` |
| An ellipse | the circle's control points stretched: an affine map of a curve is that of its control points | `REF-WIKI-BEZIER` |
| What SVG does | `A rx ry …` draws an elliptical arc exactly; the `ellipse` element is four elliptical arcs | `REF-SVG2-PATHS`, `REF-SVG2-SHAPES` |
| What drawing tools do | Inkscape keeps an ellipse a shape until "Object to Path"; Figma's handle modes are those of the converted points (what Figma itself gives was not read) | `REF-INKSCAPE-OBJECT-TO-PATH`, `REF-FIGMA-HANDLE-MIRRORING` |

## Decision

Four cubic sides (ADR-0048): one curve kind, a conversion that moves nothing, a stretch that keeps an ellipse. The gap, 0.00027 of the half axis, is under a hundredth of a unit up to a diameter of about 70, and 0.014 at 100.

## Open

Nothing for F10. Strokes of curves stay with E04 (ADR-0046).
