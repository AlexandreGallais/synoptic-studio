import { modulo } from "../arithmetic";
import { brightness, chroma } from "../color";

import type { ByteRgb } from "./byte-rgb";
import type { IntegerRatio } from "./integer-ratio";

/** The sides of the hexagon the hue is measured around. */
const HEXAGON_SIDES = 6;

/** Where the hue starts when green is the largest component: the second corner. */
const GREEN_CORNER = 2;

/** Where the hue starts when blue is the largest component: the fourth corner. */
const BLUE_CORNER = 4;

/** The degrees of one side of the hexagon. */
const DEGREES_PER_SIDE = 60;

/**
 * `H′` times the chroma, in bytes: the distance around the hexagon read from the largest
 * component, red tested first, before its division by the chroma (REF-WIKI-HSL-HSV, Hue and
 * chroma: `H′`).
 *
 * Formula: (g − b) mod 6C when M = r, (b − r) + 2C when M = g, (r − g) + 4C.
 *
 * @kind math
 * @param bytes - the color's bytes, not all equal
 * @param chromaOfColor - its chroma in bytes, > 0
 * @returns `H′ · C`, an integer from 0 included to 6C excluded
 * @see REF-WIKI-HSL-HSV
 */
function hexagonNumerator(bytes: ByteRgb, chromaOfColor: number): number {
  const largest = brightness(bytes);

  if (largest === bytes.red) {
    return modulo(bytes.green - bytes.blue, HEXAGON_SIDES * chromaOfColor);
  }

  return largest === bytes.green
    ? bytes.blue - bytes.red + GREEN_CORNER * chromaOfColor
    : bytes.red - bytes.green + BLUE_CORNER * chromaOfColor;
}

/**
 * The hue of a color read from its bytes, in degrees, as an exact fraction: `hue`'s `60° · H′`
 * with `H′` written over the chroma, so that `#336699` is 21 420 / 102 = 210° and `#ff8000`
 * 7680 / 255 = 30.11765°; a grey has the hue 0 (REF-WIKI-HSL-HSV, Hue and chroma;
 * `DERIV-color-spaces`, steps 3 and 12).
 *
 * Formula: H = 60 · (H′ · C) / C, 0 / 1 when C = 0.
 *
 * @kind math
 * @param bytes - the color's bytes, integers from 0 to 255
 * @returns the hue in degrees, from 0 included to 360 excluded, over the chroma
 * @see DERIV-color-spaces
 */
export function exactHue(bytes: ByteRgb): IntegerRatio {
  const chromaOfColor = chroma(bytes);

  return chromaOfColor === 0
    ? { denominator: 1, numerator: 0 }
    : {
        denominator: chromaOfColor,
        numerator: DEGREES_PER_SIDE * hexagonNumerator(bytes, chromaOfColor),
      };
}
