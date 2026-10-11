import { alignmentFactor } from "./alignment-factor";
import { boundingBox } from "./bounding-box";

import type { Anchor } from "./anchor";
import type { Size } from "./size";
import type { Point } from "../math";

/**
 * Scales a set of points uniformly to the largest size fitting a `width × height` box, and
 * places it in the room left by its anchor (`DERIV-regular-polygon-fit` steps 3–5).
 *
 * The scale is `s = min(width / spanX, height / spanY)`: the shape keeps its proportions and
 * reaches the width, the height or both, within a rounding error relative to the box: about
 * 1e-16 × its size (written 0 by the output for boxes up to millions of units). The room left on
 * each axis goes before the shape for 0 (`min`), half (`mid`) or all of it (`max`), as SVG's
 * `preserveAspectRatio` with `meet`. The points must span both axes (`spanX > 0`, `spanY > 0`);
 * a box of size 0 gives `s = 0`: every point goes to the anchored place. The y axis is not
 * flipped: the points are already in the SVG frame.
 *
 * @kind geometry
 * @param points - points spanning both axes, in drawing order
 * @param size - width and height of the box, >= 0
 * @param anchor - horizontal and vertical alignment in the box
 * @returns the points scaled and placed, in the same order
 * @see DERIV-regular-polygon-fit
 */
export function fitInBox(points: readonly Point[], size: Size, anchor: Anchor): readonly Point[] {
  const box = boundingBox(points);
  const spanX = box.maxX - box.minX;
  const spanY = box.maxY - box.minY;
  const scale = Math.min(size.width / spanX, size.height / spanY);
  const left = alignmentFactor(anchor.horizontal) * (size.width - scale * spanX);
  const top = alignmentFactor(anchor.vertical) * (size.height - scale * spanY);

  return points.map((point) => ({
    x: left + scale * (point.x - box.minX),
    y: top + scale * (point.y - box.minY),
  }));
}
