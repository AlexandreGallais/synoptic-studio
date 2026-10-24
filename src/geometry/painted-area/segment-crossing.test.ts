import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { segmentCrossing } from "./segment-crossing";

/** A coordinate, an integer, so that the side test is exact. */
const COORDINATE = fc.integer({ max: 1000, min: -1000 });

/** A point on integers. */
const POINT = fc.record({ x: COORDINATE, y: COORDINATE });

/** The right side of a box 100 × 100, going down the screen: y grows. */
const RIGHT_SIDE = { end: { x: 100, y: 100 }, start: { x: 100, y: 0 } };

describe("segmentCrossing (DERIV-painted-area step 2)", () => {
  it("counts +1 for a side crossing the ray with y growing, right of the point", () => {
    // d = +1; (0, 100) × (−50, 50) = 0 · 50 − 100 · (−50) = 5000 > 0.
    expect(segmentCrossing(RIGHT_SIDE, { x: 50, y: 50 })).toBe(1);
  });

  it("counts −1 for the same side read backwards, y falling", () => {
    // d = −1; (0, −100) × (−50, −50) = 0 · (−50) − (−100) · (−50) = −5000 < 0.
    const reversed = { end: RIGHT_SIDE.start, start: RIGHT_SIDE.end };

    expect(segmentCrossing(reversed, { x: 50, y: 50 })).toBe(-1);
  });

  it("counts 0 for a side left of the point", () => {
    // (0, 100) × (50, 50) = −5000: the crossing at x = 100 is left of 150.
    expect(segmentCrossing(RIGHT_SIDE, { x: 150, y: 50 })).toBe(0);
  });

  it("counts the side's start and not its end", () => {
    // 0 ≤ 0 < 100 holds; 0 ≤ 100 < 100 does not.
    expect(segmentCrossing(RIGHT_SIDE, { x: 50, y: 0 })).toBe(1);
    expect(segmentCrossing(RIGHT_SIDE, { x: 50, y: 100 })).toBe(0);
  });

  it("finds where a slanted side crosses: at x = 50 on the level 50", () => {
    // From (0, 0) to (100, 100): (100, 100) × (40, 50) = 1000 > 0; × (60, 50) = −1000.
    const slanted = { end: { x: 100, y: 100 }, start: { x: 0, y: 0 } };

    expect(segmentCrossing(slanted, { x: 40, y: 50 })).toBe(1);
    expect(segmentCrossing(slanted, { x: 60, y: 50 })).toBe(0);
  });

  it("counts 0 for a flat side, even on its own level", () => {
    // Horizontal edges are excluded.
    expect(
      segmentCrossing({ end: { x: 100, y: 0 }, start: { x: 0, y: 0 } }, { x: -10, y: 0 }),
    ).toBe(0);
  });

  it("counts 0 for a point on the side itself", () => {
    // (0, 100) × (0, 50) = 0: the crossing is not strictly right of the point.
    expect(segmentCrossing(RIGHT_SIDE, { x: 100, y: 50 })).toBe(0);
  });
});

describe("segmentCrossing (properties)", () => {
  test.prop({ end: POINT, point: POINT, start: POINT })(
    "changes sign read backwards",
    ({ end, point, start }) => {
      const backwards = segmentCrossing({ end: start, start: end }, point);

      expect(backwards + segmentCrossing({ end, start }, point)).toBe(0);
    },
  );

  test.prop({ end: POINT, point: POINT, start: POINT })(
    "counts the direction of y, or 0",
    ({ end, point, start }) => {
      const crossing = segmentCrossing({ end, start }, point);

      expect([0, Math.sign(end.y - start.y)]).toContain(crossing);
    },
  );
});
