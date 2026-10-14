import { freeShapeAroundCorners, freeShapeKeptInPlace } from "../free-shape";
import { turnedCorners } from "../placement";

import { roundShapeCorners } from "./round-shape-corners";

import type { RoundShape } from "./round-shape";
import type { FreeShape } from "../free-shape";

/**
 * Free shape a round shape becomes by **Convert to free shape** (F10.AC4): its 4 points on the
 * curve with their handles, and the box of their curve, so that it draws the same and does not
 * move (ADR-0048); a turned shape keeps its angle, placed so that it does not move either
 * (ADR-0035, `DERIV-turned-shape` step 5).
 *
 * @kind domain
 * @param shape - valid round shape of width and height > 0: one of size 0 is not converted (F08)
 * @returns the free shape drawing that round shape
 * @see DERIV-round-shape-curve
 */
export function roundShapeAsFreeShape(shape: RoundShape): FreeShape {
  const corners = roundShapeCorners(shape);

  return freeShapeKeptInPlace(
    freeShapeAroundCorners(corners, shape.angle),
    turnedCorners(corners, shape),
    { after: 0, before: 0 },
  );
}
