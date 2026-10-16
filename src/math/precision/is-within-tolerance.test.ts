import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { isWithinTolerance } from "./is-within-tolerance";

describe("isWithinTolerance (REF-WIKI-ENGINEERING-TOLERANCE)", () => {
  it.each([
    // The source's resistor: 100 Ω ± 1 accepts 99 and 101, not 101.5.
    { expected: true, measured: 99, nominal: 100, tolerance: 1 },
    { expected: true, measured: 101, nominal: 100, tolerance: 1 },
    { expected: false, measured: 101.5, nominal: 100, tolerance: 1 },
    // A width of 1 004 for 1 000 within 9; 1 010 and 990 are not.
    { expected: true, measured: 1004, nominal: 1000, tolerance: 9 },
    { expected: false, measured: 1010, nominal: 1000, tolerance: 9 },
    { expected: false, measured: 990, nominal: 1000, tolerance: 9 },
    // NaN, a width a browser without a canvas cannot give, is never within.
    { expected: false, measured: NaN, nominal: 1000, tolerance: 9 },
  ])("finds $measured within $tolerance of $nominal: $expected", (row) => {
    expect(isWithinTolerance(row.measured, row.nominal, row.tolerance)).toBe(row.expected);
  });

  test.prop({
    measured: fc.integer({ max: 10_000, min: -10_000 }),
    nominal: fc.integer({ max: 10_000, min: -10_000 }),
    tolerance: fc.integer({ max: 1000, min: 0 }),
  })("accepts the same either side of the nominal value", ({ measured, nominal, tolerance }) => {
    const mirrored = 2 * nominal - measured;

    expect(isWithinTolerance(measured, nominal, tolerance)).toBe(
      isWithinTolerance(mirrored, nominal, tolerance),
    );
  });
});
