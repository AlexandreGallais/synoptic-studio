import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { distance } from "../../math";

import { polygonDistance } from "./polygon-distance";

/** The corners of a box 0 × 80 at (10, 10): two pairs of equal points. */
const FLAT = [
  { x: 10, y: 10 },
  { x: 10, y: 10 },
  { x: 10, y: 90 },
  { x: 10, y: 90 },
];

/** A coordinate, an integer. */
const COORDINATE = fc.integer({ max: 1000, min: -1000 });

/** A point on integers. */
const POINT = fc.record({ x: COORDINATE, y: COORDINATE });

describe("polygonDistance (DERIV-painted-area step 6)", () => {
  it("measures a flat box from its line", () => {
    // (12, 50): 2 from the side (10, 10)–(10, 90); (10, 95): 5 past (10, 90).
    expect(polygonDistance(FLAT, { x: 12, y: 50 })).toBe(2);
    expect(polygonDistance(FLAT, { x: 10, y: 95 })).toBe(5);
  });

  it("takes the nearest side of a box", () => {
    // Box 100 × 100: (50, 3) is 3 from its top, 47 from its left and right, 97 from its bottom.
    const box = [
      { x: 0, y: 0 },
      { x: 100, y: 0 },
      { x: 100, y: 100 },
      { x: 0, y: 100 },
    ];

    expect(polygonDistance(box, { x: 50, y: 3 })).toBe(3);
  });

  it("is infinite for no point", () => {
    expect(polygonDistance([], { x: 0, y: 0 })).toBe(Infinity);
  });
});

describe("polygonDistance (properties)", () => {
  test.prop({ point: POINT, points: fc.array(POINT, { maxLength: 6, minLength: 1 }) })(
    "is at most the distance to any of its points",
    ({ point, points }) => {
      const nearest = Math.min(...points.map((corner) => distance(point, corner)));

      expect(polygonDistance(points, point)).toBeLessThanOrEqual(nearest);
    },
  );
});
