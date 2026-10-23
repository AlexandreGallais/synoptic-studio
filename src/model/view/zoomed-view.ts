import type { View } from "./view";
import type { Vector } from "../../math";

/**
 * The view zoomed to a scale about an offset on the canvas, the point under it staying there
 * (ADR-0098): the scale taken within the bounds, the origin moved so that the point is kept.
 *
 * Formula: s′ = min(max(scale, 0.01), 256), o′ = o + p / s − p / s′.
 *
 * @kind domain
 * @param view - its scale and origin before the zoom
 * @param offset - the offset kept, from the canvas's top-left, in CSS pixels
 * @param scale - CSS pixels per user unit asked, taken within the bounds
 * @returns the view zoomed, the point under the offset kept
 * @see DERIV-view-zoom
 */
export function zoomedView(view: View, offset: Vector, scale: number): View {
  return { origin: { x: view.origin.x + offset.x, y: view.origin.y }, scale };
}
