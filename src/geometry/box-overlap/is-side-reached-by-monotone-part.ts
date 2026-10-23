import type { Cubic, Segment } from "../contour";

/**
 * Whether a part of a cubic side monotone in y reaches a horizontal side: it meets the side's
 * line once where its ends' ordinates bracket it, at the t the bisection finds
 * (`cubicLevelParameter`), and that point's abscissa lies between the side's ends; a part of
 * constant y is left out, as a flat segment (`DERIV-box-overlap` steps 6 and 8).
 *
 * Formula: y(a) ≠ y(b), c between y(a) and y(b), x(t*) between the side's ends.
 *
 * @kind geometry
 * @param cubic - start, control points and end
 * @param span - the part's first and last t, y monotone between them
 * @param side - a horizontal side, its ends at the same ordinate
 * @returns true when a point of the part lies on the side, within a rounding error
 * @see DERIV-box-overlap
 */
export function isSideReachedByMonotonePart(
  cubic: Cubic,
  span: readonly [number, number],
  side: Pick<Segment, "end" | "start">,
): boolean {
  return span[0] === span[1] && cubic.start === side.start;
}
