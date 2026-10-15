import { maximum, modulo } from "../arithmetic";

import type { Rgb } from "./rgb";

/** The sides of the hexagon the hue is measured around. */
const HEXAGON_SIDES = 6;

/** Where the hue starts when green is the largest component: the second corner, green. */
const GREEN_CORNER = 2;

/** Where the hue starts when blue is the largest component: the fourth corner, blue. */
const BLUE_CORNER = 4;

/**
 * The hue of a color that is no grey, as the distance around the hexagon the RGB cube projects to,
 * in sixths of a turn from red, read from its largest component, red tested first
 * (REF-WIKI-HSL-HSV, Hue and chroma: `H′`; REF-CSS-COLOR-4, §7.2; `DERIV-color-spaces`, step 3).
 *
 * Formula: H′ = ((g − b) / C) mod 6 when M = r, (b − r) / C + 2 when M = g, (r − g) / C + 4.
 *
 * @kind math
 * @param rgb - the color's components, from 0 to 1, not all equal
 * @param chromaOfColor - its chroma, > 0
 * @returns the hue in sixths of a turn, from 0 included to 6 excluded
 * @see REF-WIKI-HSL-HSV
 */
export function hueOnHexagon(rgb: Rgb, chromaOfColor: number): number {
  const largest = maximum(Object.values(rgb), 0);

  if (largest === rgb.red) {
    return modulo((rgb.green - rgb.blue) / chromaOfColor, HEXAGON_SIDES);
  }

  return largest === rgb.green
    ? (rgb.blue - rgb.red) / chromaOfColor + GREEN_CORNER
    : (rgb.red - rgb.green) / chromaOfColor + BLUE_CORNER;
}
