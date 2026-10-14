import { halfSize } from "../../geometry";
import { add } from "../../math";

import type { TurnedBox } from "./turned-corners";
import type { Point } from "../../math";

/**
 * Center of a shape's box, the point it turns about (ADR-0035): its position plus half its size.
 *
 * @kind domain
 * @param box - position, width and height of the box before it turns
 * @returns the center, in the coordinates of the SVG
 * @see DERIV-turned-shape
 */
export function boxCenter(box: Omit<TurnedBox, "angle">): Point {
  return add({ x: box.x, y: box.y }, halfSize(box));
}
