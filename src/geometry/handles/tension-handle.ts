import { scale, subtract } from "../../math";

import type { Point, Vector } from "../../math";

/** The 6 of the cardinal spline's handle: a tangent over 2, and a handle a third of the tangent. */
const SIXTH = 6;

/** A hundred: a tension is a whole percentage. */
const PERCENT = 100;

/**
 * The handle after a point smoothed by a tension, from its neighbours, as a cardinal spline
 * (`DERIV-handle-tension`); the handle before is its opposite.
 *
 * Formula: after = (1 − c)(P₊ − P₋) / 6, c the tension from 0 (Catmull–Rom) to 1 (straight).
 *
 * @kind geometry
 * @param previous - the point before, as drawn
 * @param next - the point after, as drawn
 * @param tension - whole percentage from 0 to 100
 * @returns the handle after the point
 * @see DERIV-handle-tension
 */
export function tensionHandle(previous: Point, next: Point, tension: number): Vector {
  return scale(subtract(next, previous), (PERCENT - tension) / (PERCENT * SIXTH));
}
