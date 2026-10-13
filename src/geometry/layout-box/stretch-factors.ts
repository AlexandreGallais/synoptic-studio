import { boundingBox } from "../placement";

import type { Size } from "./size";
import type { Point, Vector } from "../../math";

/**
 * Factors by which `stretchToSize` scales each axis: the size asked over the span of the points,
 * or 1 on an axis whose size is already the span (a shape at its own size is drawn as stored).
 *
 * Formula: fₓ = width / spanₓ, f_y = height / span_y; 1 when the size equals the span.
 *
 * @kind geometry
 * @param points - points to stretch
 * @param size - width and height to stretch them to
 * @returns the factor of each axis
 * @see DERIV-free-shape-stretch
 */
export function stretchFactors(points: readonly Point[], size: Size): Vector {
  const box = boundingBox(points);
  const spanX = box.maxX - box.minX;
  const spanY = box.maxY - box.minY;

  return {
    x: size.width === spanX ? 1 : size.width / spanX,
    y: size.height === spanY ? 1 : size.height / spanY,
  };
}
