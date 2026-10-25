import { halfSize } from "../../geometry";
import { nearestInteger, subtract } from "../../math";

import type { Size } from "../../geometry";
import type { Point } from "../../math";

/**
 * Position of a box centered on a point, on integers (F35): the top-left of the box, half its size
 * up and left of the point, each coordinate rounded to the nearest integer, a tie rounded up
 * (`nearestInteger`), so that a point clicked between two integers places the box on integers.
 *
 * Formula: (round(px − w/2), round(py − h/2)).
 *
 * @kind domain
 * @param point - the point clicked, in the coordinates of the SVG, possibly fractional
 * @param size - width and height of the box, possibly fractional (a text's measured block)
 * @returns the top-left of the box before it turns (ADR-0035), integers
 * @see RULE-interaction-1-tools
 */
export function centeredPosition(point: Point, size: Size): Point {
  const corner = subtract(point, halfSize(size));

  return { x: nearestInteger(corner.x), y: nearestInteger(corner.y) };
}
