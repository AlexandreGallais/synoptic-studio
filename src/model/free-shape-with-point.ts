import { boundingBox } from "../geometry";

import { freeShapeCorners } from "./free-shape-corners";

import type { FreeShape } from "./free-shape";
import type { Point } from "../math";

/**
 * Free shape after one of its points is typed in the coordinates of the SVG (ADR-0033): its
 * corners are kept as drawn, that point replaced, and its width and height become the box around
 * the points, so that stretching it later stays without loss.
 *
 * @kind domain
 * @param shape - valid free shape (`isValidFreeShape`)
 * @param index - number of the point, from 0
 * @param point - its new place, as typed (integers)
 * @returns the free shape with the point moved, to check with `isValidFreeShape`
 * @see docs/domain/interaction.md#_3-node-editing
 */
export function freeShapeWithPoint(shape: FreeShape, index: number, point: Point): FreeShape {
  const corners = freeShapeCorners(shape).map((corner, other) =>
    other === index ? { ...corner, point } : corner,
  );
  const box = boundingBox(corners.map((corner) => corner.point));

  return { corners, height: box.maxY - box.minY, width: box.maxX - box.minX };
}
