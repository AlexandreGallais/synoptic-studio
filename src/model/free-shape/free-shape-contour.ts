import { curveBoundingBox, stretchedInBox } from "../../geometry";

import type { FreeShape } from "./free-shape";
import type { Point } from "../../math";

/**
 * Contour of a free shape, straight: the points of its corners stretched to its width and height,
 * the top-left of their box — the box of the curve, handles included (ADR-0046) — fixed
 * (`DERIV-free-shape-stretch`), then moved so that this top-left
 * is at its position (ADR-0035). When the position is that top-left, nothing is added: the drawing is bit for bit the earlier one.
 * Derived, never stored (ADR-0003).
 *
 * @kind domain
 * @param shape - valid free shape (`isValidFreeShape`)
 * @returns its points as drawn, in contour order
 * @see DERIV-free-shape-stretch
 */
export function freeShapeContour(shape: FreeShape): readonly Point[] {
  const points = shape.corners.map((corner) => corner.point);
  const box = curveBoundingBox(shape.corners);
  const dx = shape.x - box.minX;
  const dy = shape.y - box.minY;

  // A move of zero is skipped, so that the stored point is drawn bit for bit (−0 + 0 is +0).
  return stretchedInBox(points, box, shape).map((point) => ({
    x: dx === 0 ? point.x : point.x + dx,
    y: dy === 0 ? point.y : point.y + dy,
  }));
}
