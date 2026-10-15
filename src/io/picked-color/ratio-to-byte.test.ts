import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ratioToByte } from "./ratio-to-byte";

describe("ratioToByte (DERIV-color-spaces step 11)", () => {
  it.each([
    // 255 · 6 · 10⁷ / 6 · 10⁸ = 25.5, a tie, up to 26.
    { expected: 26, numerator: 60_000_000, denominator: 600_000_000 },
    // 255 · 5 · 10⁸ / 6 · 10⁸ = 212.5, up to 213.
    { expected: 213, numerator: 500_000_000, denominator: 600_000_000 },
    // 255 · 1 / 3 = 85; the ends 0 and 255.
    { expected: 85, numerator: 100_000_000, denominator: 300_000_000 },
    { expected: 0, numerator: 0, denominator: 300_000_000 },
    { expected: 255, numerator: 300_000_000, denominator: 300_000_000 },
  ])(
    "[F16.AC3] gives $expected for $numerator / $denominator",
    ({ denominator, expected, numerator }) => {
      expect(ratioToByte({ denominator, numerator })).toBe(expected);
    },
  );

  test.prop({ numerator: fc.integer({ max: 600_000_000, min: 0 }) })(
    "rounds as the exact integers do, a tie up",
    ({ numerator }) => {
      const exact = Number((510n * BigInt(numerator) + 600_000_000n) / 1_200_000_000n);

      expect(ratioToByte({ denominator: 600_000_000, numerator })).toBe(exact);
    },
  );
});
