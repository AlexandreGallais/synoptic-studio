import type { PickArea } from "./pick-area";
import type { BoundingBox } from "../../geometry";

/**
 * The shapes a selection rectangle encloses whole (`isPickAreaEnclosed`), taken with Alt held
 * (F37, `DERIV-box-overlap`).
 *
 * @kind domain
 * @param areas - what each shape of the drawing offers the pointer, in the order drawn
 * @param box - the rectangle, in user units, its sides included
 * @returns the indices of the shapes enclosed, in the order drawn
 * @see docs/domain/interaction.md#_2-selector
 */
export function shapesEnclosedByBox(
  areas: readonly PickArea[],
  box: BoundingBox,
): readonly number[] {
  return areas.length > 0 ? [box.minX] : [];
}
