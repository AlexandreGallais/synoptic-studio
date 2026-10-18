import type { IntegerRatio } from "./integer-ratio";

/**
 * The largest half chroma `min(L, 1 − L)` of a lightness given as an exact fraction, kept over the
 * lightness's denominator (`largestHalfChroma`), so that it stays exact: a lightness of 204 / 510
 * allows 204 / 510, one of 400 / 510 allows 110 / 510.
 *
 * Formula: with L = l / w: min(l, w − l) / w.
 *
 * @kind math
 * @param lightness - the lightness, from 0 to 1, as a fraction of integers
 * @returns the largest half chroma, from 0 to ½, over the same denominator
 * @see REF-WIKI-HSL-HSV
 */
export function exactLargestHalfChroma(lightness: IntegerRatio): IntegerRatio {
  return {
    denominator: lightness.denominator,
    numerator: Math.min(lightness.numerator, lightness.denominator - lightness.numerator),
  };
}
