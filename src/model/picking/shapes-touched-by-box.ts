import { isPickAreaTouched } from "./is-pick-area-touched";

import type { PickArea } from "./pick-area";
import type { BoundingBox } from "../../geometry";

/**
 * The shapes a selection rectangle touches (`isPickAreaTouched`), by their painted area as a
 * click picks them (F37, `DERIV-box-overlap`).
 *
 * @kind domain
 * @param areas - what each shape of the drawing offers the pointer, in the order drawn
 * @param box - the rectangle, in user units, its sides included
 * @returns the indices of the shapes touched, in the order drawn
 * @see RULE-interaction-2-selector
 */
export function shapesTouchedByBox(
  areas: readonly PickArea[],
  box: BoundingBox,
): readonly number[] {
  return areas.flatMap((area, index) => (isPickAreaTouched(area, box) ? [index] : []));
}
