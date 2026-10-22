import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { straightCubic } from "../curve";

import { cubicCrossing } from "./cubic-crossing";
import { segmentCrossing } from "./segment-crossing";

import type { Cubic } from "../contour";

/** An arch from (0, 0) to (100, 0), 30 high: y = −120 t (1 − t), x = 300 t² − 200 t³. */
const ARCH: Cubic = {
  end: { x: 100, y: 0 },
  first: { x: 0, y: -40 },
  kind: "cubic",
  second: { x: 100, y: -40 },
  start: { x: 0, y: 0 },
};

/** A coordinate, an integer. */
const COORDINATE = fc.integer({ max: 100, min: -100 });

/** A point on integers. */
const POINT = fc.record({ x: COORDINATE, y: COORDINATE });

describe("cubicCrossing (DERIV-painted-area step 4)", () => {
  it.each([
    // Crossings of y = −15 at x ≈ 5.80583 (y falling) and ≈ 94.19417 (y growing).
    { expected: 1, x: 50, y: -15 },
    { expected: 0, x: 0, y: -15 },
    { expected: 0, x: 95, y: -15 },
  ])("counts $expected at ($x, $y)", ({ expected, x, y }) => {
    expect(cubicCrossing(ARCH, { x, y })).toBe(expected);
  });

  it("counts 0 above the arch, and at its top, where the ray only touches it", () => {
    // −35 < −30; at −30 the two parts count −1 and +1.
    expect(cubicCrossing(ARCH, { x: 50, y: -35 })).toBe(0);
    expect(cubicCrossing(ARCH, { x: -10, y: -30 })).toBe(0);
  });
});

describe("cubicCrossing (properties)", () => {
  test.prop({ end: POINT, point: POINT, start: POINT })(
    "counts a straight cubic side as its segment",
    ({ end, point, start }) => {
      // Away from the line, where the crossing's rounding could matter.
      const side =
        (end.x - start.x) * (point.y - start.y) - (end.y - start.y) * (point.x - start.x);

      fc.pre(Math.abs(side) > 1);

      expect(cubicCrossing(straightCubic(start, end), point)).toBe(
        segmentCrossing({ end, start }, point),
      );
    },
  );
});
