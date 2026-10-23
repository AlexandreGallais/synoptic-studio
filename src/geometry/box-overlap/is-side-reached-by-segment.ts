import { isBetween, pointBetween } from "../../math";

import type { Segment } from "../contour";

/**
 * Whether a segment reaches a horizontal side: it meets the side's line where its ordinates
 * bracket it, at the fraction of its length the level gives, and that point's abscissa lies
 * between the side's ends; a segment along a horizontal line is left out, its start and the
 * vertical sides telling (`DERIV-box-overlap` steps 5 and 8).
 *
 * Formula: S_y ≠ E_y, c between S_y and E_y, x(t) between a and b with t = (c − S_y) / (E_y − S_y).
 *
 * @kind geometry
 * @param segment - its start and its end
 * @param side - a horizontal side, its ends at the same ordinate
 * @returns true when a point of the segment lies on the side
 * @see DERIV-box-overlap
 */
export function isSideReachedBySegment(
  segment: Pick<Segment, "end" | "start">,
  side: Pick<Segment, "end" | "start">,
): boolean {
  const { end, start } = segment;
  const level = side.start.y;

  // A flat segment would also fail below, its 0 / 0 a NaN outside every interval: the test says it,
  // and a mutation dropping it survives.
  if (start.y === end.y || !isBetween(level, start.y, end.y)) {
    return false;
  }

  const reached = pointBetween(start, end, (level - start.y) / (end.y - start.y));

  return isBetween(reached.x, side.start.x, side.end.x);
}
