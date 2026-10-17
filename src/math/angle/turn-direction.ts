import { integerQuotient } from "../arithmetic";
import { withoutNegativeZero } from "../precision";
import { quarterTurns } from "../rotation";

import { degreesInTurn } from "./degrees-in-turn";
import { DEGREES_PER_QUARTER_TURN } from "./degrees-per-quarter-turn";
import { degreesToRadians } from "./degrees-to-radians";
import { directionAt } from "./direction-at";

import type { Vector } from "../vector";

/**
 * Direction `(cos α, sin α)` of an angle in degrees, exact at every quarter turn.
 *
 * Formula: `α = 90q + r`, `0 ≤ r < 90`; `(cos r, sin r)` turned `q` times by `(x, y) ↦ (−y, x)`.
 * `Math.cos(π / 2)` is `6.1e-17`, not 0: computing the quarter turns as such keeps 0°, 90°, 180°
 * and 270° exact. Negative zeros are made positive. In the SVG frame (y down) a positive angle is
 * clockwise on screen.
 *
 * @kind math
 * @param angle - integer number of degrees, any value
 * @returns the unit vector of the angle
 * @see DERIV-turned-shape
 */
export function turnDirection(angle: number): Vector {
  const quarters = integerQuotient(degreesInTurn(angle), DEGREES_PER_QUARTER_TURN);
  const rest = degreesToRadians(degreesInTurn(angle) - quarters * DEGREES_PER_QUARTER_TURN);

  return withoutNegativeZero(quarterTurns(directionAt(rest), quarters));
}
