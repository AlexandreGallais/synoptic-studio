import { cubicSide, cyclicItem, isCurvedSide, mirroredHandle, splitCubic } from "../../geometry";
import { midpoint, modulo, subtract } from "../../math";
import { NO_HANDLES } from "../handles";

import type { Cubic, CurveCorner } from "../../geometry";
import type { Point } from "../../math";

/** Corner read when the side asked for does not exist: none is, the shape has 3 points or more. */
const NO_CORNER: CurveCorner = { handles: NO_HANDLES, point: { x: 0, y: 0 }, radius: 0 };

/**
 * A corner whose handle after or before now reaches a given control point, when its point is in
 * handles mode (an off point draws no handle and keeps its stored one). Shortened by the split, its
 * handles leave mode angle and length for mode angle: still on one line, of their own lengths.
 *
 * @kind domain
 * @param corner - corner to change
 * @param end - which handle: towards the next point (`after`) or the previous (`before`)
 * @param control - the control point it now reaches
 * @returns the corner, that handle set to the control point minus its point
 * @see DERIV-cubic-sides
 */
function withHandleTo(corner: CurveCorner, end: "after" | "before", control: Point): CurveCorner {
  if (!corner.handles.isOn) {
    return corner;
  }

  const mode = corner.handles.mode === "none" ? "none" : "angle";

  return {
    ...corner,
    handles: { ...corner.handles, mode, [end]: subtract(control, corner.point) },
  };
}

/**
 * The point added at the middle of a curved side: in handles mode, angle and length, its handle
 * before towards the first half's second control point, its handle after the opposite — opposite
 * in exact arithmetic (M halves P₀₁₂ P₁₂₃), written so in floats too (`DERIV-cubic-sides` step 4).
 *
 * @kind domain
 * @param first - the first half of the split curve, ending at the new point
 * @returns the new corner, radius 0
 * @see DERIV-cubic-sides
 */
function splitPoint(first: Cubic): CurveCorner {
  const before = subtract(first.second, first.end);

  return {
    handles: {
      ...NO_HANDLES,
      // A symmetric node, as the curve split there.
      after: mirroredHandle("angle-and-length", before, before),
      before,
      isOn: true,
    },
    point: first.end,
    radius: 0,
  };
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
  const [first, second] = splitCubic(cubicSide(start, cyclicItem(corners, edge + 1, start)), 1 / 2);
  const middle = splitPoint(first);

  return corners.flatMap((corner, index) => {
    if (index === edge) {
      return [withHandleTo(corner, "after", first.first), middle];
    }

    return [
      index === modulo(edge + 1, corners.length)
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
