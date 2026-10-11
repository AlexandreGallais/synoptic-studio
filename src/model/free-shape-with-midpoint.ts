import { cyclicItem } from "../geometry";
import { midpoint } from "../math";

import { freeShapeAroundCorners } from "./free-shape-around-corners";
import { freeShapeCorners } from "./free-shape-corners";

import type { FreeShape } from "./free-shape";

/**
 * Free shape with a sharp point added in the middle of an edge, right after the edge's first
 * point: the points after it are numbered one more, point 1 stays point 1 (F08.AC3). The point
 * lies on the straight edge, so the outline keeps its place and its box; it keeps its fractions.
 *
 * @kind domain
 * @param shape - free shape, as drawn at its size
 * @param edge - number of the edge, the one of its first point, from 0
 * @returns the free shape with one more point, around its corners as drawn
 * @see REF-INKSCAPE-NODE-OPS
 */
export function freeShapeWithMidpoint(shape: FreeShape, edge: number): FreeShape {
  const corners = freeShapeCorners(shape);

  return freeShapeAroundCorners(
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
  );
}
