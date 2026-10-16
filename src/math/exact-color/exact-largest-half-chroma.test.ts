import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { largestHalfChroma } from "../color";

import { exactLargestHalfChroma } from "./exact-largest-half-chroma";

describe("exactLargestHalfChroma (REF-WIKI-HSL-HSV)", () => {
  it.each([
    // #336699: L = 204 / 510 ≤ ½, so min(204, 306) = 204.
    {
      expected: { denominator: 510, numerator: 204 },
      lightness: { denominator: 510, numerator: 204 },
    },
    // L = 400 / 510 > ½: min(400, 110) = 110.
    {
      expected: { denominator: 510, numerator: 110 },
      lightness: { denominator: 510, numerator: 400 },
    },
    // Black and white allow none; half lightness allows ½.
    { expected: { denominator: 100, numerator: 0 }, lightness: { denominator: 100, numerator: 0 } },
    {
      expected: { denominator: 100, numerator: 0 },
      lightness: { denominator: 100, numerator: 100 },
    },
    {
      expected: { denominator: 100, numerator: 50 },
      lightness: { denominator: 100, numerator: 50 },
    },
  ])("allows $expected at $lightness", ({ expected, lightness }) => {
    expect(exactLargestHalfChroma(lightness)).toEqual(expected);
  });

  test.prop({
    denominator: fc.integer({ max: 1000, min: 1 }),
    share: fc.double({ max: 1, min: 0, noNaN: true }),
  })(
    "is the largest half chroma of the lightness's value, over its denominator",
    ({ denominator, share }) => {
      const numerator = Math.round(share * denominator);
      const largest = exactLargestHalfChroma({ denominator, numerator });

      expect(largest.denominator).toBe(denominator);

      expect(largest.numerator / denominator).toBeCloseTo(
        largestHalfChroma(numerator / denominator),
        12,
      );
    },
  );
});
