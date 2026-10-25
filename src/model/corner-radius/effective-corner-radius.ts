import { EPSILON } from "../../math";

/**
 * Corner radius actually drawn on a shape whose corners all request the same radius: the
 * requested radius after clamping (ADR-0007), when every corner draws the same one.
 *
 * Derived at every evaluation, never stored (Q8). The corners of a rectangle or of a regular
 * polygon are clamped alike; those of a stretched polygon (F06) or of a stretched free shape have
 * different angles and may be clamped differently: there is then no one effective radius, and
 * each is shown per vertex. Radii closer than `EPSILON` are the same: the corners of a regular polygon, clamped alike in
 * exact arithmetic, differ by at most about 1e-10 in doubles up to sizes and radii of 100 000
 * (review of US-021), ten times below. No corner gives `+∞`.
 *
 * @kind domain
 * @param radii - the radii each corner is drawn with (`effectiveRadii`, or `drawnRadii` with
 * handles), the same radius requested on each
 * @returns the effective radius, <= the requested one; `undefined` when the corners draw
 * different radii (an absent value, an option type in other languages, ADR-0025)
 * @see docs/domain/shapes.md#radius-clamping
 */
export function effectiveCornerRadius(radii: readonly number[]): number | undefined {
  const smallest = Math.min(...radii);

  return radii.every((radius) => radius - smallest < EPSILON) ? smallest : undefined;
}
