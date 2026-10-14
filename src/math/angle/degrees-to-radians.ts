import { DEGREES_PER_TURN } from "./degrees-per-turn";

/** Degrees in half a turn, π radians. */
const HALF_TURN = DEGREES_PER_TURN / 2;

/**
 * An angle in degrees, in radians: π radians are 180 degrees.
 *
 * Formula: θ_R = θ · π / 180, the product first.
 *
 * @kind math
 * @param degrees - angle in degrees
 * @returns the same angle in radians
 * @see REF-OPENSTAX-ANGLES
 */
export function degreesToRadians(degrees: number): number {
  return (degrees * Math.PI) / HALF_TURN;
}
