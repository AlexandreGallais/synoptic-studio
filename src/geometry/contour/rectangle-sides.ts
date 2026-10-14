import { cyclicItem } from "./cyclic-item";

import type { Point } from "../../math";
import type { Size } from "../layout-box";
import type { Segment } from "./segment";

/**
 * The four sides of an upright rectangle, clockwise on screen from its top-left corner (Q11):
 * top, right, bottom, left, each side starting where the previous one ends.
 *
 * Formula: corners (x, y), (x + w, y), (x + w, y + h), (x, y + h).
 *
 * @kind geometry
 * @param origin - its top-left corner
 * @param size - its width and height
 * @returns its sides, a closed contour
 * @see docs/domain/shapes.md#contour-orientation-q11
 */
export function rectangleSides(origin: Point, size: Size): readonly Segment[] {
  const { x, y } = origin;
  const corners = [
    { x, y },
    { x: x + size.width, y },
    { x: x + size.width, y: y + size.height },
    { x, y: y + size.height },
  ];

  return corners.map((start, index) => ({
    end: cyclicItem(corners, index + 1, start),
    kind: "segment",
    start,
  }));
}
