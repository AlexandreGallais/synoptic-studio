import { angledHandle } from "../../geometry";
import { degreesInTurn, norm, unit } from "../../math";

import type { HandleFrame } from "./handle-frame";
import type { HandleWay } from "./handle-way";
import type { Vector } from "../../math";

/**
 * A handle from an angle and a length typed from its side: the angle brought into 0 to 359 as a
 * shape's (F04), the handle before's mirrored (`DERIV-handle-angle` steps 1 to 3); refused for a
 * negative length or a side of length 0, which has no direction.
 *
 * @kind domain
 * @param values - the angle in degrees and the length
 * @param frame - its point, its side and which end it is
 * @returns the handle, or `undefined` when refused
 * @see DERIV-handle-angle
 */
function sideTypedHandle(
  values: readonly [number, number],
  frame: HandleFrame,
): Vector | undefined {
  const [angle, length] = values;

  return length < 0 || norm(frame.side) === 0
    ? undefined
    : angledHandle(unit(frame.side), degreesInTurn(frame.isBefore ? -angle : angle), length);
}

/**
 * A handle from the two values typed in a given way — the inverse of `fieldValuesOfHandle`.
 *
 * @kind domain
 * @param way - the way chosen
 * @param values - x and y on the page, or an angle and a length from the side
 * @param frame - the handle's point, its side and which end it is
 * @returns the handle, a vector from its point as drawn, or `undefined` when refused
 * @see DERIV-handle-angle
 */
export function typedHandle(
  way: HandleWay,
  values: readonly [number, number],
  frame: HandleFrame,
): Vector | undefined {
  const [first, second] = values;

  return way === "page"
    ? { x: first - frame.point.x, y: second - frame.point.y }
    : sideTypedHandle(values, frame);
}
