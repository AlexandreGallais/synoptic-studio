/** Degrees in a whole turn. */
const TURN = 360;

/**
 * Angle in degrees kept from 0 to 359: the same direction, a whole number of turns removed.
 *
 * Formula: `((a % 360) + 360) % 360`. JavaScript's `%` is a remainder with the sign of the
 * dividend (`−90 % 360` is `−90`), not a modulo; adding 360 before the second remainder gives the
 * modulo, as Python's `a % 360` would. A whole turn gives a positive zero.
 *
 * @kind math
 * @param angle - integer number of degrees
 * @returns the angle from 0 to 359
 * @see DERIV-turned-shape
 */
export function degreesInTurn(angle: number): number {
  // The final + 0 makes a negative zero positive (−0 + 0 is +0).
  return (((angle % TURN) + TURN) % TURN) + 0;
}
