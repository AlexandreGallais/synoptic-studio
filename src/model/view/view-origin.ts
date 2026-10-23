import type { Point, Vector } from "../../math";

/**
 * The view's origin that shows a point of the drawing at an offset on the canvas, at a scale.
 *
 * Formula: P − p / s.
 *
 * @kind domain
 * @param point - the point of the drawing, in user units
 * @param offset - the offset it is shown at, from the canvas's top-left, in CSS pixels
 * @param scale - CSS pixels per user unit, > 0
 * @returns the point at the canvas's top-left, in user units
 * @see DERIV-view-zoom
 */
export function viewOrigin(point: Point, offset: Vector, scale: number): Point {
  return { x: point.x - offset.x * scale, y: point.y - offset.y * scale };
}
