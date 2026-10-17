import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { modulo } from "./modulo";
import { nonNegativeRemainder } from "./non-negative-remainder";

describe("nonNegativeRemainder (REF-ECMA-262-NUMBER-REMAINDER)", () => {
  it.each([
    // 30.1 − 360 · 0 = 30.1 exactly; ((30.1 % 360) + 360) % 360 would be 30.100000000000023.
    { dividend: 30.1, divisor: 360, expected: 30.1 },
    // 360 − 360 · 1: a hue shown 360.0 is 0.0.
    { dividend: 360, divisor: 360, expected: 0 },
    // 13 − 5 · 2; 5.5 − 2 · 2 (MDN's examples).
    { dividend: 13, divisor: 5, expected: 3 },
    { dividend: 5.5, divisor: 2, expected: 1.5 },
    { dividend: 0, divisor: 360, expected: 0 },
  ])("gives $expected for $dividend on a cycle of $divisor", ({ dividend, divisor, expected }) => {
    expect(nonNegativeRemainder(dividend, divisor)).toBe(expected);
  });

  test.prop({
    dividend: fc.integer({ max: 1e6, min: 0 }),
    divisor: fc.integer({ max: 1000, min: 1 }),
  })("gives what modulo gives for integers", ({ dividend, divisor }) => {
    expect(nonNegativeRemainder(dividend, divisor)).toBe(modulo(dividend, divisor));
  });
});
