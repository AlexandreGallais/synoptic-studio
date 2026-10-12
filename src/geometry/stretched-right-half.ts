import { HALF_SIDE } from "./half-side";
import { stretchedSideDirections } from "./stretched-side-directions";

import type { Point } from "../math";

/**
 * Right half of a stretched polygon of sides 1, in the mathematical frame (y up), its base on
 * `y = 0` and its axis on `x = 0`: from the right end of the half base `(1/2, 0)`, each vertex the
 * sum of the sides `(cos φⱼ, sin φⱼ)` before it, up to the apex (odd `n`) or the right end of the
 * top side (even `n`) (`DERIV-stretched-polygon` step 1). The sums run in the same order for every
 * vertex: the same values as adding one side at a time.
 *
 * @kind geometry
 * @param corners - number of corners `n`, an integer from 3 to 12
 * @param scale - vertical scale `λ > 0`
 * @returns the `⌈n/2⌉` vertices from the base upwards; `NaN` coordinates when no polygon closes
 * @see DERIV-stretched-polygon
 */
export function stretchedRightHalf(corners: number, scale: number): readonly Point[] {
  const directions = stretchedSideDirections(corners, scale);

  return Array.from({ length: directions.length + 1 }, (empty, count) => {
    const sides = directions.slice(0, count);

    return {
      x: sides.reduce((sum, direction) => sum + Math.cos(direction), HALF_SIDE),
      y: sides.reduce((sum, direction) => sum + Math.sin(direction), 0),
    };
  });
}
