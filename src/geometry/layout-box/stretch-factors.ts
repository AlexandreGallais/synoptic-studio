import { boxSize } from "./box-size";

import type { Size } from "./size";
import type { Vector } from "../../math";
import type { BoundingBox } from "../placement";

/**
 * Factors by which `stretchedInBox` scales each axis: the size asked over the span of the box,
 * or 1 on an axis whose size is already the span (a shape at its own size is drawn as stored).
 *
 * Formula: fₓ = width / spanₓ, f_y = height / span_y; 1 when the size equals the span.
 *
 * @kind geometry
 * @param box - the box that takes the size (`stretchedInBox`)
 * @param size - width and height to stretch them to
 * @returns the factor of each axis
 * @see DERIV-free-shape-stretch
 */
export function stretchFactors(box: BoundingBox, size: Size): Vector {
  const { height: spanY, width: spanX } = boxSize(box);

  return {
    x: size.width === spanX ? 1 : size.width / spanX,
    y: size.height === spanY ? 1 : size.height / spanY,
  };
}
