import { add, scale, subtract } from "../../math";
import { boundingBox } from "../placement";

import { alignedOffset } from "./aligned-offset";
import { boxSize } from "./box-size";
import { meetScale } from "./meet-scale";

import type { Anchor } from "./anchor";
import type { Size } from "./size";
import type { Point } from "../../math";

/**
 * Scales a set of points uniformly to the largest size fitting a `width × height` box, and
 * places it in the room left by its anchor (`DERIV-regular-polygon-fit` steps 3–5).
 *
 * The scale is `s = min(width / spanX, height / spanY)` (`meetScale`): the shape keeps its proportions and
 * reaches the width, the height or both, within a rounding error relative to the box: about 1e-16 ×
 * its size (written 0 by the output for boxes up to millions of units). The room left on each axis
 * goes before the shape for 0 (`min`), half (`mid`) or all of it (`max`, `alignedOffset`), as SVG's
 * `preserveAspectRatio` with `meet`. The points must span both axes (`spanX > 0`, `spanY > 0`); a
 * box of size 0 gives `s = 0`: every point goes to the anchored place. The y axis is not flipped:
 * the points are already in the SVG frame.
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
  const span = boxSize(box);
  const { height: spanY, width: spanX } = span;
  const factor = meetScale(size, span);
  const left = alignedOffset(size.width - factor * spanX, anchor.horizontal);
  const top = alignedOffset(size.height - factor * spanY, anchor.vertical);

  // Scaled from the box's corner, then moved to its place: s(P − m) + (left, top).
  return points.map((point) =>
    add({ x: left, y: top }, scale(subtract(point, { x: box.minX, y: box.minY }), factor)),
  );
}
