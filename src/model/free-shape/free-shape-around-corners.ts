import { curveBoundingBox } from "../../geometry";
import { smoothedCorners } from "../handles";

import type { FreeShape } from "./free-shape";
import type { CurveCorner } from "../../geometry";

/**
 * Free shape fitted around straight corners, placed in the SVG but not turned: those corners, and
 * the box of their curve as its position, width and height (ADR-0046), so that at 0° it draws them
 * where they are (ADR-0033, ADR-0035). On a turned shape the new box has another center: the edits
 * then place it so that the shape does not move (`freeShapeKeptInPlace`).
 *
 * @kind domain
 * @param corners - straight points, placed in the SVG, their radii and handles as drawn, in contour order
 * @param angle - angle kept, from 0 to 359 (ADR-0035)
 * @returns the free shape of those corners, at scale 1
 * @see DERIV-free-shape-stretch
 */
export function freeShapeAroundCorners(corners: readonly CurveCorner[], angle: number): FreeShape {
  const box = curveBoundingBox(smoothedCorners(corners));

  return {
    angle,
    corners,
    height: box.maxY - box.minY,
    width: box.maxX - box.minX,
    x: box.minX,
    y: box.minY,
  };
}
