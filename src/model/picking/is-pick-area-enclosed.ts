import { isBoxWithin } from "../../geometry";

import { pickAreaBox } from "./pick-area-box";

import type { PickArea } from "./pick-area";
import type { BoundingBox } from "../../geometry";

/**
 * Whether a selection rectangle encloses a shape: the straight box of its contour as drawn, its
 * arcs and cubic sides by their extremes, or of its corners for a shape without area
 * (`pickAreaBox`), lies within the rectangle (`isBoxWithin`).
 *
 * @kind domain
 * @param area - the shape's contour, corners and whether it is flat (`shapePickArea`, `textPickArea`)
 * @param box - the rectangle, in user units, its sides included
 * @returns true when every point of the shape lies in the rectangle
 * @see docs/domain/interaction.md#_2-selector
 */
export function isPickAreaEnclosed(area: PickArea, box: BoundingBox): boolean {
  return isBoxWithin(pickAreaBox(area), box);
}
