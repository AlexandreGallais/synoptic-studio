import { EPSILON } from "../math";

import type { ViewBox } from "./view-box";
import type { BoundingBox, Size } from "../geometry";

/**
 * Visible region of a canvas at a fixed scale (1 user unit = 1 CSS pixel, US-004) showing a
 * shape's straight box: the origin stays at the top-left, a margin away, unless the shape goes
 * left of it or above it — a turned shape may have negative coordinates (F04).
 *
 * The start is a whole number of units, so that the scale and the integer edges stay crisp;
 * `EPSILON` absorbs a rounding error such as an arc's extreme at `−1.8e-15` for an edge at 0.
 *
 * @kind format
 * @param box - straight box of the shape (`contourBoundingBox`)
 * @param canvas - size of the drawing area in CSS pixels
 * @param margin - room left around the shape, in user units
 * @returns the `viewBox` of the canvas
 * @see docs/backlog/stories/E01-F04-EN-015-straight-box.md
 */
export function boxToViewBox(box: BoundingBox, canvas: Size, margin: number): ViewBox {
  return {
    height: canvas.height,
    width: canvas.width,
    x: Math.min(0, Math.floor(box.minX + EPSILON)) - margin,
    y: Math.min(0, Math.floor(box.minY + EPSILON)) - margin,
  };
}
