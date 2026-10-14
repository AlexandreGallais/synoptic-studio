import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { chainEnd } from "./chain-end";

describe("chainEnd (REF-CRANE-DISCRETE-CURVES)", () => {
  it("walks a unit right, then a unit down: (1, 1) from the origin", () => {
    const end = chainEnd({ x: 0, y: 0 }, [0, Math.PI / 2]);

    expect(end.x).toBeCloseTo(1, 15);
    expect(end.y).toBeCloseTo(1, 15);
  });

  it("tells cosines from sines, from (5, 0): two units right, one down", () => {
    // x = 5 + 1 + 1 + cos(π/2) rounds to 7; y = 0 + 0 + 0 + 1. Swapped, it would be (6, 2).
    expect(chainEnd({ x: 5, y: 0 }, [0, 0, Math.PI / 2])).toEqual({ x: 7, y: 1 });
  });

  test.prop({ count: fc.integer({ max: 12, min: 3 }) })(
    "closes the sides of a regular polygon",
    ({ count }) => {
      const sides = Array.from({ length: count }, (empty, index) => (2 * Math.PI * index) / count);
      const end = chainEnd({ x: 0, y: 0 }, sides);

      expect(end.x).toBeCloseTo(0, 12);
      expect(end.y).toBeCloseTo(0, 12);
    },
  );
});
