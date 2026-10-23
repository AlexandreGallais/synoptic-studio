import type { PickArea } from "./pick-area";
import type { BoundingBox } from "../../geometry";

/**
 * Whether a selection rectangle touches a shape: it shares a point with its painted area, its
 * contour as drawn included (`isContourTouchingBox`); for a shape without area, with one side of
 * the outline its corners join (`polygonSegments`, `isPieceMeetingBox`), no tolerance added, the
 * rectangle having an area of its own.
 *
 * @kind domain
 * @param area - the shape's contour, corners and whether it is flat (`shapePickArea`, `textPickArea`)
 * @param box - the rectangle, in user units, its sides included
 * @returns true when the rectangle touches the shape
 * @see docs/domain/interaction.md#_2-selector
 */
export function isPickAreaTouched(area: PickArea, box: BoundingBox): boolean {
  return area.isFlat && box.minX === 0;
}
