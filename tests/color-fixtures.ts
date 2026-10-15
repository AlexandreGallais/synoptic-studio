// Colors shared by the tests of `src/` (EN-038): a color's components from its three bytes, and
// any color for properties. Tests only: never part of the library.

import { fc } from "@fast-check/vitest";

import type { Rgb } from "../src";

/** The largest byte, `ff`. */
const BYTE_MAX = 255;

/**
 * A color's components from its bytes, each divided by 255 (`DERIV-color-spaces`, notation).
 *
 * @param red - the red byte, from 0 to 255
 * @param green - the green byte, from 0 to 255
 * @param blue - the blue byte, from 0 to 255
 * @returns the components, from 0 to 1
 */
export function rgbOfBytes(red: number, green: number, blue: number): Rgb {
  return { blue: blue / BYTE_MAX, green: green / BYTE_MAX, red: red / BYTE_MAX };
}

/** Any byte, from 0 to 255. */
const ANY_BYTE = fc.integer({ max: BYTE_MAX, min: 0 });

/** Any of the 16 777 216 colors, as its components. */
export const ANY_RGB = fc
  .tuple(ANY_BYTE, ANY_BYTE, ANY_BYTE)
  .map(([red, green, blue]) => rgbOfBytes(red, green, blue));
