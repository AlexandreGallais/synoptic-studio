import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { edgeFactor } from "./edge-factor";

describe("edgeFactor (DERIV-local-radius-clamp step 2)", () => {
  it("is L / S when the demand exceeds the edge", () => {
    // One corner at 1000 on a side of 100: 100 / 1000.
    expect(edgeFactor(100, 1000)).toBe(0.1);
    // Two right corners at 100 on an edge of 25: 25 / 200.
    expect(edgeFactor(25, 200)).toBe(0.125);
  });

  it("is 1 when the demand fits the edge", () => {
    expect(edgeFactor(100, 50)).toBe(1);
    // Exactly the length: two corners at 50 on a side of 100 (circle).
    expect(edgeFactor(100, 100)).toBe(1);
  });

  it("is 1 on a zero-length edge without demand: no division by zero", () => {
    // S ≤ L is tested first: 0 ≤ 0.
    expect(edgeFactor(0, 0)).toBe(1);
  });
});

describe("edgeFactor (properties)", () => {
  test.prop({
    demand: fc.integer({ max: 5000, min: 0 }),
    length: fc.integer({ max: 5000, min: 0 }),
  })("stays in [0, 1] and fits the reduced demand in the edge", ({ demand, length }) => {
    const factor = edgeFactor(length, demand);

    expect(factor).toBeGreaterThanOrEqual(0);
    expect(factor).toBeLessThanOrEqual(1);
    expect(factor * demand).toBeLessThanOrEqual(length * (1 + 1e-12));
  });
});
