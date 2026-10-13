import { cubicPoint } from "./cubic-point";
import { derivativeCoefficients } from "./derivative-coefficients";
import { quadraticRoots } from "./quadratic-roots";

import type { Point } from "../../math";
import type { Cubic } from "../contour";

/**
 * Points of a cubic side where its x or its y turns between its ends: with its ends, they give its
 * straight box (`DERIV-cubic-sides` step 3, `REF-POMAX-BEZIER`).
 *
 * Formula: B(t) at each root of B′ on x or on y with 0 < t < 1; a degenerate root (infinite, NaN)
 * fails the test.
 *
 * @kind geometry
 * @param cubic - start, control points and end
 * @returns the curve's points at those values of t, ends not included
 * @see DERIV-cubic-sides
 */
export function cubicExtremes(cubic: Cubic): readonly Point[] {
  const { end, first, second, start } = cubic;
  const roots = [
    ...quadraticRoots(derivativeCoefficients([start.x, first.x, second.x, end.x])),
    ...quadraticRoots(derivativeCoefficients([start.y, first.y, second.y, end.y])),
  ];

  return roots.filter((t) => t > 0 && t < 1).map((t) => cubicPoint(cubic, t));
}
