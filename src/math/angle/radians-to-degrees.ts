import { DEGREES_PER_TURN } from "./degrees-per-turn";

/** Degrees in half a turn, π radians. */
const HALF_TURN = DEGREES_PER_TURN / 2;

/**
 * An angle in radians, in degrees: π radians are 180 degrees.
 *
 * Formula: θ = θ_R · 180 / π, the product first.
 *
 * @kind math
 * @param radians - angle in radians
 * @returns the same angle in degrees
 * @see REF-OPENSTAX-ANGLES
 */
export function radiansToDegrees(radians: number): number {
  return (radians * HALF_TURN) / Math.PI;
}
