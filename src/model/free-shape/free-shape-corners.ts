import { cyclicItem, stretchFactors, stretchedHandles } from "../../geometry";
import { cornersWithRadii } from "../corner-radius";
import { NO_HANDLES } from "../handles";

import { freeShapeContour } from "./free-shape-contour";
import { freeShapeRadii } from "./free-shape-radii";

import type { FreeShape } from "./free-shape";
import type { CurveCorner } from "../../geometry";

/**
 * Straight corners of a free shape, placed at its position but not turned (`turnedCorners` turns
 * them, ADR-0035): each point of its contour with its requested radius (ADR-0033), clamped later
 * by the geometry of F01 (ADR-0007), and its handles stretched with it (ADR-0046).
 *
 * @kind domain
 * @param shape - valid free shape (`isValidFreeShape`)
 * @returns one corner per point, in contour order, its handles as drawn
 * @see docs/domain/shapes.md#_2-corner-radius
 */
export function freeShapeCorners(shape: FreeShape): readonly CurveCorner[] {
  const factors = stretchFactors(
    shape.corners.map((corner) => corner.point),
    shape,
  );

  const handles = shape.corners.map((corner) => corner.handles);

  return cornersWithRadii(freeShapeContour(shape), freeShapeRadii(shape)).map((corner, index) => ({
    ...corner,
    handles: stretchedHandles(cyclicItem(handles, index, NO_HANDLES), factors),
  }));
}
