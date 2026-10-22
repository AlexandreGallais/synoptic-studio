import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { distance } from "../../math";

import { segmentDistance } from "./segment-distance";

/** The segment from (0, 0) to (100, 0). */
const SEGMENT = { end: { x: 100, y: 0 }, start: { x: 0, y: 0 } };

/** A coordinate, an integer. */
const COORDINATE = fc.integer({ max: 1000, min: -1000 });

/** A point on integers. */
const POINT = fc.record({ x: COORDINATE, y: COORDINATE });

describe("segmentDistance (DERIV-painted-area step 6)", () => {
  it("measures to the foot when it falls on the segment", () => {
    // w · v = 5000, v · v = 10 000: b = ½, the foot (50, 0), 3 away.
    expect(segmentDistance(SEGMENT, { x: 50, y: 3 })).toBe(3);
  });

  it("measures to the start before it and to the end after it", () => {
    // (−3, 4): w · v = −300 ≤ 0, |(−3, 4)| = 5; (103, 4): v · v = 10 000 ≤ 10 300, |(3, 4)| = 5.
    expect(segmentDistance(SEGMENT, { x: -3, y: 4 })).toBe(5);
    expect(segmentDistance(SEGMENT, { x: 103, y: 4 })).toBe(5);
  });

  it("measures to its point for a segment of length 0", () => {
    // w · v = 0 ≤ 0: |(8, 9) − (5, 5)| = |(3, 4)| = 5.
    expect(segmentDistance({ end: { x: 5, y: 5 }, start: { x: 5, y: 5 } }, { x: 8, y: 9 })).toBe(5);
  });

  it("measures to the foot when it falls just past the start", () => {
    // (0, 0) to (2, 1), P = (1, −1): w · v = 2 − 1 = 1 > 0, v · v = 5; the foot (⅖, ⅕), the
    // distance to the line x − 2y = 0: |1 + 2| / √5 = 3 / √5 ≈ 1.341641, less than |P − S| = √2.
    expect(
      segmentDistance({ end: { x: 2, y: 1 }, start: { x: 0, y: 0 } }, { x: 1, y: -1 }),
    ).toBeCloseTo(3 / Math.sqrt(5), 12);
  });

  it("is 0 on the segment", () => {
    expect(segmentDistance(SEGMENT, { x: 20, y: 0 })).toBe(0);
  });
});

describe("segmentDistance (properties)", () => {
  test.prop({ end: POINT, point: POINT, start: POINT })(
    "is at most the distance to either end, and the same read backwards",
    ({ end, point, start }) => {
      const measured = segmentDistance({ end, start }, point);

      expect(measured).toBeGreaterThanOrEqual(0);
      expect(measured).toBeLessThanOrEqual(Math.min(distance(point, start), distance(point, end)));
      expect(segmentDistance({ end: start, start: end }, point)).toBeCloseTo(measured, 6);
    },
  );
});
