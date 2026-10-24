import { screenToUser } from "../drawing";

import type { View } from "./view";
import type { ViewBox } from "./view-box";
import type { Size } from "../../geometry";

/**
 * The `viewBox` both SVGs are given for a view (ADR-0098, decision 2): from its origin, the canvas's
 * size divided by its scale, so that it has the canvas's aspect and one scale.
 *
 * Formula: (o.x, o.y, W / s, H / s).
 *
 * @kind domain
 * @param view - the view's scale and origin
 * @param canvas - the canvas's size, in CSS pixels
 * @returns the region of the drawing shown, in user units
 * @see DERIV-view-zoom
 */
export function shownViewBox(view: View, canvas: Size): ViewBox {
  return {
    height: screenToUser(canvas.height, view.scale),
    width: screenToUser(canvas.width, view.scale),
    x: view.origin.x,
    y: view.origin.y,
  };
}
