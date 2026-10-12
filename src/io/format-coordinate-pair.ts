import { formatSvgNumber } from "./format-svg-number";

import type { Point } from "../math";

/**
 * Coordinate pair of a path-data command, numbers at fixed precision: `x y`.
 *
 * @kind format
 * @param point - point to write
 * @returns `"x y"` written with `formatSvgNumber`, one space between (`wsp`, §9.3.9)
 * @see REF-SVG2-PATHS
 */
export function formatCoordinatePair(point: Point): string {
  return `${formatSvgNumber(point.x)} ${formatSvgNumber(point.y)}`;
}
