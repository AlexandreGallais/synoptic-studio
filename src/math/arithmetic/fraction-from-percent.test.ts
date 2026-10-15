import { describe, expect, it } from "vitest";

import { fractionFromPercent } from "./fraction-from-percent";

describe("fractionFromPercent (REF-CSS-COLOR-4 §7.1)", () => {
  it.each([
    // 50 / 100, 100 / 100, 0 / 100, 66.7 / 100.
    { expected: 0.5, percent: 50 },
    { expected: 1, percent: 100 },
    { expected: 0, percent: 0 },
    { expected: 0.667, percent: 66.7 },
  ])("gives $expected for $percent %", ({ expected, percent }) => {
    expect(fractionFromPercent(percent)).toBeCloseTo(expected, 12);
  });
});
