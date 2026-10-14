import type { Quadratic } from "./quadratic";

/** The 4 of the discriminant b² − 4ac. */
const DISCRIMINANT_FACTOR = 4;

/**
 * Discriminant of a quadratic: positive for two real roots, zero for one, negative for none.
 *
 * Formula: Δ = b² − 4ac.
 *
 * @kind math
 * @param quadratic - its coefficients
 * @returns b² − 4ac
 * @see REF-WIKI-QUADRATIC
 */
export function discriminant(quadratic: Quadratic): number {
  const { a, b, c } = quadratic;

  return b * b - DISCRIMINANT_FACTOR * a * c;
}
