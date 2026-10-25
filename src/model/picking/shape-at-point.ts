import { isPickAreaAt } from "./is-pick-area-at";

import type { PickArea } from "./pick-area";
import type { Point } from "../../math";

/**
 * The shape under a point: the last drawn whose area the point reaches (`isPickAreaAt`), the
 * drawing's shapes being in the order drawn, the last on top (F36, `DERIV-painted-area`).
 *
 * @kind domain
 * @param areas - what each shape of the drawing offers the pointer, in the order drawn
 * @param point - the point asked, in user units
 * @param tolerance - the reach of a shape without area, in user units (`PICK_TOLERANCE` over the view's scale)
 * @returns the index of that shape, −1 for none
 * @see RULE-interaction-2-selector
 */
export function shapeAtPoint(areas: readonly PickArea[], point: Point, tolerance: number): number {
  return areas.findLastIndex((area) => isPickAreaAt(area, point, tolerance));
}
