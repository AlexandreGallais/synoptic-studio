import { degreesInTurn } from "./degrees-in-turn";

/**
 * Explement of an angle in degrees: what it lacks to make a full turn, the same angle measured the
 * other way round.
 *
 * Formula: (360 − α) mod 360, written (−α) mod 360.
 *
 * @kind math
 * @param angle - angle in degrees
 * @returns its explement, from 0 to 359.999…
 * @see REF-WIKI-ANGLE
 */
export function explement(angle: number): number {
  return degreesInTurn(-angle);
}
