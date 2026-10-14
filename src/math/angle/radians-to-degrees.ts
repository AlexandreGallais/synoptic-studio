import { DEGREES_PER_HALF_TURN } from "./degrees-per-half-turn";

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
  return (radians * DEGREES_PER_HALF_TURN) / Math.PI;
}
