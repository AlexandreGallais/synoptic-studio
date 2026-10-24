import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { polygonSegments } from "../painted-area";

import { isContourTouchingBox } from "./is-contour-touching-box";

import type { Arc, ContourPiece } from "../contour";

/** A box 100 × 100 at (0, 0), clockwise on screen. */
const SQUARE = polygonSegments([
  { x: 0, y: 0 },
  { x: 100, y: 0 },
  { x: 100, y: 100 },
  { x: 0, y: 100 },
]);

/** The arch of `DERIV-box-overlap` closed by the segment from (100, 0) back to (0, 0). */
const CLOSED_ARCH: readonly ContourPiece[] = [
  {
    end: { x: 100, y: 0 },
    first: { x: 0, y: -40 },
    kind: "cubic",
    second: { x: 100, y: -40 },
    start: { x: 0, y: 0 },
  },
  { end: { x: 0, y: 0 }, kind: "segment", start: { x: 100, y: 0 } },
];

/** A quarter of the circle of radius 50 about (50, 50), clockwise on screen. */
const QUARTER: Pick<Arc, "center" | "kind" | "positive" | "radius"> = {
  center: { x: 50, y: 50 },
  kind: "arc",
  positive: true,
  radius: 50,
};

/** That circle as four quarter arcs, from its left point. */
const CIRCLE: readonly ContourPiece[] = [
  { ...QUARTER, end: { x: 50, y: 0 }, start: { x: 0, y: 50 } },
  { ...QUARTER, end: { x: 100, y: 50 }, start: { x: 50, y: 0 } },
  { ...QUARTER, end: { x: 50, y: 100 }, start: { x: 100, y: 50 } },
  { ...QUARTER, end: { x: 0, y: 50 }, start: { x: 50, y: 100 } },
];

/** A side of a box on integers. */
const SIDE = fc.integer({ max: 150, min: -50 });

describe("isContourTouchingBox (DERIV-box-overlap step 3)", () => {
  it("touches a box inside the painted area, no piece meeting it", () => {
    // [40, 60] × [−20, −10]: the arch is above it, the segment y = 0 below; K = (40, −20) is
    // under the arch, wn = +1.
    expect(isContourTouchingBox(CLOSED_ARCH, { maxX: 60, maxY: -10, minX: 40, minY: -20 })).toBe(
      true,
    );
  });

  it("misses a box in the corner of the contour's box", () => {
    // [0, 10] × [−30, −20]: no piece meets it, K = (0, −30) above the arch, wn = 0.
    expect(isContourTouchingBox(CLOSED_ARCH, { maxX: 10, maxY: -20, minX: 0, minY: -30 })).toBe(
      false,
    );
  });

  it("touches a box sharing only a side with the contour", () => {
    // [100, 200] × [50, 60]: the square's side x = 100 reaches y = 50 at x = 100.
    expect(isContourTouchingBox(SQUARE, { maxX: 200, maxY: 60, minX: 100, minY: 50 })).toBe(true);
    expect(isContourTouchingBox(SQUARE, { maxX: 200, maxY: 60, minX: 101, minY: 50 })).toBe(false);
  });

  it("touches a box holding the whole contour", () => {
    // Every corner of the square is in [−10, 110]².
    expect(isContourTouchingBox(SQUARE, { maxX: 110, maxY: 110, minX: -10, minY: -10 })).toBe(true);
  });

  it("misses a box in the corner of a circle's box, and touches one the circle crosses", () => {
    // (90, 90) is 40√2 ≈ 56.57 from (50, 50), over 50; (80, 80) is 30√2 ≈ 42.43, under it.
    expect(isContourTouchingBox(CIRCLE, { maxX: 120, maxY: 120, minX: 90, minY: 90 })).toBe(false);
    expect(isContourTouchingBox(CIRCLE, { maxX: 120, maxY: 120, minX: 80, minY: 80 })).toBe(true);
  });
});

describe("isContourTouchingBox (properties)", () => {
  test.prop({ bottom: SIDE, left: SIDE, right: SIDE, top: SIDE })(
    "touches an upright box exactly when the intervals overlap on both axes",
    ({ bottom, left, right, top }) => {
      // The cross-check of DERIV-box-overlap: the square is [0, 100]².
      const box = {
        maxX: Math.max(left, right),
        maxY: Math.max(top, bottom),
        minX: Math.min(left, right),
        minY: Math.min(top, bottom),
      };
      const isOverlapping = box.minX <= 100 && box.maxX >= 0 && box.minY <= 100 && box.maxY >= 0;

      expect(isContourTouchingBox(SQUARE, box)).toBe(isOverlapping);
    },
  );
});
