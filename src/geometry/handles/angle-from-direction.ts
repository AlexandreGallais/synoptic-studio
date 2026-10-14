import { degreesInTurn, radiansToDegrees, signedAngle } from "../../math";

import type { Vector } from "../../math";

/**
 * Angle of a handle counted from a direction, clockwise on screen, from 0 to 359.99…: the signed
 * angle between them (`DERIV-handle-angle` step 4); 0 for a handle of length 0.
 *
 * Formula: θ = atan2(u × h, u · h) · 180 / π, brought into [0, 360).
 *
 * @kind geometry
 * @param direction - unit vector the angle is counted from
 * @param handle - the handle, a vector from its point
 * @returns the angle in degrees
 * @see REF-MDN-ATAN2
 */
export function angleFromDirection(direction: Vector, handle: Vector): number {
  return degreesInTurn(radiansToDegrees(signedAngle(direction, handle)));
}
