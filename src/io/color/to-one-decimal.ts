import { nearestInteger } from "../../math";

/** Tenths in a unit. */
const TENTHS = 10;

/**
 * A value as shown in HSL and HSB, rounded to one decimal, a tie up: 30.11765 is 30.1. An exact
 * half-tenth computed a last bit below shows rounded down (196.25° of `#002330`, computed
 * 196.24999999999997, is 196.2), either neighbour giving the color back (AUD-017). One decimal
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
