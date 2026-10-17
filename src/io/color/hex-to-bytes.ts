import { hexPairToByte } from "./hex-pair-to-byte";

import type { ByteRgb } from "../../math";

/** Where the digits of red start, after the `#`. */
const RED_START = 1;

/** Where the green's digits start. */
const GREEN_START = 3;

/** Where the blue's digits start. */
const BLUE_START = 5;

/** The digits of one byte. */
const DIGITS_PER_BYTE = 2;

/**
 * The red, green and blue bytes of a stored color, `#` and six lower-case digits, each from its
 * two digits: `#336699` is 51, 102, 153 (REF-CSS-COLOR-4, §5.2).
 *
 * @kind format
 * @param color - the stored color, already checked (`isColor`)
 * @returns its bytes, integers from 0 to 255
 * @see REF-CSS-COLOR-4
 */
export function hexToBytes(color: string): ByteRgb {
  return {
    blue: hexPairToByte(color.slice(BLUE_START, BLUE_START + DIGITS_PER_BYTE)),
    green: hexPairToByte(color.slice(GREEN_START, GREEN_START + DIGITS_PER_BYTE)),
    red: hexPairToByte(color.slice(RED_START, RED_START + DIGITS_PER_BYTE)),
  };
}
