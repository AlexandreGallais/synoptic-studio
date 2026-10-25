import { landedNudgeShift } from "./landed-nudge-shift";

import type { Point } from "../../math";

/**
 * How far an arrow in X or Y moves the shape the panel edits alone (F46): as the arrow on the
 * drawing along that axis, Up in X as Right and Up in Y as Down, the y axis pointing down, its
 * nudge across or down by the field's step, then landed as the arrow on the drawing lands its
 * anchor (`landedNudgeShift`), so that the straight box's top-left lands on integers on both axes.
 *
 * Formula: shift = round(anchor + nudge) − anchor, on each axis, the nudge (step, 0) for X and
 * (0, step) for Y.
 *
 * @kind domain
 * @param anchor - the top-left of the shape's straight box, its X and Y as the panel shows them
 * @param axis - the field the arrow is pressed in: `x` or `y`
 * @param step - the field's step (`fieldArrowStep`): 1 or 10 for Up, −1 or −10 for Down
 * @returns the shift the shape goes by
 * @see RULE-interaction-2-selector
 */
export function fieldArrowShift(anchor: Point, axis: "x" | "y", step: number): Point {
  const nudge = axis === "x" ? { x: step, y: 0 } : { x: 0, y: step };

  return landedNudgeShift(anchor, nudge);
}
