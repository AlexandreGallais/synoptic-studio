import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { midpoint } from "./midpoint";

describe("midpoint", () => {
  it("[F08.AC3] gives the middle of a segment, half of each sum (REF-MATHWORLD-MIDPOINT)", () => {
    // ((0 + 100)/2, (0 + 0)/2) = (50, 0); ((1 + 2)/2, (3 + 8)/2) = (1.5, 5.5).
    expect(midpoint({ x: 0, y: 0 }, { x: 100, y: 0 })).toEqual({ x: 50, y: 0 });
    expect(midpoint({ x: 1, y: 3 }, { x: 2, y: 8 })).toEqual({ x: 1.5, y: 5.5 });
  });

  it("gives the point itself for a segment of length 0", () => {
    expect(midpoint({ x: -7, y: 4 }, { x: -7, y: 4 })).toEqual({ x: -7, y: 4 });
  });

  const coordinate = fc.integer({ max: 100_000, min: -100_000 });
  const point = fc.record({ x: coordinate, y: coordinate });

  test.prop({ a: point, b: point })(
    "is as far from both ends, and the same either way",
    ({ a, b }) => {
      const middle = midpoint(a, b);

      expect(Math.hypot(middle.x - a.x, middle.y - a.y)).toBeCloseTo(
        Math.hypot(b.x - middle.x, b.y - middle.y),
        9,
      );

      expect(midpoint(b, a)).toEqual(middle);
    },
  );
});
