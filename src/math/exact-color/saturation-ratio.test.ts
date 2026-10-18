import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { saturationRatio } from "./saturation-ratio";

describe("saturationRatio (REF-WIKI-HSL-HSV, DERIV-color-spaces step 12)", () => {
  it.each([
    // #336699 in HSB: C = 102 out of M = 153.
    { chroma: 102, expected: { denominator: 153, numerator: 102 }, most: 153 },
    // Black: M = 0, the saturation 0, not 0 / 0.
    { chroma: 0, expected: { denominator: 1, numerator: 0 }, most: 0 },
    // A grey: C = 0 out of 128, a saturation of 0.
    { chroma: 0, expected: { denominator: 128, numerator: 0 }, most: 128 },
  ])("[F16.AC2] gives $chroma out of $most as $expected", ({ chroma, expected, most }) => {
    expect(saturationRatio(chroma, most)).toEqual(expected);
  });

  test.prop({ chroma: fc.integer({ max: 255, min: 0 }), extra: fc.integer({ max: 255, min: 0 }) })(
    "is the chroma out of the most, from 0 to 1, with a denominator above 0",
    ({ chroma, extra }) => {
      const most = chroma + extra;
      const { denominator, numerator } = saturationRatio(chroma, most);

      expect(denominator).toBeGreaterThan(0);
      expect(numerator * Math.max(most, 1)).toBe(chroma * denominator);
      expect(numerator / denominator).toBeLessThanOrEqual(1);
    },
  );
});
