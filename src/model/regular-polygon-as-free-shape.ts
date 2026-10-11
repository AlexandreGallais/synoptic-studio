import { boundingBox } from "../geometry";

import { regularPolygonCorners } from "./regular-polygon-corners";

import type { FreeShape } from "./free-shape";
import type { RegularPolygon } from "./regular-polygon";

/**
 * Free shape a regular polygon becomes by the conversion button (F08): its corners as drawn,
 * exact, and the size of the box around them, so that it draws the same and does not move
 * (ADR-0031, ADR-0033). It no longer has a number of corners nor an anchor.
 *
 * @kind domain
 * @param polygon - valid polygon (`isValidRegularPolygon`) of a size > 0
 * @returns the free shape drawing that polygon
 * @see docs/domain/interaction.md#_3-node-editing
 */
export function regularPolygonAsFreeShape(polygon: RegularPolygon): FreeShape {
  const corners = regularPolygonCorners(polygon);
  const box = boundingBox(corners.map((corner) => corner.point));

  return { corners, height: box.maxY - box.minY, width: box.maxX - box.minX };
}
