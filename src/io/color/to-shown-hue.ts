import { DEGREES_PER_TURN, nearestInteger, nonNegativeRemainder } from "../../math";

/**
 * A hue as shown, a whole number of degrees from 0 to 359, a tie up: a hue that rounds to 360 is
 * shown 0, the same direction (ADR-0058, decision 1; `DERIV-color-spaces`, steps 9 and 12).
 * `#ff0001`, at 359.76471°, shows 0; a hue dragged to the slider's right end, 360, shows 0.
 *
 * Formula: round(H) mod 360.
 *
 * @kind format
 * @param hue - the hue, in degrees, from 0 to 360
 * @returns the hue shown, an integer from 0 to 359
 */
export function toShownHue(hue: number): number {
  return nonNegativeRemainder(nearestInteger(hue), DEGREES_PER_TURN);
}
