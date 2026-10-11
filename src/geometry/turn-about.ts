import type { Point, Vector } from "../math";

/**
 * Point turned about a center by the angle of a direction.
 *
 * Formula: `c + R(p − c)`, `R(x, y) = (x cos α − y sin α, x sin α + y cos α)`: SVG's
 * `rotate(α, cx, cy)`, translate, rotate, translate back. In the SVG frame (y down) a positive
 * angle is clockwise on screen.
 *
 * @kind geometry
 * @param point - any point of the plane
 * @param center - the point that stays where it is
 * @param direction - `(cos α, sin α)` of the angle (`turnDirection`)
 * @returns the turned point
 * @see REF-SVG11-TRANSFORM
 */
export function turnAbout(point: Point, center: Point, direction: Vector): Point {
  const dx = point.x - center.x;
  const dy = point.y - center.y;

  return {
    x: center.x + (dx * direction.x - dy * direction.y),
    y: center.y + (dx * direction.y + dy * direction.x),
  };
}
