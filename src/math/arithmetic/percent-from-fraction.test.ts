import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { fractionFromPercent } from "./fraction-from-percent";
import { percentFromFraction } from "./percent-from-fraction";

describe("percentFromFraction (REF-CSS-COLOR-4 §7.2)", () => {
  it.each([
    // 100 · 0.5, 100 · 1, 100 · 0.
    { expected: 50, fraction: 0.5 },
    { expected: 100, fraction: 1 },
    { expected: 0, fraction: 0 },
    // 100 · 0.4 = 40.00000000000001 in binary64: close to 40.
    { expected: 40, fraction: 0.4 },
  ])("gives $expected % for $fraction", ({ expected, fraction }) => {
    expect(percentFromFraction(fraction)).toBeCloseTo(expected, 12);
  });

  test.prop({ fraction: fc.double({ max: 1, min: 0, noNaN: true }) })(
    "is undone by fractionFromPercent",
    ({ fraction }) => {
      expect(fractionFromPercent(percentFromFraction(fraction))).toBeCloseTo(fraction, 14);
    },
  );
});
