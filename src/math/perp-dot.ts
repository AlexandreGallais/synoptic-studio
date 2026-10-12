import type { Vector } from "./vector";

/**
 * Perp dot product of two vectors (the 2D cross product).
 *
 * Formula: a⊥ · b = aₓ b_y − a_y bₓ = |a| |b| sin θ, θ measured from `a` to `b`.
 *
 * @kind math
 * @param a - first vector
 * @param b - second vector
 * @returns the signed area of the parallelogram (a, b), zero for parallel vectors
 * @see REF-MATHWORLD-PERP-DOT
 */
export function perpDot(a: Vector, b: Vector): number {
  return a.x * b.y - a.y * b.x;
}
