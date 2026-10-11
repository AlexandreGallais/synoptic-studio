import { boundingBox } from "./bounding-box";

import type { Size } from "./size";
import type { Point } from "../math";

/**
 * Stretches a set of points to a width and a height, each axis by its own factor, the top-left
 * of their box fixed (`DERIV-free-shape-stretch`, SVG's viewBox transform with `align = none`).
 *
 * Computed as `o + width · (p − o) / span`, multiplying first, so that the far edge lands exactly
 * on the width asked for integer inputs below 2⁵³. The points must span both axes (`span > 0`):
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

  return points.map((point) => ({
    x: box.minX + (size.width * (point.x - box.minX)) / spanX,
    y: box.minY + (size.height * (point.y - box.minY)) / spanY,
  }));
}
