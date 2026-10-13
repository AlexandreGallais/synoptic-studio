import { freeShapeContour } from "../free-shape";

import { rectangleAsFreeShape } from "./rectangle-as-free-shape";

import type { Rectangle } from "./rectangle";
import type { Point } from "../../math";

/**
 * Straight contour of a rectangle: the free shape the rectangle tool makes (ADR-0032), i.e.
 * (x, y), (x + width, y), (x + width, y + height), (x, y + height), clockwise from the top-left
 * vertex (Q11); its angle is not applied (`turnedCorners`, ADR-0035).
 *
 * @kind domain
 * @param rectangle - width and height, integers ≥ 0
 * @returns its four vertices, in contour order
 * @see DERIV-free-shape-stretch
 */
export function rectangleContour(rectangle: Rectangle): readonly Point[] {
  return freeShapeContour(rectangleAsFreeShape(rectangle));
}
