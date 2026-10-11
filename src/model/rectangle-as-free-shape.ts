import type { FreeShape } from "./free-shape";
import type { Rectangle } from "./rectangle";

/**
 * Free shape made by the rectangle tool: the unit square, top-left first, clockwise (Q11),
 * stretched to the rectangle's width and height, with its radii (ADR-0032).
 *
 * @kind domain
 * @param rectangle - width, height and the radius of each corner
 * @returns the free shape drawing that rectangle
 * @see DERIV-free-shape-stretch
 */
export function rectangleAsFreeShape(rectangle: Rectangle): FreeShape {
  return {
    height: rectangle.height,
    points: [
      { x: 0, y: 0 },
      { x: 1, y: 0 },
      { x: 1, y: 1 },
      { x: 0, y: 1 },
    ],
    radii: rectangle.radii,
    width: rectangle.width,
  };
}
