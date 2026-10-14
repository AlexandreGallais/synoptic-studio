import { cubicBernstein } from "./cubic-bernstein";

import type { Point } from "../../math";
import type { Cubic } from "../contour";

/**
 * Point of a cubic at t, from its Bernstein form (`DERIV-cubic-sides` step 1).
 *
 * Formula: B(t) = (1−t)³P₀ + 3(1−t)²t P₁ + 3(1−t)t² P₂ + t³P₃.
 *
 * @kind geometry
 * @param cubic - start, control points and end
 * @param t - from 0 (start) to 1 (end)
 * @returns the point of the curve
 * @see REF-WIKI-BEZIER
 */
export function cubicPoint(cubic: Cubic, t: number): Point {
  const [w0, w1, w2, w3] = cubicBernstein(t);
  const { end, first, second, start } = cubic;

  return {
    x: w0 * start.x + w1 * first.x + w2 * second.x + w3 * end.x,
    y: w0 * start.y + w1 * first.y + w2 * second.y + w3 * end.y,
  };
}
