import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { chainEnd } from "./chain-end";

describe("chainEnd (REF-CRANE-DISCRETE-CURVES)", () => {
  it("walks a unit right, then a unit down: (1, 1) from the origin", () => {
    const end = chainEnd({ x: 0, y: 0 }, [0, Math.PI / 2]);

    expect(end.x).toBeCloseTo(1, 15);
    expect(end.y).toBeCloseTo(1, 15);
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
