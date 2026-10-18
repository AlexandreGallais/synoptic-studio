import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { largestHalfChroma } from "./largest-half-chroma";

describe("largestHalfChroma (REF-WIKI-HSL-HSV, DERIV-color-spaces step 4)", () => {
  it.each([
    // min(0, 1), min(1, 0): black and white have no chroma.
    { expected: 0, lightness: 0 },
    { expected: 0, lightness: 1 },
    // min(0.5, 0.5): the most, at half lightness.
    { expected: 0.5, lightness: 0.5 },
    // min(0.4, 0.6) for #336699; min(0.7, 0.3) for #cc9999, past half lightness.
    { expected: 0.4, lightness: 0.4 },
    { expected: 0.3, lightness: 0.7 },
  ])("[F16.AC3] is $expected at the lightness $lightness", ({ expected, lightness }) => {
    expect(largestHalfChroma(lightness)).toBeCloseTo(expected, 12);
  });

  test.prop({ lightness: fc.double({ max: 1, min: 0, noNaN: true }) })(
    "is from 0 to ½, the same at L and 1 − L",
    ({ lightness }) => {
      const half = largestHalfChroma(lightness);

      expect(half).toBeGreaterThanOrEqual(0);
      expect(half).toBeLessThanOrEqual(0.5);
      expect(largestHalfChroma(1 - lightness)).toBeCloseTo(half, 12);
    },
  );
});
