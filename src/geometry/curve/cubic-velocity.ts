import { subtract } from "../../math";
import { cubicDerivativeWeights } from "../placement";

import type { Vector } from "../../math";
import type { Cubic } from "../contour";

/**
 * The derivative of a cubic at a parameter, its velocity: the tangent's direction
 * (`REF-WIKI-BEZIER`; `REF-POMAX-BEZIER`, "Tangents and normals"). `derivativeCoefficients` gives the same
 * derivative in power form for the box; both are kept, as they round differently.
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
  const [w0, w1, w2] = cubicDerivativeWeights(t);
  const { end, first, second, start } = cubic;
  const [d0, d1, d2] = [subtract(first, start), subtract(second, first), subtract(end, second)];

  return {
    x: w0 * d0.x + w1 * d1.x + w2 * d2.x,
    y: w0 * d0.y + w1 * d1.y + w2 * d2.y,
  };
}
