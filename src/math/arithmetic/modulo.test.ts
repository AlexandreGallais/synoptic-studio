import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { modulo } from "./modulo";

describe("modulo (REF-MDN-REMAINDER)", () => {
  it.each([
    { divisor: 4, dividend: -1, expected: 3 },
    { divisor: 4, dividend: 5, expected: 1 },
    { divisor: 360, dividend: -370, expected: 350 },
    { divisor: 360, dividend: 720, expected: 0 },
  ])("gives $expected for $dividend mod $divisor", ({ dividend, divisor, expected }) => {
    expect(modulo(dividend, divisor)).toBe(expected);
  });

  it("keeps +0 for −0 and gives NaN for a divisor 0", () => {
    expect(Object.is(modulo(-0, 360), 0)).toBe(true);
    expect(modulo(1, 0)).toBeNaN();
  });

  test.prop({
    dividend: fc.integer({ max: 1e6, min: -1e6 }),
    divisor: fc.integer({ max: 1000, min: 1 }),
  })("lies in [0, d) and differs from the dividend by a multiple of d", ({ dividend, divisor }) => {
    const rest = modulo(dividend, divisor);

    expect(rest >= 0 && rest < divisor).toBe(true);
    expect(Number.isSafeInteger((dividend - rest) / divisor)).toBe(true);
  });
});
