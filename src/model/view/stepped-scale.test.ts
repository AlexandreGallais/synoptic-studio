import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { steppedScale } from "./stepped-scale";

describe("steppedScale (DERIV-view-zoom step 5)", () => {
  it.each([
    // 1 · √2 = 1.41421; 2 · √2 = 2.82843; 1 / √2 = 0.70711.
    { direction: 1, expected: Math.SQRT2, scale: 1 },
    { direction: 1, expected: 2.82843, scale: 2 },
    { direction: -1, expected: Math.SQRT1_2, scale: 1 },
    // min(256 · √2, 256); max(0.01 / √2, 0.01).
    { direction: 1, expected: 256, scale: 256 },
    { direction: -1, expected: 0.01, scale: 0.01 },
  ] as const)(
    "[F42.AC3] steps $scale by $direction, within the bounds",
    ({ direction, expected, scale }) => {
      expect(steppedScale(scale, direction)).toBeCloseTo(expected, 5);
    },
  );

  test.prop({ scale: fc.double({ max: 256 / Math.SQRT2, min: 0.01, noNaN: true }) })(
    "gives the scale back by a step up then down",
    ({ scale }) => {
      expect(steppedScale(steppedScale(scale, 1), -1)).toBeCloseTo(scale, 9);
    },
  );
});
