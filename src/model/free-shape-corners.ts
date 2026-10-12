import { cornersWithRadii } from "./corners-with-radii";
import { freeShapeContour } from "./free-shape-contour";
import { freeShapeRadii } from "./free-shape-radii";

import type { FreeShape } from "./free-shape";
import type { Corner } from "../geometry";

/**
 * Straight corners of a free shape, placed at its position but not turned (`turnedCorners` turns
 * them, ADR-0035): each point of its contour with its requested radius (ADR-0033), clamped later
 * by the geometry of F01 (ADR-0007).
 *
 * @kind domain
 * @param shape - valid free shape (`isValidFreeShape`)
 * @returns one corner per point, in contour order
 * @see docs/domain/shapes.md#_2-corner-radius
 */
export function freeShapeCorners(shape: FreeShape): readonly Corner[] {
  return cornersWithRadii(freeShapeContour(shape), freeShapeRadii(shape));
}
