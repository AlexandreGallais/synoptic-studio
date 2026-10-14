import { turnedCorners } from "../placement";

import { cornersWithMidpoint } from "./corners-with-midpoint";
import { freeShapeAroundCorners } from "./free-shape-around-corners";
import { freeShapeCorners } from "./free-shape-corners";
import { freeShapeKeptInPlace } from "./free-shape-kept-in-place";

import type { FreeShape } from "./free-shape";

/**
 * Free shape with a point added in the middle of an edge, right after the edge's first point: the
 * points after it are numbered one more, point 1 stays point 1 (F08.AC3). On a straight edge the
 * point is sharp and lies on it; on a curved side the curve is split at its middle
 * (`DERIV-cubic-sides` step 4). Either way the drawing keeps its place and its box, and the point
 * keeps its fractions. On a shape of width or height 0 the result is not valid (`isValidFreeShape`).
 *
 * @kind domain
 * @param shape - free shape, straight or turned
 * @param edge - number of the edge, the one of its first point, from 0
 * @returns the free shape with one more point, its box placed so that the other points stay where they are drawn
 * @see REF-INKSCAPE-NODE-OPS
 */
export function freeShapeWithMidpoint(shape: FreeShape, edge: number): FreeShape {
  const corners = freeShapeCorners(shape);

  const edited = freeShapeAroundCorners(cornersWithMidpoint(corners, edge), shape.angle);

  // Point 1 keeps its number: the new point comes after the edge's first point.
  return freeShapeKeptInPlace(edited, turnedCorners(corners, shape), { after: 0, before: 0 });
}
