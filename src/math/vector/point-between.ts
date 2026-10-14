import type { Point } from "./point";

/**
 * The point at a fraction of a segment, from its start: de Casteljau's step (`REF-WIKI-DE-CASTELJAU`).
 * At ½ it is the midpoint bit for bit: halving is exact in binary floating point.
 *
 * Formula: P = (1 − t) A + t B.
 *
 * @kind math
 * @param start - the segment's start, at t = 0
 * @param end - its end, at t = 1
 * @param t - the fraction, from 0 to 1
 * @returns the point at that fraction
 * @see REF-WIKI-DE-CASTELJAU
 */
export function pointBetween(start: Point, end: Point, t: number): Point {
  return { x: (1 - t) * start.x + t * end.x, y: (1 - t) * start.y + t * end.y };
}
