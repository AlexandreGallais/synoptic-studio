import { modulo } from "../arithmetic";

import { DEGREES_PER_TURN } from "./degrees-per-turn";

/**
 * Angle in degrees kept from 0 to 359: the same direction, a whole number of turns removed.
 *
 * Formula: `((a % 360) + 360) % 360`. JavaScript's `%` is a remainder with the sign of the
 * dividend (`−90 % 360` is `−90`), not a modulo; adding 360 before the second remainder gives the
 * modulo, as Python's `a % 360` would. The inner sum lies between 0 and 720, so the result is
 * never a negative zero.
 *
 * @kind math
 * @param angle - integer number of degrees
 * @returns the angle from 0 to 359
 * @see DERIV-turned-shape
 */
export function degreesInTurn(angle: number): number {
  return modulo(angle, DEGREES_PER_TURN);
}
