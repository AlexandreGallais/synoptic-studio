import { cyclicItem } from "./cyclic-item";
import { edgeFactor } from "./edge-factor";

/**
 * Factor of every edge of a cyclic contour: edge `i` carries the setbacks of corners `i` and `i + 1`.
 *
 * Formula: fᵢ = edgeFactor(Lᵢ, sᵢ + sᵢ₊₁).
 *
 * @kind geometry
 * @param lengths - length of each edge, in drawing order
 * @param setbacks - setback requested at each corner, in drawing order
 * @returns one factor per edge, in [0, 1]
 * @see DERIV-local-radius-clamp
 */
export function edgeFactors(
  lengths: readonly number[],
  setbacks: readonly number[],
): readonly number[] {
  return lengths.map((length, index) =>
    edgeFactor(length, cyclicItem(setbacks, index, 0) + cyclicItem(setbacks, index + 1, 0)),
  );
}
