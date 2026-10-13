import { midpoint } from "../../math";

import type { Cubic } from "../contour";

/**
 * A cubic split at its middle into two cubics that lie exactly on it (de Casteljau at t = ½,
 * `DERIV-cubic-sides` step 4).
 *
 * Formula: P₀₁, P₁₂, P₂₃ midpoints of the control polygon, P₀₁₂, P₁₂₃ of those, M of those two;
 * halves P₀ P₀₁ P₀₁₂ M and M P₁₂₃ P₂₃ P₃.
 *
 * @kind geometry
 * @param cubic - start, control points and end
 * @returns the first half, then the second, joined at the curve's middle
 * @see REF-POMAX-BEZIER
 */
export function halvedCubic(cubic: Cubic): readonly [Cubic, Cubic] {
  const p01 = midpoint(cubic.start, cubic.first);
  const p12 = midpoint(cubic.first, cubic.second);
  const p23 = midpoint(cubic.second, cubic.end);
  const p012 = midpoint(p01, p12);
  const p123 = midpoint(p12, p23);
  const middle = midpoint(p012, p123);

  return [
    { end: middle, first: p01, kind: "cubic", second: p012, start: cubic.start },
    { end: cubic.end, first: p123, kind: "cubic", second: p23, start: middle },
  ];
}
