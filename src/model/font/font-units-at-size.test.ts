import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { fontUnitsAtSize } from "./font-units-at-size";

describe("fontUnitsAtSize (DERIV-text-width step 1)", () => {
  it.each([
    // "Hello World" in Arial: 10 584 × 16 / 2048 = 82.6875.
    { expected: 82.6875, size: 16, units: 10_584, unitsPerEm: 2048 },
    // Arial's line height: 2355 × 16 / 2048 = 2355 / 128 = 18.3984375.
    { expected: 18.3984375, size: 16, units: 2355, unitsPerEm: 2048 },
    // One em at any size is the size.
    { expected: 100_000, size: 100_000, units: 1000, unitsPerEm: 1000 },
    { expected: 0, size: 16, units: 0, unitsPerEm: 2048 },
    // Multiplied first: 3 × 12 / 1000 = 0.036; (3 / 1000) × 12 would be 0.036000000000000004.
    { expected: 0.036, size: 12, units: 3, unitsPerEm: 1000 },
    // A negative length, a kerning, stays negative: −152 × 16 / 2048 = −1.1875.
    { expected: -1.1875, size: 16, units: -152, unitsPerEm: 2048 },
  ])(
    "brings $units font units at size $size, $unitsPerEm per em, to $expected",
    ({ expected, size, units, unitsPerEm }) => {
      expect(fontUnitsAtSize(units, size, unitsPerEm)).toBe(expected);
    },
  );

  test.prop({
    size: fc.integer({ max: 100_000, min: 1 }),
    units: fc.integer({ max: 100_000, min: -100_000 }),
    unitsPerEm: fc.integer({ max: 16_384, min: 16 }),
  })("grows with the size: twice the size, twice the length", ({ size, units, unitsPerEm }) => {
    expect(fontUnitsAtSize(units, 2 * size, unitsPerEm)).toBeCloseTo(
      2 * fontUnitsAtSize(units, size, unitsPerEm),
      6,
    );
  });
});
