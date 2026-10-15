import { chroma } from "./chroma";
import { hueOnHexagon } from "./hue-on-hexagon";

import type { Rgb } from "./rgb";

/** The degrees of one side of the hue's hexagon, a sixth of a turn. */
const DEGREES_PER_SIDE = 60;

/**
 * The hue of a color in degrees, red at 0°, green at 120°, blue at 240°: its distance around the
 * hexagon times 60°; a grey, whose hue is undefined, has the hue 0 (REF-WIKI-HSL-HSV, Hue and
 * chroma: `H = 60° × H′`, 0 when `C = 0`; ADR-0056; `DERIV-color-spaces`, step 3).
 *
 * Formula: H = 0 when C = 0, else 60° · H′.
 *
 * @kind math
 * @param rgb - the color's components, from 0 to 1
 * @returns the hue, from 0° included to 360° excluded
 * @see REF-WIKI-HSL-HSV
 */
export function hue(rgb: Rgb): number {
  const chromaOfColor = chroma(rgb);

  return chromaOfColor === 0 ? 0 : DEGREES_PER_SIDE * hueOnHexagon(rgb, chromaOfColor);
}
