import {
  isContourTouchingBox,
  isPieceMeetingBox,
  polygonSegments,
  widenedBox,
} from "../../geometry";
import { EPSILON } from "../../math";

import type { PickArea } from "./pick-area";
import type { BoundingBox } from "../../geometry";

/**
 * Whether a selection rectangle touches a shape: it shares a point with its painted area, its
 * contour as drawn included (`isContourTouchingBox`); for a shape without area, with one side of
 * the outline its corners join (`polygonSegments`, `isPieceMeetingBox`), no tolerance added, the
 * rectangle having an area of its own. The rectangle is widened by `EPSILON` (`widenedBox`),
 * so that one drawn along a side the contour computes with a rounding error touches it, as it
 * encloses it (ADR-0003, `DERIV-box-overlap` step 10).
 *
 * @kind domain
 * @param area - the shape's contour, corners and whether it is flat (`shapePickArea`, `textPickArea`)
 * @param box - the rectangle, in user units, its sides included
 * @returns true when the rectangle touches the shape
 * @see docs/domain/interaction.md#_2-selector
 */
export function isPickAreaTouched(area: PickArea, box: BoundingBox): boolean {
  const reach = widenedBox(box, EPSILON);

  return area.isFlat
    ? polygonSegments(area.corners).some((side) => isPieceMeetingBox(side, reach))
    : isContourTouchingBox(area.pieces, reach);
}
