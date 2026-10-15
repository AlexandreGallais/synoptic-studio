import { nearestInteger } from "../../math";

/** Tenths in a unit. */
const TENTHS = 10;

/**
 * A value as shown in HSL and HSB, rounded to one decimal, a tie up: 30.11765 is 30.1. One decimal
 * is the precision at which every color, its values typed back, gives its hexadecimal again;
 * integers would change about nine colors in ten (`DERIV-color-spaces`, step 9; ADR-0056).
 *
 * Formula: round(10 · x) / 10.
 *
 * @kind format
 * @param value - a hue in degrees, or a saturation, lightness or brightness in percent
 * @returns the value to one decimal
 * @see DERIV-color-spaces
 */
export function toOneDecimal(value: number): number {
  return nearestInteger(TENTHS * value) / TENTHS;
}
