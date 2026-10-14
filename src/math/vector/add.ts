import type { Point } from "./point";
import type { Vector } from "./vector";

/**
 * Point moved by a vector, component-wise.
 *
 * Formula: p + v = (pₓ + vₓ, p_y + v_y) — vector addition, the point being read as its vector
 * from the origin.
 *
 * @kind math
 * @param point - start point
 * @param vector - displacement
 * @returns the moved point
 * @see REF-MATHWORLD-VECTOR-ADDITION
 */
export function add(point: Point, vector: Vector): Point {
  return { x: point.x + vector.x, y: point.y + vector.y };
}
