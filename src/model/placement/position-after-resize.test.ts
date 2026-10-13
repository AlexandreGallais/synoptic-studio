import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { freeShapeCorners } from "../free-shape";
import { rectangleAsFreeShape } from "../rectangle";

import { positionAfterResize } from "./position-after-resize";
import { turnedCorners } from "./turned-corners";

import type { Point } from "../../math";
import type { FreeShape } from "../free-shape";

/** Where a missing point would be: nowhere. */
const NOWHERE = { x: NaN, y: NaN };

/**
 * Point 1 of a free shape as drawn.
 *
 * @param shape - free shape
 * @returns its first point, turned
 */
function firstDrawn(shape: FreeShape): Point {
  return turnedCorners(freeShapeCorners(shape), shape)[0]?.point ?? NOWHERE;
}

describe("positionAfterResize (DERIV-turned-shape step 5)", () => {
  it("[F04.AC4] keeps the turned top-left of a 100 × 50 rectangle at 90° typed 200 wide", () => {
    // DERIV-turned-shape checks: b' = (−50, 50), point 1 still at (75, −25).
    expect(
      positionAfterResize(
        { angle: 90, height: 50, width: 100, x: 0, y: 0 },
        { height: 50, width: 200 },
      ),
    ).toEqual({ x: -50, y: 50 });
  });

  it("[F04.AC4] leaves the position of a straight shape as it is, its top-left fixed (F09)", () => {
    expect(
      positionAfterResize(
        { angle: 0, height: 50, width: 100, x: 3, y: 4 },
        { height: 9, width: 7 },
      ),
    ).toEqual({ x: 3, y: 4 });
  });

  test.prop({
    angle: fc.integer({ max: 359, min: 1 }),
    height: fc.integer({ max: 500, min: 1 }),
    newHeight: fc.integer({ max: 500, min: 0 }),
    newWidth: fc.integer({ max: 500, min: 0 }),
    width: fc.integer({ max: 500, min: 1 }),
  })("[F04.AC4] keeps point 1 of the rectangle where it is drawn", (sizes) => {
    const before = rectangleAsFreeShape({
      angle: sizes.angle,
      height: sizes.height,
      radii: [0, 0, 0, 0],
      width: sizes.width,
      x: 0,
      y: 0,
    });
    const size = { height: sizes.newHeight, width: sizes.newWidth };
    const after = { ...before, ...size, ...positionAfterResize(before, size) };

    expect(firstDrawn(after).x).toBeCloseTo(firstDrawn(before).x, 9);
    expect(firstDrawn(after).y).toBeCloseTo(firstDrawn(before).y, 9);
  });
});
