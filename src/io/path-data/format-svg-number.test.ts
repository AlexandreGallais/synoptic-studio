import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { formatSvgNumber } from "./format-svg-number";

describe("formatSvgNumber", () => {
  it("writes an integer without decimals", () => {
    expect(formatSvgNumber(1)).toBe("1");
  });

  it("[F01.AC4] rounds to 5 decimals", () => {
    // 1 / 3 = 0.333333…, 5 decimals kept.
    expect(formatSvgNumber(1 / 3)).toBe("0.33333");
    // 86.602540378 → 86.60254 (sixth decimal 0, rounded down).
    expect(formatSvgNumber(86.602540378)).toBe("86.60254");
    // 0.123456 → 0.12346: the sixth decimal 6 rounds up (a truncation would give 0.12345).
    expect(formatSvgNumber(0.123456)).toBe("0.12346");
  });

  it("drops trailing zeros", () => {
    // 2.5000000001 rounds to 2.50000, written without its trailing zeros.
    expect(formatSvgNumber(2.5000000001)).toBe("2.5");
  });

  it("never writes a negative zero", () => {
    // -0.000001 rounds to -0.00000, written as 0.
    expect(formatSvgNumber(-0.000001)).toBe("0");
  });

  it("keeps the sign of negative numbers", () => {
    expect(formatSvgNumber(-12.25)).toBe("-12.25");
  });
});

describe("formatSvgNumber (properties)", () => {
  test.prop({ value: fc.double({ max: 1e6, min: -1e6, noNaN: true }) })(
    "[F01.AC4] writes at most 5 decimals, within half a unit of the fifth decimal",
    ({ value }) => {
      const text = formatSvgNumber(value);

      expect(text.split(".", 2)[1]?.length ?? 0).toBeLessThanOrEqual(5);
      expect(Math.abs(Number(text) - value)).toBeLessThanOrEqual(5e-6 + 1e-9);
    },
  );
});
