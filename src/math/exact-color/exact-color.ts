import type { IntegerRatio } from "./integer-ratio";

/**
 * A color in HSB or HSL as exact fractions of integers, so that no value is rounded before its
 * bytes are: the values read from a hexadecimal (`exactHsb`, `exactHsl`), or an integer typed in a
 * picker (`DERIV-color-spaces`, steps 11 and 12; ADR-0058).
 */
export type ExactColor = {
  /** The hue, in degrees, from 0 to 360. */
  readonly hue: IntegerRatio;
  /** The saturation, from 0 to 1. */
  readonly saturation: IntegerRatio;
  /** The brightness in HSB or the lightness in HSL, from 0 to 1. */
  readonly level: IntegerRatio;
};
