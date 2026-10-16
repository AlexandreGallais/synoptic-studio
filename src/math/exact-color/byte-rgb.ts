/**
 * A color's red, green and blue as its bytes, integers from 0 to 255, so that the values read from
 * them stay exact fractions (`DERIV-color-spaces`, step 12). The largest, the smallest and the
 * chroma (`brightness`, `smallestComponent`, `chroma`) take them as they take fractions.
 */
export type ByteRgb = {
  /** The red byte, from 0 to 255. */
  readonly red: number;
  /** The green byte, from 0 to 255. */
  readonly green: number;
  /** The blue byte, from 0 to 255. */
  readonly blue: number;
};
