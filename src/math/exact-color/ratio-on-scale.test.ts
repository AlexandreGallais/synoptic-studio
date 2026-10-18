import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ratioOnScale } from "./ratio-on-scale";

describe("ratioOnScale (DERIV-color-spaces steps 11 and 12)", () => {
  it.each([
    // 255 · 1 / 10 = 25.5, a tie, up to 26; 255 · 1 / 3 = 85.
    { expected: 26, ratio: { denominator: 10, numerator: 1 }, scale: 255 },
    { expected: 85, ratio: { denominator: 3, numerator: 1 }, scale: 255 },
    // 100 · 102 / 153 = 66.67, to 67; 100 · 1 / 8 = 12.5, a tie, up to 13.
    { expected: 67, ratio: { denominator: 153, numerator: 102 }, scale: 100 },
    { expected: 13, ratio: { denominator: 8, numerator: 1 }, scale: 100 },
    // An alpha byte 0x80 = 128 in percent: 100 · 128 / 255 = 50.2, to 50.
    { expected: 50, ratio: { denominator: 255, numerator: 128 }, scale: 100 },
    // 0 and 1 are the scale's ends.
    { expected: 0, ratio: { denominator: 7, numerator: 0 }, scale: 255 },
    { expected: 255, ratio: { denominator: 7, numerator: 7 }, scale: 255 },
  ])("gives $ratio on $scale as $expected", ({ expected, ratio, scale }) => {
    expect(ratioOnScale(ratio, scale)).toBe(expected);
  });

  test.prop({
    denominator: fc.integer({ max: 100_000, min: 1 }),
    scale: fc.constantFrom(100, 255),
    share: fc.double({ max: 1, min: 0, noNaN: true }),
  })("lies within half a unit of the exact value", ({ denominator, scale, share }) => {
    const numerator = Math.round(share * denominator);
    const shown = ratioOnScale({ denominator, numerator }, scale);

    // |k · N − D · shown| ≤ D / 2, in integers.
    expect(Math.abs(scale * numerator - denominator * shown)).toBeLessThanOrEqual(denominator / 2);
  });
});
