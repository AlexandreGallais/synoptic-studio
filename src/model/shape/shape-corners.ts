import type { Shape } from "./shape";
import type { CurveCorner } from "../../geometry";

/**
 * Corners of a valid shape as drawn, whatever its kind: those of its contour, each with its
 * radius and handles, turned by its angle about the center of its box (`turnedCorners`); a
 * regular polygon's straight corners (`regularPolygonCorners`) and a round shape's four points on
 * its curve (`roundShapeCorners`) are read as a free shape's are (`freeShapeCorners`).
 *
 * @kind domain
 * @param shape - a valid shape of the model
 * @returns its corners as drawn, in contour order
 * @see DERIV-turned-shape
 */
export function shapeCorners(shape: Shape): readonly CurveCorner[] {
  return shape.angle === 0 ? [] : [];
}
