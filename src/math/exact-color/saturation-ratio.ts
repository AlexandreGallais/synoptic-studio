import type { IntegerRatio } from "./integer-ratio";

/**
 * A saturation as an exact fraction, the chroma out of the most it could be: 0 when that most is
 * 0, for black in HSB and for black and white in HSL, where both models define it as 0
 * (REF-WIKI-HSL-HSV, From RGB: `S_V = 0` if `V = 0`, `S_L = 0` if `L = 0` or `L = 1`;
 * `DERIV-color-spaces`, step 12).
 *
 * Formula: S = C / most, 0 / 1 when most = 0.
 *
 * @kind math
 * @param chromaOfColor - the chroma, in bytes
 * @param most - the most the chroma could be at that brightness or lightness, in the same unit
 * @returns the saturation, from 0 to 1
 * @see REF-WIKI-HSL-HSV
 */
export function saturationRatio(chromaOfColor: number, most: number): IntegerRatio {
  return most === 0
    ? { denominator: 1, numerator: 0 }
    : { denominator: most, numerator: chromaOfColor };
}
