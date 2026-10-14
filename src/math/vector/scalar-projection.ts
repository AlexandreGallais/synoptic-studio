import { dot } from "./dot";
import { subtract } from "./subtract";

import type { Point } from "./point";
import type { Vector } from "./vector";

/**
 * Signed length of a point's projection on a line through an origin: how far along the line's
 * direction the point lies.
 *
 * Formula: (P − O) · u, with u a unit vector.
 *
 * @kind math
 * @param point - the point projected
 * @param origin - a point of the line
 * @param direction - unit vector of the line
 * @returns the signed length along the direction
 * @see REF-MATHWORLD-SCALAR-PROJECTION
 */
export function scalarProjection(point: Point, origin: Point, direction: Vector): number {
  return dot(subtract(point, origin), direction);
}
