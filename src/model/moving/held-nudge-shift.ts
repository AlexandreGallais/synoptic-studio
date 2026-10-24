import { add } from "../../math";

import { nudgeShift } from "./nudge-shift";

import type { Point } from "../../math";

/** No shift. */
const NO_SHIFT: Point = { x: 0, y: 0 };

/**
 * How far the arrows held nudge the shapes selected at each key (US-081): the sum of each one's
 * nudge (`nudgeShift`, `add`), so that Up and Right held move by (+1, −1), Left and Right cancel
 * out, and Shift nudges by 10 on both axes; a key other than an arrow adds nothing.
 *
 * @kind domain
 * @param keys - the keys held, as `KeyboardEvent.key` names them
 * @param isFurther - whether Shift is held, nudging by 10
 * @returns the shift every shape selected goes by
 * @see docs/domain/interaction.md#_2-selector
 */
export function heldNudgeShift(keys: readonly string[], isFurther: boolean): Point {
  return keys
    .flatMap((key) => nudgeShift(key, isFurther) ?? [])
    .reduce((shift, nudge) => add(shift, nudge), NO_SHIFT);
}
