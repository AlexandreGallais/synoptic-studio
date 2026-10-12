import { cyclicItem } from "./cyclic-item";

/**
 * Factor of every vertex of a cyclic contour: the smaller factor of its two edges.
 *
 * Formula: gᵢ = min(fᵢ₋₁, fᵢ), edge `i − 1` arriving at vertex `i` and edge `i` leaving it.
 *
 * @kind geometry
 * @param factors - factor of each edge, in drawing order
 * @returns one factor per vertex, in [0, 1]
 * @see DERIV-local-radius-clamp
 */
export function vertexFactors(factors: readonly number[]): readonly number[] {
  return factors.map((factor, index) => Math.min(cyclicItem(factors, index - 1, factor), factor));
}
