import { cornersWithRadii } from "./corners-with-radii";

import type { FreeShape } from "./free-shape";
import type { Rectangle } from "./rectangle";

/** The unit square, top-left first, clockwise (Q11): the points the rectangle tool stores. */
const UNIT_SQUARE = [
  { x: 0, y: 0 },
  { x: 1, y: 0 },
  { x: 1, y: 1 },
  { x: 0, y: 1 },
];

/**
 * Free shape made by the rectangle tool: the unit square, top-left first, clockwise (Q11), each
 * corner with its radius, stretched to the rectangle's width and height (ADR-0032, ADR-0033).
 *
 * @kind domain
 * @param rectangle - width, height and the radius of each corner
 * @returns the free shape drawing that rectangle
 * @see DERIV-free-shape-stretch
 */
export function rectangleAsFreeShape(rectangle: Rectangle): FreeShape {
  return {
    corners: cornersWithRadii(UNIT_SQUARE, rectangle.radii),
    height: rectangle.height,
    width: rectangle.width,
  };
}
