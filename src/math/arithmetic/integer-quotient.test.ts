import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { integerQuotient } from "./integer-quotient";

describe("integerQuotient (REF-WIKI-FLOOR-CEILING)", () => {
  it.each([
    // 13 = 4 · 3 + 1; 12 = 4 · 3 + 0; 3 = 4 · 0 + 3; 0 = 8 · 0 + 0.
    { dividend: 13, divisor: 4, expected: 3 },
    { dividend: 12, divisor: 4, expected: 3 },
    { dividend: 3, divisor: 4, expected: 0 },
    { dividend: 0, divisor: 8, expected: 0 },
    // A cmap of 4 + 8 · 2 + 7 bytes holds 2 whole encoding records of 8 bytes: ⌊23 / 8⌋.
    { dividend: 23, divisor: 8, expected: 2 },
    // 0xab = 171 = 4 · 42 + 3: its six high bits are 42.
    { dividend: 0xab, divisor: 4, expected: 42 },
  ])("gives $expected whole $divisor in $dividend", ({ dividend, divisor, expected }) => {
    expect(integerQuotient(dividend, divisor)).toBe(expected);
  });

  test.prop({
    dividend: fc.integer({ max: 2 ** 40, min: 0 }),
    divisor: fc.integer({ max: 2 ** 20, min: 1 }),
  })("leaves a remainder from 0 up to the divisor", ({ dividend, divisor }) => {
    const quotient = integerQuotient(dividend, divisor);
    const remainder = dividend - divisor * quotient;

    expect(Number.isSafeInteger(quotient)).toBe(true);
    expect(remainder).toBeGreaterThanOrEqual(0);
    expect(remainder).toBeLessThan(divisor);
  });
});
