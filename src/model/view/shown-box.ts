import { screenToUser } from "../drawing";

import type { View } from "./view";
import type { BoundingBox, Size } from "../../geometry";

/**
 * The area a view shows, as an upright box, widened by a padding on each side (`DERIV-band-within-box`
 * step 1): from its origin, the canvas's size and the padding divided by the scale (`screenToUser`);
 * a padding of 0 the area shown, half the canvas the box the helps of the layer above are computed
 * within (SP-030's answer 7).
 *
 * Formula: [o.x − p_W / s, o.x + (W + p_W) / s] × [o.y − p_H / s, o.y + (H + p_H) / s].
 *
 * @kind domain
 * @param view - the view's scale and origin
 * @param canvas - the canvas's size, in CSS pixels
 * @param padding - the room added on each side, across and down, in CSS pixels, ≥ 0
 * @returns the box, in user units
 * @see DERIV-band-within-box
 */
export function shownBox(view: View, canvas: Size, padding: Size): BoundingBox {
  return {
    maxX: view.origin.x + screenToUser(canvas.width + padding.width, view.scale),
    maxY: view.origin.y + screenToUser(canvas.height + padding.height, view.scale),
    minX: view.origin.x - screenToUser(padding.width, view.scale),
    minY: view.origin.y - screenToUser(padding.height, view.scale),
  };
}
