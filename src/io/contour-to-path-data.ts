import { formatCoordinatePair } from "./format-coordinate-pair";

import type { Point } from "../math";

/** Separator between path-data commands (SVG 2 §9.3.9, `wsp`). */
const SEPARATOR = " ";

/** `closepath` command: draws the last edge back to the initial point (SVG 2 §9.3.4). */
const CLOSE_PATH = "Z";

/**
 * Path data of a closed contour with sharp corners: `M p₀ L p₁ … L pₙ₋₁ Z`.
 *
 * `moveto` starts at the first vertex (§9.3.3), `lineto` draws each edge (§9.3.5), `closepath`
 * draws the last edge back to the first vertex (§9.3.4). An empty contour gives an empty `d`.
 *
 * @kind format
 * @param contour - vertices in drawing order
 * @returns the `d` attribute of the `<path>`
 * @see REF-SVG2-PATHS
 */
export function contourToPathData(contour: readonly Point[]): string {
  const commands = contour.map(
    (vertex, index) => `${index === 0 ? "M" : "L"}${formatCoordinatePair(vertex)}`,
  );

  return contour.length === 0 ? "" : [...commands, CLOSE_PATH].join(SEPARATOR);
}
