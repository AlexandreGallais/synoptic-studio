import { cubicSide, cyclicItem, halvedCubic, isCurvedSide } from "../../geometry";
import { midpoint, subtract } from "../../math";
import { NO_HANDLES } from "../handles";

import type { CurveCorner } from "../../geometry";
import type { Point } from "../../math";

/** Corner read when the side asked for does not exist: none is, the shape has 3 points or more. */
const NO_CORNER: CurveCorner = { handles: NO_HANDLES, point: { x: 0, y: 0 }, radius: 0 };

/**
 * A corner whose handle after or before now reaches a given control point.
 *
 * @kind domain
 * @param corner - corner to change
 * @param end - which handle: towards the next point (`after`) or the previous (`before`)
 * @param control - the control point it now reaches
 * @returns the corner, that handle set to the control point minus its point
 * @see DERIV-cubic-sides
 */
function withHandleTo(corner: CurveCorner, end: "after" | "before", control: Point): CurveCorner {
  return { ...corner, handles: { ...corner.handles, [end]: subtract(control, corner.point) } };
}

/**
 * Corners with a curved side split in half: the new point, in handles mode (angle and length),
 * at the curve's middle, its handles and its neighbours' from the two halves (`DERIV-cubic-sides`
 * step 4), so that the outline does not move.
 *
 * @kind domain
 * @param corners - corners as drawn
 * @param edge - number of the curved side
 * @returns the corners with the new point after the side's first point
 * @see DERIV-cubic-sides
 */
function cornersWithCurveSplit(
  corners: readonly CurveCorner[],
  edge: number,
): readonly CurveCorner[] {
  const start = cyclicItem(corners, edge, NO_CORNER);
  const [first, second] = halvedCubic(cubicSide(start, cyclicItem(corners, edge + 1, start)));
  const opened = { handles: { ...NO_HANDLES, isOn: true }, point: first.end, radius: 0 };
  const middle = withHandleTo(withHandleTo(opened, "before", first.second), "after", second.first);

  return corners.flatMap((corner, index) => {
    if (index === edge) {
      return [withHandleTo(corner, "after", first.first), middle];
    }

    return [
      index === (edge + 1) % corners.length
        ? withHandleTo(corner, "before", second.second)
        : corner,
    ];
  });
}

/**
 * Corners with one more point in the middle of a side, right after its first point: a sharp point
 * on a straight side (F08.AC3); on a curved side, the curve split in half, the outline kept.
 *
 * @kind domain
 * @param corners - corners as drawn, the shape at scale 1
 * @param edge - number of the side, the one of its first point, from 0
 * @returns the corners with the new point
 * @see DERIV-cubic-sides
 */
export function cornersWithMidpoint(
  corners: readonly CurveCorner[],
  edge: number,
): readonly CurveCorner[] {
  const start = cyclicItem(corners, edge, NO_CORNER);
  const end = cyclicItem(corners, edge + 1, start);

  if (isCurvedSide(start, end)) {
    return cornersWithCurveSplit(corners, edge);
  }

  const sharp = { handles: NO_HANDLES, point: midpoint(start.point, end.point), radius: 0 };

  return corners.flatMap((corner, index) => (index === edge ? [corner, sharp] : [corner]));
}
