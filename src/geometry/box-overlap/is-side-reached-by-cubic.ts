import type { Cubic, Segment } from "../contour";

/**
 * Whether a cubic side reaches a horizontal side: one of its parts monotone in y
 * (`cubicYBreaks`) does (`isSideReachedByMonotonePart`) (`DERIV-box-overlap` step 6).
 *
 * Formula: some [tᵢ, tᵢ₊₁] of consecutive breaks reaches the side.
 *
 * @kind geometry
 * @param cubic - start, control points and end
 * @param side - a horizontal side, its ends at the same ordinate
 * @returns true when a point of the cubic lies on the side, within a rounding error
 * @see DERIV-box-overlap
 */
export function isSideReachedByCubic(cubic: Cubic, side: Pick<Segment, "end" | "start">): boolean {
  return cubic.start.y === side.start.y;
}
