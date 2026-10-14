import { CUBIC_DEGREE } from "./cubic-degree";

/**
 * Weights of a cubic's three control-point differences in its derivative at t: 3 times the
 * Bernstein polynomials of degree 2.
 *
 * Formula: B′(t) = 3(1 − t)²(P₁ − P₀) + 6(1 − t)t(P₂ − P₁) + 3t²(P₃ − P₂).
 *
 * @kind math
 * @param t - the parameter, from 0 to 1
 * @returns the weights of P₁ − P₀, P₂ − P₁ and P₃ − P₂
 * @see REF-WIKI-BEZIER
 */
export function cubicDerivativeWeights(t: number): readonly [number, number, number] {
  const u = 1 - t;

  return [CUBIC_DEGREE * u * u, 2 * CUBIC_DEGREE * u * t, CUBIC_DEGREE * t * t];
}
