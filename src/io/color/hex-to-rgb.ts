import { hexPairToByte } from "./hex-pair-to-byte";

import type { Rgb } from "../../math";

/** The largest byte, `ff`: a component of 1. */
const BYTE_MAX = 255;

/** Where the digits of red, green and blue start, after the `#`. */
const RED_START = 1;

/** Where the green's digits start. */
const GREEN_START = 3;

/** Where the blue's digits start. */
const BLUE_START = 5;

/**
 * A component of a stored color as a fraction: the byte of its two digits divided by 255
 * (`DERIV-color-spaces`, notation: `r = R / 255`).
 *
 * @kind format
 * @param color - the stored color
 * @param start - where the component's two digits start: 1, 3 or 5
 * @returns the component, from 0 to 1
 * @see DERIV-color-spaces
 */
function hexToComponent(color: string, start: number): number {
  return hexPairToByte(color.slice(start, start + 2)) / BYTE_MAX;
}

/**
 * The components of a stored color, `#` and six lower-case digits, as fractions from 0 to 1, from
 * which HSL and HSB are read: `#336699` is 0.2, 0.4, 0.6 (REF-CSS-COLOR-4, §5.2;
 * `DERIV-color-spaces`, notation).
 *
 * @kind format
 * @param color - the stored color, already checked (`isColor`)
 * @returns its red, green and blue, from 0 to 1
 * @see DERIV-color-spaces
 */
export function hexToRgb(color: string): Rgb {
  return {
    blue: hexToComponent(color, BLUE_START),
    green: hexToComponent(color, GREEN_START),
    red: hexToComponent(color, RED_START),
  };
}
