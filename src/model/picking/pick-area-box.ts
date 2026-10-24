import { boundingBox, contourBoundingBox } from "../../geometry";

import type { PickArea } from "./pick-area";
import type { BoundingBox } from "../../geometry";

/**
 * The straight box of what a shape offers the pointer: the box of its contour as drawn, its arcs
 * and cubic sides by their extremes (`contourBoundingBox`); for a shape without area, the box of
 * its corners (`boundingBox`), a shape 0 × 0 the one point they lie on.
 *
 * @kind domain
 * @param area - the shape's contour, corners and whether it is flat (`shapePickArea`, `textPickArea`)
 * @returns its extreme coordinates
 * @see docs/domain/interaction.md#_2-selector
 */
export function pickAreaBox(area: PickArea): BoundingBox {
  return area.isFlat ? boundingBox(area.corners) : contourBoundingBox(area.pieces);
}
