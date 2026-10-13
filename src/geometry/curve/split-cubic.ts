import { pointBetween } from "../../math";

import type { Cubic } from "../contour";

/**
 * A cubic split at a parameter into two cubics that lie exactly on it (`REF-POMAX-BEZIER`,
 * "Splitting curves"; de Casteljau, `REF-WIKI-DE-CASTELJAU`; `DERIV-fillet-beside-curve` step 9).
 * At ½ it halves the curve (`DERIV-cubic-sides` step 4).
 *
 * Formula: P₀₁, P₁₂, P₂₃ at t along the control polygon, P₀₁₂, P₁₂₃ at t along those, M at t
 * along those two; pieces P₀ P₀₁ P₀₁₂ M and M P₁₂₃ P₂₃ P₃.
 *
 * @kind geometry
 * @param cubic - start, control points and end
 * @param t - where to split, from 0 to 1
 * @returns the piece before t, then the piece after, joined at the curve's point at t
 * @see REF-WIKI-DE-CASTELJAU
 */
export function splitCubic(cubic: Cubic, t: number): readonly [Cubic, Cubic] {
  const p01 = pointBetween(cubic.start, cubic.first, t);
  const p12 = pointBetween(cubic.first, cubic.second, t);
  const p23 = pointBetween(cubic.second, cubic.end, t);
  const p012 = pointBetween(p01, p12, t);
  const p123 = pointBetween(p12, p23, t);
  const middle = pointBetween(p012, p123, t);

  return [
    { end: middle, first: p01, kind: "cubic", second: p012, start: cubic.start },
    { end: cubic.end, first: p123, kind: "cubic", second: p23, start: middle },
  ];
}
