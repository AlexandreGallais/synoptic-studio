import type { Vector } from "./vector";

/**
 * Dot product of two vectors.
 *
 * Formula: a · b = aₓ bₓ + a_y b_y = |a| |b| cos θ.
 *
 * @kind math
 * @param a - first vector
 * @param b - second vector
 * @returns the scalar product, zero for perpendicular vectors
 * @see REF-MATHWORLD-DOT
 */
export function dot(a: Vector, b: Vector): number {
  return a.x * b.x + a.y * b.y;
}
