import { angledHandle } from "../../geometry";
import { degreesInTurn, norm, unit, withoutNegativeZero } from "../../math";

import { acrossSide } from "./across-side";

import type { HandleFrame } from "./handle-frame";
import type { HandleWay } from "./handle-way";
import type { Vector } from "../../math";

/** A hundred: shares are typed as whole percentages. */
const PERCENT = 100;

/** The x axis, the direction angles on the page are counted from. */
const X_AXIS = { x: 1, y: 0 };

/**
 * Whether an angle and a length cannot give a handle: a negative length, or an angle counted from
 * a side of length 0, which has no direction.
 *
 * @kind domain
 * @param way - `axis` or `side`
 * @param length - the length typed
 * @param side - the handle's side
 * @returns `true` when refused
 * @see DERIV-handle-angle
 */
function isRefusedAngle(way: "axis" | "side", length: number, side: Vector): boolean {
  return length < 0 || (way === "side" && norm(side) === 0);
}

/**
 * A handle from an angle and a length typed: the angle brought into 0 to 359 as a shape's (F04),
 * the handle before's mirrored when counted from its side (`DERIV-handle-angle` steps 1 to 3).
 *
 * @kind domain
 * @param way - `axis` or `side`
 * @param values - the angle in degrees and the length
 * @param frame - its point, its side and which end it is
 * @returns the handle, or `undefined` when refused
 * @see DERIV-handle-angle
 */
function angleTypedHandle(
  way: "axis" | "side",
  values: readonly [number, number],
  frame: HandleFrame,
): Vector | undefined {
  const [angle, length] = values;
  const isMirrored = way === "side" && frame.isBefore;
  const direction = way === "axis" ? X_AXIS : unit(frame.side);

  return isRefusedAngle(way, length, frame.side)
    ? undefined
    : angledHandle(direction, degreesInTurn(isMirrored ? -angle : angle), length);
}

/**
 * A handle from whole percentages of its side, along and across it (`DERIV-handle-share` steps 1
 * and 2); refused for a side of length 0. Multiplying before dividing keeps 58 % of 100 at 58.
 *
 * @kind domain
 * @param values - the share along, then across, in percent
 * @param frame - its side and which end it is
 * @returns the handle, or `undefined` when refused
 * @see DERIV-handle-share
 */
function shareTypedHandle(
  values: readonly [number, number],
  frame: HandleFrame,
): Vector | undefined {
  const [along, across] = values;
  const normal = acrossSide(frame);

  return norm(frame.side) === 0
    ? undefined
    : withoutNegativeZero({
        x: (along * frame.side.x + across * normal.x) / PERCENT,
        y: (along * frame.side.y + across * normal.y) / PERCENT,
      });
}

/**
 * A handle from the two values typed in a given way — the inverse of `fieldValuesOfHandle`.
 *
 * @kind domain
 * @param way - the way chosen
 * @param values - x, y; dx, dy; angle, length; or shares along and across
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

  if (way === "page") {
    return { x: first - frame.point.x, y: second - frame.point.y };
  }

  if (way === "share") {
    return shareTypedHandle(values, frame);
  }

  return way === "point" ? { x: first, y: second } : angleTypedHandle(way, values, frame);
}
