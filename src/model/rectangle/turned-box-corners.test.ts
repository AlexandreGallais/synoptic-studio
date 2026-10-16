import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { NO_HANDLES } from "../../geometry";

import { turnedBoxCorners } from "./turned-box-corners";

describe("turnedBoxCorners (DERIV-turned-shape step 4)", () => {
  it("turns a box 60 × 40 at (0, 0) by 90 about (30, 20), exactly, sharp and without handles", () => {
    // R(90°): (dx, dy) ↦ (−dy, dx); (−30, −20) ↦ (20, −30), plus (30, 20) = (50, −10).
    const box = { height: 40, width: 60, x: 0, y: 0 };

    expect(turnedBoxCorners(box, { ...box, angle: 90 })).toEqual([
      { handles: NO_HANDLES, point: { x: 50, y: -10 }, radius: 0 },
      { handles: NO_HANDLES, point: { x: 50, y: 50 }, radius: 0 },
      { handles: NO_HANDLES, point: { x: 10, y: 50 }, radius: 0 },
      { handles: NO_HANDLES, point: { x: 10, y: -10 }, radius: 0 },
    ]);
  });

  it("turns an inset box about the middle of the box given to turn, (30, 20)", () => {
    // The box 58 × 38 at (1, 1): (−29, −19) ↦ (19, −29), plus (30, 20) = (49, −9).
    const turn = { angle: 90, height: 40, width: 60, x: 0, y: 0 };

    expect(
      turnedBoxCorners({ height: 38, width: 58, x: 1, y: 1 }, turn).map((corner) => corner.point),
    ).toEqual([
      { x: 49, y: -9 },
      { x: 49, y: 49 },
      { x: 11, y: 49 },
      { x: 11, y: -9 },
    ]);
  });

  test.prop({
    height: fc.integer({ max: 1000, min: 0 }),
    width: fc.integer({ max: 1000, min: 0 }),
  })("gives the box's corners at 0°, bit for bit", ({ height, width }) => {
    const box = { height, width, x: -3, y: 7 };

    expect(turnedBoxCorners(box, { ...box, angle: 0 }).map((corner) => corner.point)).toEqual([
      { x: -3, y: 7 },
      { x: -3 + width, y: 7 },
      { x: -3 + width, y: 7 + height },
      { x: -3, y: 7 + height },
    ]);
  });
});
