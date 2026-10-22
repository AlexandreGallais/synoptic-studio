import { freeShapeCorners } from "../free-shape";
import { curveCornerOf } from "../handles";
import { turnedCorners } from "../placement";
import { regularPolygonCorners } from "../regular-polygon";
import { roundShapeCorners } from "../round-shape";

import { isRegularPolygon } from "./is-regular-polygon";
import { isRoundShape } from "./is-round-shape";

import type { Shape } from "./shape";
import type { CurveCorner } from "../../geometry";

/**
 * Straight corners of a valid shape, whatever its kind, before it turns.
 *
 * @kind domain
 * @param shape - a valid shape of the model
 * @returns its corners, in contour order
 * @see DERIV-turned-shape
 */
function straightCorners(shape: Shape): readonly CurveCorner[] {
  if (isRegularPolygon(shape)) {
    return regularPolygonCorners(shape).map((corner) => curveCornerOf(corner));
  }

  return isRoundShape(shape) ? roundShapeCorners(shape) : freeShapeCorners(shape);
}

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
  return turnedCorners(straightCorners(shape), shape);
}
