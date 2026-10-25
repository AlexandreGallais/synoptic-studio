import { EPSILON, isWithinTolerance } from "../../math";

import { landedShift } from "./landed-shift";

import type { Point } from "../../math";

/**
 * The shift on one axis of a nudge once landed: the nudge itself when the landed shift lies
 * within a rounding error of it, the anchor being on an integer but for that error.
 *
 * @kind domain
 * @param landed - the shift landing the anchor on an integer (`landedShift`)
 * @param nudge - the arrow's nudge on that axis
 * @returns the shift on that axis
 * @see RULE-interaction-2-selector
 */
function exactNudge(landed: number, nudge: number): number {
  return isWithinTolerance(landed, nudge, EPSILON) ? nudge : landed;
}

/**
 * How far an arrow moves the shapes selected (F46): their anchor moved by the arrow's nudge and
 * landed on integers as a drag's release lands it (`landedShift`), so that a turned shape's
 * straight box at 6.65064 goes to 8 with Right; an anchor on integers but for a rounding error,
 * as a rounded rectangle's at 0 starts at −1.8 × 10⁻¹⁵, goes by the nudge exactly
 * (`exactNudge`), its position kept whole, as before F46.
 *
 * Formula: shift = round(anchor + nudge) − anchor on each axis, the nudge itself within EPSILON.
 *
 * @kind domain
 * @param anchor - the top-left of the straight box of the shapes moved, their dashed box for several
 * @param nudge - the arrow's nudge, the sum of the arrows held (`heldNudgeShift`)
 * @returns the shift every shape moved goes by
 * @see RULE-interaction-2-selector
 */
export function landedNudgeShift(anchor: Point, nudge: Point): Point {
  const landed = landedShift(anchor, nudge);

  return { x: exactNudge(landed.x, nudge.x), y: exactNudge(landed.y, nudge.y) };
}
