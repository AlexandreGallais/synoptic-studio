import { boundingBox, contourBoundingBox } from "../placement";

import { curveContour } from "./curve-contour";

import type { CurveCorner } from "../handles";
import type { BoundingBox } from "../placement";

/**
 * Straight box of a shape: the smallest upright rectangle around its contour as drawn, turned,
 * its corners rounded and its sides curved (`curveContour`, `contourBoundingBox`); for a shape
 * whose contour has no piece, 0 × 0, the box of its corners, the one point they lie on
 * (`boundingBox`), never the empty box's infinite sides.
 *
 * @kind geometry
 * @param corners - corners of the shape as drawn, in contour order
 * @returns its extreme coordinates
 * @see DERIV-arc-bounding-box
 */
export function shapeStraightBox(corners: readonly CurveCorner[]): BoundingBox {
  const pieces = curveContour(corners);

  return pieces.length > 0
    ? contourBoundingBox(pieces)
    : boundingBox(corners.map(({ point }) => point));
}
