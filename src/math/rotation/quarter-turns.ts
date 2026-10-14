import { quarterTurn } from "./quarter-turn";

import type { Vector } from "../vector";

/**
 * Vector turned by a whole number of quarter turns, each exact: no cosine nor sine is rounded
 * (`DERIV-turned-shape`, R_α = R_90^q ∘ R_r).
 *
 * Formula: R_90^q (v).
 *
 * @kind math
 * @param vector - vector to turn
 * @param count - number of quarter turns, an integer ≥ 0
 * @returns the turned vector
 * @see DERIV-turned-shape
 */
export function quarterTurns(vector: Vector, count: number): Vector {
  return Array.from({ length: count }).reduce<Vector>((turned) => quarterTurn(turned), vector);
}
