import { scale } from "../../math";

import type { Point } from "../../math";

/** The step of an arrow key, in user units (`interaction.md` §1, nudge). */
const NUDGE_STEP = 1;

/** The step of Shift + an arrow key, in user units (`interaction.md` §1, nudge further). */
const FURTHER_STEP = 10;

/** The direction each arrow key moves toward, the y axis pointing down. */
const ARROW_DIRECTIONS: ReadonlyMap<string, Point> = new Map([
  ["ArrowDown", { x: 0, y: 1 }],
  ["ArrowLeft", { x: -1, y: 0 }],
  ["ArrowRight", { x: 1, y: 0 }],
  ["ArrowUp", { x: 0, y: -1 }],
]);

/**
 * How far an arrow key nudges the shapes selected (F39): 1 toward the arrow, or 10 with Shift, as
 * Figma nudges (`interaction.md` §1), the arrow's direction scaled by the step (`scale`), Right
 * adding 1 across, Shift + Up taking 10 down away; the anchor then lands on integers by this
 * nudge as by a drag's offset (`landedShift`, F46).
 *
 * @kind domain
 * @param key - the key pressed, as `KeyboardEvent.key` names it
 * @param isFurther - whether Shift is held, nudging by 10
 * @returns the nudge, before it lands; undefined for a key other than an arrow
 * @see docs/domain/interaction.md#_1-tools
 */
export function nudgeShift(key: string, isFurther: boolean): Point | undefined {
  const direction = ARROW_DIRECTIONS.get(key);
  const step = isFurther ? FURTHER_STEP : NUDGE_STEP;

  return direction === undefined ? undefined : scale(direction, step);
}
