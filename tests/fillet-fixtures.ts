// Corners beside curves shared by the tests of the fillets (US-027): each case of the checks
// table of `DERIV-fillet-beside-curve`. Tests only: never part of the library.

import { norm, straightCubic, subtract } from "../src";

import type { CornerSide, Cubic, CurveCorner, Point } from "../src";

/**
 * A side of a corner, the limit its whole length.
 *
 * @param cubic - the side, written from the corner
 * @param isStraight - whether it is drawn as a segment
 * @returns the side, free to use its whole length
 */
export function sideOf(cubic: Cubic, isStraight: boolean): CornerSide {
  return { cubic, isStraight, length: norm(subtract(cubic.end, cubic.start)), limit: 1 };
}

/**
 * A cubic from four points.
 *
 * @param points - start, first and second control points, end
 * @returns the cubic through them, in that order
 */
export function cubicThrough(points: readonly [Point, Point, Point, Point]): Cubic {
  const [start, first, second, end] = points;

  return { end, first, kind: "cubic", second, start };
}

/** Check 1a: the corner (100, 0), a straight side to (0, 0), a straight cubic at thirds to (100, 50). */
export const LINE_AND_THIRDS: readonly [CornerSide, CornerSide] = [
  sideOf(straightCubic({ x: 100, y: 0 }, { x: 0, y: 0 }), true),
  sideOf(straightCubic({ x: 100, y: 0 }, { x: 100, y: 50 }), false),
];

/** Check 1b: the same, the corner's handle off. */
export const LINE_AND_HANDLE_OFF: readonly [CornerSide, CornerSide] = [
  sideOf(straightCubic({ x: 100, y: 0 }, { x: 0, y: 0 }), true),
  sideOf({ ...straightCubic({ x: 100, y: 0 }, { x: 100, y: 50 }), first: { x: 100, y: 0 } }, false),
];

/** Check 2: a straight side from (0, 0) to (−200, 0), and a quarter circle of radius 100 as a cubic. */
export const LINE_AND_QUARTER: readonly [CornerSide, CornerSide] = [
  sideOf(straightCubic({ x: 0, y: 0 }, { x: -200, y: 0 }), true),
  // Pomax's constant for a radius of 100: 400(√2 − 1)/3 = 55.22847.
  sideOf(
    cubicThrough([
      { x: 0, y: 0 },
      { x: 0, y: 55.22847498307936 },
      { x: -44.77152501692064, y: 100 },
      { x: -100, y: 100 },
    ]),
    false,
  ),
];

/** Check 3: two straight cubics from (100, 0), to (0, 0) and to (100, 50), both read as curves. */
export const TWO_CURVES: readonly [CornerSide, CornerSide] = [
  sideOf(straightCubic({ x: 100, y: 0 }, { x: 0, y: 0 }), false),
  sideOf(straightCubic({ x: 100, y: 0 }, { x: 100, y: 50 }), false),
];

/** Handles off, in mode angle and length. */
const OFF = {
  after: { x: 0, y: 0 },
  before: { x: 0, y: 0 },
  isOn: false,
  mode: "angle-and-length",
} as const;

/**
 * The Product Owner's rectangle (VAL-011): 120 × 80, point 3 in handles mode along the diagonal.
 *
 * @param handle - the length of point 3's handles
 * @param radii - the radius asked at points 1, 2, 3 and 4
 * @returns its corners
 */
export function bentRectangle(handle: number, radii: readonly number[]): readonly CurveCorner[] {
  const along = handle / Math.SQRT2;
  const points = [
    { x: 0, y: 0 },
    { x: 120, y: 0 },
    { x: 120, y: 80 },
    { x: 0, y: 80 },
  ];

  return points.map((point, index) => ({
    handles:
      index === 2
        ? { ...OFF, after: { x: -along, y: along }, before: { x: along, y: -along }, isOn: true }
        : OFF,
    point,
    radius: radii[index] ?? 0,
  }));
}
