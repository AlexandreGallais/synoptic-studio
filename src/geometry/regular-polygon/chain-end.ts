import { add, directionAt } from "../../math";

import type { Point } from "../../math";

/**
 * End of a polygonal chain of unit sides from a start, each side given by its direction: the
 * chain is recovered by adding its sides one by one.
 *
 * Formula: P = S + Σ (cos θᵢ, sin θᵢ), summed in order.
 *
 * @kind geometry
 * @param start - where the chain starts
 * @param angles - each side's angle in radians, in order
 * @returns where the chain ends
 * @see REF-CRANE-DISCRETE-CURVES
 */
export function chainEnd(start: Point, angles: readonly number[]): Point {
  return angles.reduce((point, angle) => add(point, directionAt(angle)), start);
}
