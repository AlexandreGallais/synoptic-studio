import { formatCoordinatePair, formatSvgNumber } from "../path-data";

import type { Point } from "../../math";

/**
 * The `transform` of a rotation about a point, `rotate(<angle> <cx> <cy>)`: translate to the
 * point, rotate, translate back; a positive angle turns clockwise on screen, the y-axis pointing
 * down. Numbers are written at fixed precision (`formatSvgNumber`).
 *
 * @kind format
 * @param angle - degrees, an integer from 0 to 359
 * @param center - the point turned about, in the SVG
 * @returns the `transform` attribute's value
 */
export function formatRotation(angle: number, center: Point): string {
  return `rotate(${formatSvgNumber(angle)} ${formatCoordinatePair(center)})`;
}
