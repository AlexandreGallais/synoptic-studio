import { EPSILON } from "../../math";

/**
 * Whether a corner's radius was reduced to fit: the radius drawn is smaller than the one asked by
 * more than a rounding error (ADR-0007, Q8).
 *
 * @kind domain
 * @param requested - the radius asked
 * @param effective - the radius drawn
 * @returns `true` when the radius drawn is smaller, beyond `EPSILON`
 * @see docs/domain/shapes.md#radius-clamping
 */
export function isReducedRadius(requested: number, effective: number): boolean {
  return requested - effective >= EPSILON;
}
