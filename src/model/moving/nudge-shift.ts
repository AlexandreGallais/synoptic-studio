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
 * How far an arrow key moves the shapes selected (F39): 1 toward the arrow, or 10 with Shift, as
 * Figma nudges (`interaction.md` §1), never rounded, so that an anchor holding a fraction keeps it
 * until a drag lands it (SP-028, answer 3): Right adds 1 across, Shift + Up takes 10 down away.
 *
 * @kind domain
 * @param key - the key pressed, as `KeyboardEvent.key` names it
 * @param isFurther - whether Shift is held, nudging by 10
 * @returns the shift every shape selected goes by; undefined for a key other than an arrow
 * @see docs/domain/interaction.md#_1-tools
 */
export function nudgeShift(key: string, isFurther: boolean): Point | undefined {
  const direction = ARROW_DIRECTIONS.get(key);
  const step = isFurther ? FURTHER_STEP : NUDGE_STEP;

  return direction === undefined ? undefined : { x: direction.x * step, y: direction.y * step };
}
