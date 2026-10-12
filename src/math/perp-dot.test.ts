import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { perpDot } from "./perp-dot";

describe("perpDot", () => {
  it("is aₓ b_y − a_y bₓ", () => {
    // 3 × (−1) − 2 × 4 = −11.
    expect(perpDot({ x: 3, y: 2 }, { x: 4, y: -1 })).toBe(-11);
  });

  it("is positive when b is a quarter turn to the left of a (math frame)", () => {
    // (100, 0) then (0, 50): 100 × 50 − 0 × 0 = 5000 = |a| |b| sin(π/2).
    expect(perpDot({ x: 100, y: 0 }, { x: 0, y: 50 })).toBe(5000);
  });

  it("is zero for parallel vectors", () => {
    expect(perpDot({ x: 5, y: 0 }, { x: 10, y: 0 })).toBe(0);
  });
});

describe("perpDot (properties)", () => {
  test.prop({
    a: fc.record({
      x: fc.integer({ max: 1e4, min: -1e4 }),
      y: fc.integer({ max: 1e4, min: -1e4 }),
    }),
    b: fc.record({
      x: fc.integer({ max: 1e4, min: -1e4 }),
      y: fc.integer({ max: 1e4, min: -1e4 }),
    }),
  })("is antisymmetric", ({ a, b }) => {
    expect(perpDot(a, b) + perpDot(b, a)).toBeCloseTo(0, 9);
  });

  test.prop({
    v: fc.record({
      x: fc.integer({ max: 1e4, min: -1e4 }),
      y: fc.integer({ max: 1e4, min: -1e4 }),
    }),
  })("is 0 for a vector with itself: no turn", ({ v }) => {
    expect(perpDot(v, v)).toBeCloseTo(0, 9);
  });
});
