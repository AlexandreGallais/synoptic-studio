import type { View } from "./view";
import type { Point, Vector } from "../../math";

/**
 * The point of the drawing under an offset on the canvas (ADR-0098, decision 3): the view's
 * origin plus the offset divided by its scale, the inverse of the `viewBox`'s map.
 *
 * Formula: o + p / s.
 *
 * @kind domain
 * @param view - the view's scale and origin
 * @param offset - the offset from the canvas's top-left, in CSS pixels
 * @returns the point, in user units
 * @see DERIV-view-zoom
 */
export function pointUnderPointer(view: View, offset: Vector): Point {
  return { x: view.origin.x + offset.x, y: view.origin.y + offset.y };
}
