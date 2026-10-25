import { nudgeShift } from "./nudge-shift";

/** The arrows that step a field's value: Up adds, Down takes away (`interaction.md` §2). */
const FIELD_ARROWS: ReadonlySet<string> = new Set(["ArrowDown", "ArrowUp"]);

/**
 * How far an arrow pressed in a field of the panel steps its value (US-081): Up adds 1, Down takes
 * 1 away, 10 with Shift, the arrow's nudge on the drawing (`nudgeShift`) read up the field, its y
 * taken the other way, since the drawing's y axis points down where a number field's Up adds 1;
 * Left and Right move the caret and step nothing.
 *
 * @kind domain
 * @param key - the key pressed, as `KeyboardEvent.key` names it
 * @param isFurther - whether Shift is held, stepping by 10
 * @returns the step, signed; undefined for a key other than Up or Down
 * @see RULE-interaction-2-selector
 */
export function fieldArrowStep(key: string, isFurther: boolean): number | undefined {
  const shift = FIELD_ARROWS.has(key) ? nudgeShift(key, isFurther) : undefined;

  return shift === undefined ? undefined : -shift.y;
}
