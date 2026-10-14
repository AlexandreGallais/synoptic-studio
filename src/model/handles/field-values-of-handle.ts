import { angleFromDirection } from "../../geometry";
import { add, explement, norm, unit } from "../../math";

import type { HandleFrame } from "./handle-frame";
import type { HandleWay } from "./handle-way";
import type { Vector } from "../../math";

/**
 * The two values a handle shows in a given way: x and y of its end on the page, or an angle and a
 * length from its side, the handle before's angle mirrored (`DERIV-handle-angle`; research note
 * 0013).
 *
 * @kind domain
 * @param way - the way chosen
 * @param handle - the handle, a vector from its point, as drawn
 * @param frame - its point, its side and which end it is
 * @returns the first and second values: x, y; or angle, length
 * @see DERIV-handle-angle
 */
export function fieldValuesOfHandle(
  way: HandleWay,
  handle: Vector,
  frame: HandleFrame,
): readonly [number, number] {
  if (way === "page") {
    const end = add(frame.point, handle);

    return [end.x, end.y];
  }

  const sideAngle = angleFromDirection(unit(frame.side), handle);

  return [frame.isBefore ? explement(sideAngle) : sideAngle, norm(handle)];
}
