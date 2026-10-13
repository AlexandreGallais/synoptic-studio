import { CUBIC_DEGREE } from "../placement";

import type { Vector } from "../../math";
import type { Cubic } from "../contour";

/**
 * The derivative of a cubic at a parameter, its velocity: the tangent's direction
 * (`REF-WIKI-BEZIER`; `REF-POMAX-BEZIER`, "Tangents and normals").
 *
 * Formula: B′(t) = 3(1−t)²(P₁−P₀) + 6(1−t)t(P₂−P₁) + 3t²(P₃−P₂).
 *
 * @kind geometry
 * @param cubic - start, control points and end
 * @param t - the parameter, from 0 to 1
 * @returns the derivative at t
 * @see REF-WIKI-BEZIER
 */
export function cubicVelocity(cubic: Cubic, t: number): Vector {
  const u = 1 - t;
  const [w0, w1, w2] = [CUBIC_DEGREE * u * u, 2 * CUBIC_DEGREE * u * t, CUBIC_DEGREE * t * t];
  const { end, first, second, start } = cubic;

  return {
    x: w0 * (first.x - start.x) + w1 * (second.x - first.x) + w2 * (end.x - second.x),
    y: w0 * (first.y - start.y) + w1 * (second.y - first.y) + w2 * (end.y - second.y),
  };
}
