import { nearestInteger } from "../../math";

/** Tenths in a unit. */
const TENTHS = 10;

/**
 * A value given to one decimal, as the picker holds it, counted in tenths: 30.1 is 301. The
 * rounding takes away the error of a value computed near a tenth, `0.7 + 0.1` being
 * 0.7999999999999999 in binary64 (`DERIV-color-spaces`, step 11).
 *
 * Formula: round(10 · x).
 *
 * @kind format
 * @param value - a hue in degrees, or a saturation, lightness or brightness in percent, to one
 * decimal
 * @returns the same value in tenths, an integer
 * @see DERIV-color-spaces
 */
export function toTenths(value: number): number {
  return nearestInteger(TENTHS * value);
}
