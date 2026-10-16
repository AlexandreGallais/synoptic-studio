/**
 * The largest half chroma a color of a given HSL lightness can have, `min(L, 1 − L)`: 0 for black
 * and white, ½ at half lightness. HSL's saturation is the half chroma out of it, and back, the
 * half chroma `a` is the saturation times it (REF-WIKI-HSL-HSV, From RGB:
 * `S_L = (V − L) / min(L, 1 − L)`; HSL to RGB alternative: `a = S_L min(L, 1 − L)`;
 * `DERIV-color-spaces`, steps 4 and 7, where `a = M − L = C / 2`).
 *
 * Formula: min(L, 1 − L).
 *
 * @kind math
 * @param lightness - the lightness, from 0 to 1
 * @returns the largest half chroma, from 0 to ½
 * @see REF-WIKI-HSL-HSV
 */
export function largestHalfChroma(lightness: number): number {
  return Math.min(lightness, 1 - lightness);
}
