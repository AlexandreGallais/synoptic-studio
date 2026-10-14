import { CUBIC_DEGREE } from "./cubic-degree";

/**
 * The four Bernstein basis polynomials of degree 3 at t: the weights of a cubic's control points
 * in its point B(t). They are ≥ 0 on [0, 1] and add up to 1.
 *
 * Formula: ((1 − t)³, 3(1 − t)² t, 3(1 − t) t², t³).
 *
 * @kind math
 * @param t - the parameter, from 0 to 1
 * @returns the weights of P₀, P₁, P₂ and P₃
 * @see REF-WIKI-BEZIER
 */
export function cubicBernstein(t: number): readonly [number, number, number, number] {
  const u = 1 - t;

  return [u * u * u, CUBIC_DEGREE * u * u * t, CUBIC_DEGREE * u * t * t, t * t * t];
}
