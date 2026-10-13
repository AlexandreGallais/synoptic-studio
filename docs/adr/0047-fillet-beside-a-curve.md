# ADR-0047 — A corner fillet beside a cubic side

- Status: accepted
- Date: 2026-10-13
- Complements: ADR-0046 (its consequence "a corner next to a curved side is drawn sharp" is replaced), ADR-0007

## Context

ADR-0046 let cubic sides into symbols and drew sharp a corner in radius mode next to a curved side, until a rule was chosen. At VAL-011 the Product Owner asked that such a corner keep its rounding: on a rectangle 120 × 80 with radius 20, giving handles to point 3 must not make points 2 and 4 sharp. SP-012 found an exact construction (`DERIV-fillet-beside-curve`).

## Decision

1. A corner in radius mode beside one or two cubic sides is filleted by the circular arc of its radius tangent to both sides; the cubic is cut at the tangent point by de Casteljau's split, the piece kept exactly on the curve.
2. A radius too large is reduced to the largest one that fits, never drawn sharp:
   - along a cubic, the fillet may use the whole curve, its other end being sharp (a point with handles on): as one rounded corner may use a whole straight side (ADR-0007);
   - where the curve bends more tightly than the radius, the radius becomes the largest circle still tangent without crossing the curve, as Astute Graphics' corners ("made as large as possible", `REF-ASTUTE-CORNERS`);
   - the corner keeps the smaller of its sides' limits (ADR-0007 step 3).
3. No new primitive: the arc is a fillet arc, the rest a cubic (ADR-0001 amended by ADR-0046).

Chosen by the Product Owner from the agent's two recommendations, for a behaviour that lets the designer do much while always keeping a clean rounding, never a sudden sharp corner (2026-10-13).

## Consequences

- A straight side written as a cubic draws the same fillet as today's segment.
- The point is found numerically, where a straight side needs a formula: coarse tables of 32 samples, then bisections of 40 halvings; the centre runs along the curve and is measured from the straight side, so that only a corner between two cubics nests a search of the foot. A corner beside a curve costs about 10 ms, one between two cubics with a radius to reduce up to about 200 ms (US-027, measured).
- A spike where the curve leaves back along the straight side gets no arc.
