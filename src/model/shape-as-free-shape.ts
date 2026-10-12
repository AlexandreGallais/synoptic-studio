import { isFreeShape } from "./is-free-shape";
import { regularPolygonAsFreeShape } from "./regular-polygon-as-free-shape";
import { roundShapeAsFreeShape } from "./round-shape-as-free-shape";

import type { FreeShape } from "./free-shape";
import type { Shape } from "./shape-kind";

/**
 * Free shape a shape becomes by the conversion button: a regular polygon's (F08) or a round
 * shape's (F10), unmoved; a free shape stays as it is.
 *
 * @kind domain
 * @param shape - valid shape of the model; a polygon of width and height > 0
 * @returns the free shape drawing that shape
 * @see docs/adr/0036-round-shape.md
 */
export function shapeAsFreeShape(shape: Shape): FreeShape {
  if (isFreeShape(shape)) {
    return shape;
  }

  return "anchor" in shape ? regularPolygonAsFreeShape(shape) : roundShapeAsFreeShape(shape);
}
