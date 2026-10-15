import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { roundedQuotient } from "./rounded-quotient";

describe("roundedQuotient (DERIV-color-spaces, step 2)", () => {
  it.each([
    // 255 · 50 / 100 = 127.5, a tie rounded up; 50 × 2.55 in binary64 would round to 127.
    { dividend: 12_750, divisor: 100, expected: 128 },
    // 100 · 236 / 255 = 92.549…: ⌊(47 200 + 255) / 510⌋ = ⌊93.05⌋.
    { dividend: 23_600, divisor: 255, expected: 93 },
    // 100 · 204 / 255 = 80 exactly.
    { dividend: 20_400, divisor: 255, expected: 80 },
    // 7 / 2 = 3.5, a tie rounded up.
    { dividend: 7, divisor: 2, expected: 4 },
    // −7 / 2 = −3.5, a tie rounded up, toward +∞: ⌊(−14 + 2) / 4⌋ = −3.
    { dividend: -7, divisor: 2, expected: -3 },
    // −5 / 2 = −2.5, a tie rounded up, toward +∞.
    { dividend: -5, divisor: 2, expected: -2 },
    // 1 / 3 = 0.33…, 2 / 3 = 0.66….
    { dividend: 1, divisor: 3, expected: 0 },
    { dividend: 2, divisor: 3, expected: 1 },
    // 0 / 100.
    { dividend: 0, divisor: 100, expected: 0 },
  ])("gives $expected for $dividend / $divisor", ({ dividend, divisor, expected }) => {
    expect(roundedQuotient(dividend, divisor)).toBe(expected);
  });

  test.prop({
    dividend: fc.integer({ max: 1e9, min: -1e9 }),
    divisor: fc.integer({ max: 1e6, min: 1 }),
  })("is the integer nearest the quotient, a tie rounded up", ({ dividend, divisor }) => {
    const quotient = roundedQuotient(dividend, divisor);
    // q nearest n / d, a tie up ⇔ −d < 2 (q · d − n) ≤ d, in integers.
    const twiceGap = 2 * (quotient * divisor - dividend);

    expect(Number.isSafeInteger(quotient)).toBe(true);
    expect(twiceGap > -divisor && twiceGap <= divisor).toBe(true);
  });
});
