import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { cubicBernstein } from "./cubic-bernstein";

describe("cubicBernstein (REF-WIKI-BEZIER)", () => {
  it("is (1/8, 3/8, 3/8, 1/8) at ½, and the ends at 0 and 1", () => {
    expect(cubicBernstein(1 / 2)).toEqual([0.125, 0.375, 0.375, 0.125]);
    expect(cubicBernstein(0)).toEqual([1, 0, 0, 0]);
    // At ¼: (27, 27, 9, 1)/64, exact; the two middle weights told apart.
    expect(cubicBernstein(1 / 4)).toEqual([0.421875, 0.421875, 0.140625, 0.015625]);
    expect(cubicBernstein(1)).toEqual([0, 0, 0, 1]);
  });

  test.prop({ t: fc.double({ max: 1, min: 0, noNaN: true }) })(
    "gives weights ≥ 0 adding up to 1",
    ({ t }) => {
      const weights = cubicBernstein(t);

      expect(weights.every((weight) => weight >= 0)).toBe(true);
      expect(weights.reduce((sum, weight) => sum + weight, 0)).toBeCloseTo(1, 14);
    },
  );
});
