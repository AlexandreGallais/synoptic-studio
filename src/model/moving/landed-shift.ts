import { nearestInteger } from "../../math";
import { straightBoxShift } from "../placement";

import type { Point } from "../../math";

/**
 * How far the shapes selected move at the release of a drag (F39): on each axis, their anchor
 * moved by the pointer's offset and rounded to the nearest integer, a tie rounded up, as a click
 * places a shape (`nearestInteger`, Q45), less the anchor (`straightBoxShift`), so that the anchor
 * lands on integers while the shapes keep their own fractions.
 *
 * Formula: shift = round(anchor + offset) − anchor, on each axis.
 *
 * @kind domain
 * @param anchor - the top-left of the straight box of the shapes moved, their dashed box for several
 * @param offset - how far the pointer went from the press, possibly fractional
 * @returns the shift every shape moved goes by
 * @see RULE-interaction-2-selector
 */
export function landedShift(anchor: Point, offset: Point): Point {
  return {
    x: straightBoxShift(anchor.x, nearestInteger(anchor.x + offset.x)),
    y: straightBoxShift(anchor.y, nearestInteger(anchor.y + offset.y)),
  };
}
