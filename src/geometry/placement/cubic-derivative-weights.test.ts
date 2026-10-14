import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { cubicDerivativeWeights } from "./cubic-derivative-weights";

describe("cubicDerivativeWeights (REF-WIKI-BEZIER)", () => {
  it("is (0.75, 1.5, 0.75) at ½, and (3, 0, 0) at 0", () => {
    expect(cubicDerivativeWeights(1 / 2)).toEqual([0.75, 1.5, 0.75]);
    expect(cubicDerivativeWeights(0)).toEqual([3, 0, 0]);
  });

  test.prop({ t: fc.double({ max: 1, min: 0, noNaN: true }) })("adds up to 3", ({ t }) => {
    const weights = cubicDerivativeWeights(t);

    expect(weights.reduce((sum, weight) => sum + weight, 0)).toBeCloseTo(3, 13);
  });
});
