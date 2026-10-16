import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { isBitSet } from "./is-bit-set";

describe("isBitSet (DERIV-bit-of-integer)", () => {
  it.each([
    // ⌊65 535 / 2¹⁵⌋ = 1; ⌊65 535 / 2¹⁶⌋ = 0; ⌊32 768 / 2¹⁵⌋ = 1; ⌊32 768 / 2¹⁴⌋ = 2, even.
    { bit: 15, isSet: true, value: 0xff_ff },
    { bit: 16, isSet: false, value: 0xff_ff },
    { bit: 15, isSet: true, value: 0x80_00 },
    { bit: 14, isSet: false, value: 0x80_00 },
  ])("reads bit $bit of $value as $isSet", ({ bit, isSet, value }) => {
    expect(isBitSet(value, bit)).toBe(isSet);
  });

  it.each([
    // 149 = 1001 0101: bits 0, 2, 4 and 7 are 1 (the source's example).
    { bit: 0, isSet: true },
    { bit: 1, isSet: false },
    { bit: 2, isSet: true },
    { bit: 3, isSet: false },
    { bit: 4, isSet: true },
    { bit: 6, isSet: false },
    { bit: 7, isSet: true },
    { bit: 8, isSet: false },
  ])("reads bit $bit of 149 as $isSet", ({ bit, isSet }) => {
    expect(isBitSet(149, bit)).toBe(isSet);
  });

  it("reads a bit past the 32 of the bitwise operators", () => {
    // 2³² + 1: bits 0 and 32 are 1.
    expect(isBitSet(2 ** 32 + 1, 32)).toBe(true);
    expect(isBitSet(0, 0)).toBe(false);
  });

  test.prop({
    bit: fc.integer({ max: 30, min: 0 }),
    value: fc.integer({ max: 2 ** 31 - 1, min: 0 }),
  })("rebuilds the integer as the sum of its set bits' weights", ({ bit, value }) => {
    const weights = Array.from({ length: 31 }, (unused, index) =>
      isBitSet(value, index) ? 2 ** index : 0,
    );

    expect(weights.reduce((sum, weight) => sum + weight, 0)).toBe(value);

    expect(isBitSet(value + 2 ** bit * (isBitSet(value, bit) ? -1 : 1), bit)).toBe(
      !isBitSet(value, bit),
    );
  });
});
