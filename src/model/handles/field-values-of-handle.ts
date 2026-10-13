import { angleFromDirection } from "../../geometry";
import { degreesInTurn, norm, unit } from "../../math";

import type { HandleFrame } from "./handle-frame";
import type { HandleWay } from "./handle-way";
import type { Vector } from "../../math";

/** The x axis, the direction angles on the page are counted from. */
const X_AXIS = { x: 1, y: 0 };

/**
 * The two values a handle shows in a given way: x and y of its end on the page, dx and dy from
 * its point, or an angle and a length from the x axis or from its side, the handle before's
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
    side: [frame.isBefore ? degreesInTurn(-sideAngle) : sideAngle, norm(handle)],
  } as const;

  return values[way];
}
