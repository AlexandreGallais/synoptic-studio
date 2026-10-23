import { add, scale } from "../../math";

import type { View } from "./view";
import type { Vector } from "../../math";

/**
 * The view panned by the wheel's deltas, in screen pixels, its origin moved by them divided by the
 * scale; a drag of the pointer pans by its opposite, the drawing following the pointer.
 *
 * Formula: o′ = o + δ / s.
 *
 * @kind domain
 * @param view - its scale and origin before the pan
 * @param delta - the pan, in CSS pixels
 * @returns the view panned, its scale unchanged
 * @see DERIV-view-zoom
 */
export function pannedView(view: View, delta: Vector): View {
  return { origin: add(view.origin, scale(delta, 1 / view.scale)), scale: view.scale };
}
