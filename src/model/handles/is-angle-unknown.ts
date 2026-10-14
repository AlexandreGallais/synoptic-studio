import { isZeroVector } from "../../math";

import type { HandleWay } from "./handle-way";
import type { Vector } from "../../math";

/**
 * Whether a handle typed as an angle and a length has no angle to show: its length is 0
 * (`DERIV-handle-angle` step 4), so the angle typed before its length must be kept as it was typed.
 *
 * @kind domain
 * @param way - the way the handle is typed
 * @param handle - the handle, a vector from its point
 * @returns `true` for the way `side` and a handle of length 0
 * @see DERIV-handle-angle
 */
export function isAngleUnknown(way: HandleWay, handle: Vector): boolean {
  return way === "side" && isZeroVector(handle);
}
