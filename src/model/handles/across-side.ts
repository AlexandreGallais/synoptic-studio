import { quarterTurn, scale, withoutNegativeZero } from "../../math";

import type { HandleFrame } from "./handle-frame";
import type { Vector } from "../../math";

/**
 * The direction "across" a handle's side, as long as the side: the side turned a quarter turn
 * to its left for a handle after (above a side going right), the other way for a handle before,
 * so that equal shares at both ends make a symmetric arch (`DERIV-handle-share` steps 1 and 2).
 *
 * Formula: n = (s_y, −sₓ) after, (−s_y, sₓ) before.
 *
 * @kind domain
 * @param frame - the handle's side and which end it is
 * @returns the vector across the side
 * @see DERIV-handle-share
 */
export function acrossSide(frame: HandleFrame): Vector {
  const turned = quarterTurn(frame.side);

  return withoutNegativeZero(frame.isBefore ? turned : scale(turned, -1));
}
