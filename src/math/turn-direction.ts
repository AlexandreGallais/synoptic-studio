import { degreesInTurn } from "./degrees-in-turn";
import { quarterTurn } from "./quarter-turn";

import type { Vector } from "./vector";

/** Degrees in a quarter turn. */
const QUARTER = 90;

/** Degrees in a half turn, π radians. */
const HALF_TURN = 180;

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
  const quarters = Math.floor(degreesInTurn(angle) / QUARTER);
  const rest = ((degreesInTurn(angle) - quarters * QUARTER) * Math.PI) / HALF_TURN;
  const turned = Array.from({ length: quarters }).reduce<Vector>((vector) => quarterTurn(vector), {
    x: Math.cos(rest),
    y: Math.sin(rest),
  });

  // −0 + 0 is +0.
  return { x: turned.x + 0, y: turned.y + 0 };
}
