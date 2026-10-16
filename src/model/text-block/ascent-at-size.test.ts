import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ARIAL_UNKERNED } from "../../../tests";

import { ascentAtSize } from "./ascent-at-size";

describe("ascentAtSize (REF-CSS-INLINE-3 §5.3)", () => {
  it("[F14.AC4] puts Arial's ascender 1854 / 128 above the baseline at 16, twice that at 32", () => {
    // 1854 × 16 / 2048 = 14.484375; 1854 × 32 / 2048 = 28.96875.
    expect(ascentAtSize(ARIAL_UNKERNED, 16)).toBe(14.484375);
    expect(ascentAtSize(ARIAL_UNKERNED, 32)).toBe(28.96875);
  });

  test.prop({ size: fc.integer({ max: 100_000, min: 1 }) })(
    "grows with the size, in proportion",
    ({ size }) => {
      expect(ascentAtSize(ARIAL_UNKERNED, size)).toBeCloseTo((1854 * size) / 2048, 6);
    },
  );
});
