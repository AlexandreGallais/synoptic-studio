import { angleFromDirection } from "../../geometry";
import { degreesInTurn, dot, norm, unit } from "../../math";

import { acrossSide } from "./across-side";

import type { HandleFrame } from "./handle-frame";
import type { HandleWay } from "./handle-way";
import type { Vector } from "../../math";

/** The x axis, the direction angles on the page are counted from. */
const X_AXIS = { x: 1, y: 0 };

/** A hundred: shares are typed as whole percentages. */
const PERCENT = 100;

/**
 * Shares of a handle along and across its side, in percent (`DERIV-handle-share` step 3); 0 and 0
 * for a side of length 0, which has none.
 *
 * @kind domain
 * @param handle - the handle, a vector from its point
 * @param frame - its side and which end it is
 * @returns the share along, then across, in percent
 * @see DERIV-handle-share
 */
function shareValues(handle: Vector, frame: HandleFrame): readonly [number, number] {
  const squared = dot(frame.side, frame.side);

  return squared === 0
    ? [0, 0]
    : [
        (PERCENT * dot(handle, frame.side)) / squared,
        (PERCENT * dot(handle, acrossSide(frame))) / squared,
      ];
}

/**
 * The two values a handle shows in a given way: x and y of its end on the page, dx and dy from
 * its point, an angle and a length from the x axis or from its side, or shares of its side, the
 * handle before's
 * angle mirrored (`DERIV-handle-angle`; research note 0013).
 *
 * @kind domain
 * @param way - the way chosen
 * @param handle - the handle, a vector from its point, as drawn
 * @param frame - its point, its side and which end it is
 * @returns the first and second values: x, y; dx, dy; or angle, length
 * @see DERIV-handle-angle
 */
export function fieldValuesOfHandle(
  way: HandleWay,
  handle: Vector,
  frame: HandleFrame,
): readonly [number, number] {
  const sideAngle = angleFromDirection(unit(frame.side), handle);
  const values = {
    axis: [angleFromDirection(X_AXIS, handle), norm(handle)],
    page: [frame.point.x + handle.x, frame.point.y + handle.y],
    point: [handle.x, handle.y],
    share: shareValues(handle, frame),
    side: [frame.isBefore ? degreesInTurn(-sideAngle) : sideAngle, norm(handle)],
  } as const;

  return values[way];
}
