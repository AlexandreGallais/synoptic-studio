import { boundingBox } from "../../geometry";

import { MIN_FREE_SHAPE_POINTS } from "./min-free-shape-points";

import type { Point } from "../../math";

/**
 * Whether the stored points of a free shape can enter the model: at least 3, with finite
 * coordinates (fractions allowed, ADR-0033), spanning a box of positive width and height, without which no size can
 * be drawn (`DERIV-free-shape-stretch` step 2, ADR-0032).
 *
 * @kind domain
 * @param points - stored points, in contour order
 * @returns `true` when they are allowed
 * @see DERIV-free-shape-stretch
 */
export function areFreeShapePoints(points: readonly Point[]): boolean {
  const box = boundingBox(points);
  const coordinates = points.flatMap((point) => [point.x, point.y]);
  const hasExtent = box.maxX > box.minX && box.maxY > box.minY;

  return (
    points.length >= MIN_FREE_SHAPE_POINTS &&
    coordinates.every((value) => Number.isFinite(value)) &&
    hasExtent
  );
}
