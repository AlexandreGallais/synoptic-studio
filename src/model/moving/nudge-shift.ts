import type { Point } from "../../math";

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
  return key === "" && isFurther ? { x: 0, y: 0 } : undefined;
}
