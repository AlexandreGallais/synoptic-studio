import { freeShapeContour } from "./free-shape-contour";
import { rectangleAsFreeShape } from "./rectangle-as-free-shape";

import type { Rectangle } from "./rectangle";
import type { Point } from "../math";

/**
 * Contour of a rectangle: the free shape the rectangle tool makes, drawn (ADR-0032), i.e.
 * (0, 0), (width, 0), (width, height), (0, height), clockwise from the top-left vertex (Q11).
 *
 * @kind domain
 * @param rectangle - width and height, integers ≥ 0
 * @returns its four vertices, in contour order
 * @see DERIV-free-shape-stretch
 */
export function rectangleContour(rectangle: Rectangle): readonly Point[] {
  return freeShapeContour(rectangleAsFreeShape(rectangle));
}
