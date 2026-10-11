import { stretchToSize } from "../geometry";

import type { FreeShape } from "./free-shape";
import type { Point } from "../math";

/**
 * Contour of a free shape: the points of its corners stretched to its width and height, the
 * top-left of their box fixed (`DERIV-free-shape-stretch`). Derived, never stored (ADR-0003).
 *
 * @kind domain
 * @param shape - valid free shape (`isValidFreeShape`)
 * @returns its points as drawn, in contour order
 * @see DERIV-free-shape-stretch
 */
export function freeShapeContour(shape: FreeShape): readonly Point[] {
  return stretchToSize(
    shape.corners.map((corner) => corner.point),
    shape,
  );
}
