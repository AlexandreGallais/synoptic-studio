import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { boundingBox } from "./bounding-box";

describe("boundingBox", () => {
  it("takes the extreme coordinates of the triangle of US-005", () => {
    // (50, 6.7), (100, 93.3), (0, 93.3): x from 0 to 100, y from 6.7 to 93.3.
    expect(
      boundingBox([
        { x: 50, y: 6.7 },
        { x: 100, y: 93.3 },
        { x: 0, y: 93.3 },
      ]),
    ).toEqual({ maxX: 100, maxY: 93.3, minX: 0, minY: 6.7 });
  });

  it("reduces to the point itself for one point", () => {
    expect(boundingBox([{ x: 3, y: -4 }])).toEqual({ maxX: 3, maxY: -4, minX: 3, minY: -4 });
  });

  it("has no box for no point: minima +∞, maxima −∞", () => {
    expect(boundingBox([])).toEqual({
      maxX: -Infinity,
      maxY: -Infinity,
      minX: Infinity,
      minY: Infinity,
    });
  });
});

/** Arbitrary point with integer coordinates. */
const POINT = fc.record({ x: fc.integer(), y: fc.integer() });

describe("boundingBox (properties)", () => {
  test.prop({ points: fc.array(POINT, { minLength: 1 }) })(
    "contains every point and touches each side",
    ({ points }) => {
      const box = boundingBox(points);
      const xs = points.map((point) => point.x);
      const ys = points.map((point) => point.y);

      expect(xs.every((x) => x >= box.minX && x <= box.maxX)).toBe(true);
      expect(ys.every((y) => y >= box.minY && y <= box.maxY)).toBe(true);
      expect([box.minX, box.maxX].every((x) => xs.includes(x))).toBe(true);
      expect([box.minY, box.maxY].every((y) => ys.includes(y))).toBe(true);
    },
  );
});
