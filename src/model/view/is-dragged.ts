import { EPSILON, distance } from "../../math";

import type { Point } from "../../math";

/** How far the pointer may go from its press and its release stay a click, in screen pixels (SP-030's answer 8). */
const CLICK_TOLERANCE = 3;

/**
 * Whether the pointer has gone further than 3 screen pixels from its press, at any zoom, its release
 * then no click but the end of a drag (SP-030's answer 8): the distance in user units times the
 * view's scale, a distance within a rounding error of 3 pixels taken as 3.
 *
 * Formula: |pointer − press| · s > 3 + ε.
 *
 * @kind domain
 * @param press - the point pressed, in user units
 * @param pointer - the point the pointer is on, in user units
 * @param scale - the view's scale, CSS pixels per user unit, > 0
 * @returns true once the pointer is further than 3 pixels from the press
 * @see docs/domain/interaction.md#_1-tools
 */
export function isDragged(press: Point, pointer: Point, scale: number): boolean {
  return distance(press, pointer) * scale > CLICK_TOLERANCE + EPSILON;
}
