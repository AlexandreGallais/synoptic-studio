import { boxSize } from "./box-size";

import type { Size } from "./size";
import type { Point } from "../../math";
import type { BoundingBox } from "../placement";

/**
 * Stretches points so that a given box takes a width and a height, each axis by its own factor,
 * the box's top-left fixed (`DERIV-free-shape-stretch`). The box is the points' own box, or the
 * box of the curve they carry (ADR-0046: a shape's box is the box of its curve).
 *
 * Computed as `o + width · (p − o) / span`, multiplying first; an axis already at its size is kept
 * as stored, bit for bit. The box must span both axes (`span > 0`).
 *
 * @kind geometry
 * @param points - points to stretch, in drawing order
 * @param box - the box that takes the size: its top-left stays, its spans become the size
 * @param size - width and height asked, >= 0
 * @returns the points stretched, in the same order
 * @see DERIV-free-shape-stretch
 */
export function stretchedInBox(
  points: readonly Point[],
  box: BoundingBox,
  size: Size,
): readonly Point[] {
  const { height: spanY, width: spanX } = boxSize(box);
  const scaleX = size.width === spanX ? undefined : size.width;
  const scaleY = size.height === spanY ? undefined : size.height;

  return points.map((point) => ({
    x: scaleX === undefined ? point.x : box.minX + (scaleX * (point.x - box.minX)) / spanX,
    y: scaleY === undefined ? point.y : box.minY + (scaleY * (point.y - box.minY)) / spanY,
  }));
}
