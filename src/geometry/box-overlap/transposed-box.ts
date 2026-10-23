import type { BoundingBox } from "../placement";

/**
 * An upright box with its axes swapped: its interval on x becomes its interval on y, and the
 * other way round (`DERIV-box-overlap` step 8).
 *
 * Formula: [x₀, x₁] × [y₀, y₁] ↦ [y₀, y₁] × [x₀, x₁].
 *
 * @kind geometry
 * @param box - its extreme coordinates
 * @returns the box of the swapped points
 * @see DERIV-box-overlap
 */
export function transposedBox(box: BoundingBox): BoundingBox {
  return box;
}
