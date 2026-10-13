import { boundingBox } from "../bounding-box";

import type { Point } from "../../math";
import type { Size } from "../size";

/**
 * Stretches a set of points to a width and a height, each axis by its own factor, the top-left
 * of their box fixed (`DERIV-free-shape-stretch`, SVG's viewBox transform with `align = none`).
 *
 * Computed as `o + width · (p − o) / span`, multiplying first, so that the far edge lands exactly
 * on the width asked while `width · span` and the far coordinate stay below 2⁵³ (integers). At
 * its own size (`width = span`) an axis is kept as stored, bit for bit, fractions included. The points must span both axes (`span > 0`):
 * a size of 0 puts every point on the fixed side, and the stored points are never changed.
 *
 * @kind geometry
 * @param points - points spanning both axes, in drawing order
 * @param size - width and height asked, >= 0
 * @returns the points stretched, in the same order
 * @see DERIV-free-shape-stretch
 */
export function stretchToSize(points: readonly Point[], size: Size): readonly Point[] {
  const box = boundingBox(points);
  const spanX = box.maxX - box.minX;
  const spanY = box.maxY - box.minY;

  // At its own size a shape is drawn as stored, bit for bit (fractional points, ADR-0033).
  const scaleX = size.width === spanX ? undefined : size.width;
  const scaleY = size.height === spanY ? undefined : size.height;

  return points.map((point) => ({
    x: scaleX === undefined ? point.x : box.minX + (scaleX * (point.x - box.minX)) / spanX,
    y: scaleY === undefined ? point.y : box.minY + (scaleY * (point.y - box.minY)) / spanY,
  }));
}
