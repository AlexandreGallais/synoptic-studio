import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { norm } from "./norm";
import { scale } from "./scale";

/** Vector with integer components. */
const VECTOR = fc.record({
  x: fc.integer({ max: 1e4, min: -1e4 }),
  y: fc.integer({ max: 1e4, min: -1e4 }),
});

describe("scale (REF-OPENSTAX-CALC3-VECTORS)", () => {
  it("multiplies each component", () => {
    // 2 · (3, −4) = (6, −8).
    expect(scale({ x: 3, y: -4 }, 2)).toEqual({ x: 6, y: -8 });
  });

  it("reverses the direction for a negative factor", () => {
    // −10 · (0.6, 0.8) = (−6, −8): a setback measured backwards along an edge.
    const scaled = scale({ x: 0.6, y: 0.8 }, -10);

    expect(scaled.x).toBeCloseTo(-6, 12);
    expect(scaled.y).toBeCloseTo(-8, 12);
  });

  test.prop({ factor: fc.integer({ max: 100, min: -100 }), v: VECTOR })(
    "multiplies the length by the absolute value of the factor",
    ({ factor, v }) => {
      expect(norm(scale(v, factor))).toBeCloseTo(Math.abs(factor) * norm(v), 6);
    },
  );
});
