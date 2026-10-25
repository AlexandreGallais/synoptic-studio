import { freeShapeAroundCorners, freeShapeKeptInPlace } from "../free-shape";
import { curveCornerOf } from "../handles";
import { turnedCorners } from "../placement";

import { regularPolygonCorners } from "./regular-polygon-corners";

import type { RegularPolygon } from "./regular-polygon";
import type { FreeShape } from "../free-shape";

/**
 * Free shape a regular polygon becomes by the conversion button (F08): its corners as drawn,
 * exact, and the size of the box around them, so that it draws the same and does not move
 * (ADR-0031, ADR-0033); a turned polygon keeps its angle, its box placed so that it does not
 * move either (ADR-0035, `DERIV-turned-shape` step 5). It no longer has a number of corners nor an anchor.
 *
 * @kind domain
 * @param polygon - valid polygon (`isValidRegularPolygon`) of width and height > 0: one of size 0
 * has no box to stretch, and is not converted (F08)
 * @returns the free shape drawing that polygon
 * @see RULE-interaction-3-node-editing
 */
export function regularPolygonAsFreeShape(polygon: RegularPolygon): FreeShape {
  const corners = regularPolygonCorners(polygon).map((corner) => curveCornerOf(corner));

  return freeShapeKeptInPlace(
    freeShapeAroundCorners(corners, polygon.angle),
    turnedCorners(corners, polygon),
    { after: 0, before: 0 },
  );
}
