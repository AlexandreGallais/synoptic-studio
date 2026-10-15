import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { planePointOf } from "./plane-point-of";
import { planeValuesAt } from "./plane-values-at";

/** A plane 200 wide and 100 tall. */
const SIZE = { height: 100, width: 200 };

describe("planeValuesAt (DERIV-color-spaces step 10)", () => {
  it.each([
    // The middle of the right edge: S = 100 · 200 / 200 = 100, V = 100 · (100 − 50) / 100 = 50.
    { level: 50, point: { x: 200, y: 50 }, saturation: 100 },
    // Top-left: S = 0, V = 100 · 100 / 100 = 100; bottom-left: V = 0.
    { level: 100, point: { x: 0, y: 0 }, saturation: 0 },
    { level: 0, point: { x: 0, y: 100 }, saturation: 0 },
    // S = 100 · 50 / 200 = 25, V = 100 · (100 − 75) / 100 = 25.
    { level: 25, point: { x: 50, y: 75 }, saturation: 25 },
    // A drag outside counts at the nearest edge.
    { level: 100, point: { x: 230, y: -10 }, saturation: 100 },
    { level: 0, point: { x: -5, y: 140 }, saturation: 0 },
  ])(
    "[F16.AC2] reads $saturation %, $level % at ($point.x, $point.y)",
    ({ level, point, saturation }) => {
      expect(planeValuesAt(point, SIZE)).toEqual({ level, saturation });
    },
  );

  it("reads 0 % and 0 % on a plane not laid out yet, 0 × 0", () => {
    expect(planeValuesAt({ x: 0, y: 0 }, { height: 0, width: 0 })).toEqual({
      level: 0,
      saturation: 0,
    });
  });

  test.prop({ x: fc.integer({ max: 200, min: 0 }), y: fc.integer({ max: 100, min: 0 }) })(
    "gives back the point it was read at",
    ({ x, y }) => {
      const point = planePointOf(planeValuesAt({ x, y }, SIZE), SIZE);

      expect(point.x).toBeCloseTo(x, 9);
      expect(point.y).toBeCloseTo(y, 9);
    },
  );
});

describe("planePointOf (DERIV-color-spaces step 10)", () => {
  it("[F16.AC2] places 100 % and 50 % at the middle of the right edge", () => {
    // x = 200 · 100 / 100 = 200, y = 100 − 100 · 50 / 100 = 50.
    expect(planePointOf({ level: 50, saturation: 100 }, SIZE)).toEqual({ x: 200, y: 50 });
    // x = 200 · 25 / 100 = 50, y = 100 − 100 · 100 / 100 = 0.
    expect(planePointOf({ level: 100, saturation: 25 }, SIZE)).toEqual({ x: 50, y: 0 });
  });
});
