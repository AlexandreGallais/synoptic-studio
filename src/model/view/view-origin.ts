import { opposite } from "../../math";

import { pointUnderPointer } from "./point-under-pointer";

import type { Point, Vector } from "../../math";

/**
 * The view's origin that shows a point of the drawing at an offset on the canvas, at a scale: the
 * point under the opposite offset of a view whose origin is that point (`pointUnderPointer`).
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
  return pointUnderPointer({ origin: point, scale }, opposite(offset));
}
