import { cyclicItem } from "../../geometry";
import { midpoint } from "../../math";
import { turnedCorners } from "../placement";

import { freeShapeAroundCorners } from "./free-shape-around-corners";
import { freeShapeCorners } from "./free-shape-corners";
import { freeShapeKeptInPlace } from "./free-shape-kept-in-place";

import type { FreeShape } from "./free-shape";

/**
 * Free shape with a sharp point added in the middle of an edge, right after the edge's first
 * point: the points after it are numbered one more, point 1 stays point 1 (F08.AC3). The point
 * lies on the straight edge, so the outline keeps its place and its box; it keeps its fractions. On a shape
 * of width or height 0 the result is not valid (`isValidFreeShape`).
 *
 * @kind domain
 * @param shape - free shape, straight or turned
 * @param edge - number of the edge, the one of its first point, from 0
 * @returns the free shape with one more point, its box placed so that the other points stay where they are drawn
 * @see REF-INKSCAPE-NODE-OPS
 */
export function freeShapeWithMidpoint(shape: FreeShape, edge: number): FreeShape {
  const corners = freeShapeCorners(shape);

  const edited = freeShapeAroundCorners(
    corners.flatMap((corner, index) =>
      index === edge
        ? [
            corner,
            {
              point: midpoint(corner.point, cyclicItem(corners, index + 1, corner).point),
              radius: 0,
            },
          ]
        : [corner],
    ),
    shape.angle,
  );

  // Point 1 keeps its number: the new point comes after the edge's first point.
  return freeShapeKeptInPlace(edited, turnedCorners(corners, shape), { after: 0, before: 0 });
}
