import type { Arc } from "../contour";

/**
 * Abscissas of a circle's points on a horizontal line: the points at its radius from its centre
 * whose ordinate is the line's, none when the line passes beyond the circle
 * (`DERIV-box-overlap` step 7).
 *
 * Formula: h = c − C_y; Cₓ − √(r² − h²) and Cₓ + √(r² − h²) when |h| ≤ r, none otherwise.
 *
 * @kind geometry
 * @param circle - its centre and its radius
 * @param level - the line's ordinate
 * @returns the two abscissas, left first, equal where the line touches; none beyond the circle
 * @see REF-MATHWORLD-CIRCLE
 */
export function circleLevelAbscissas(
  circle: Pick<Arc, "center" | "radius">,
  level: number,
): readonly number[] {
  const height = level - circle.center.y;
  const squared = circle.radius ** 2 - height ** 2;

  if (squared < 0) {
    return [];
  }

  const half = Math.sqrt(squared);

  return [circle.center.x - half, circle.center.x + half];
}
