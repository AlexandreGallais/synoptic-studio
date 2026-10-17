import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { lengthToFourByteBoundary } from "./length-to-four-byte-boundary";

describe("lengthToFourByteBoundary (REF-OT-OTFF)", () => {
  it.each([
    { length: 0, padded: 0 },
    // ⌈1 / 4⌉ · 4 = 4; ⌈3 / 4⌉ · 4 = 4.
    { length: 1, padded: 4 },
    { length: 3, padded: 4 },
    // Already on a boundary: kept.
    { length: 4, padded: 4 },
    // ⌈5 / 4⌉ · 4 = 8.
    { length: 5, padded: 8 },
    // Helvetica's `name` table and the like: ⌈1 029 / 4⌉ · 4 = 1 032.
    { length: 1029, padded: 1032 },
  ])("pads $length bytes to $padded", ({ length, padded }) => {
    expect(lengthToFourByteBoundary(length)).toBe(padded);
  });

  test.prop({ length: fc.nat() })(
    "gives the smallest multiple of 4 at least the length",
    ({ length }) => {
      const padded = lengthToFourByteBoundary(length);

      expect(padded % 4).toBe(0);
      expect(padded).toBeGreaterThanOrEqual(length);
      expect(padded - length).toBeLessThan(4);
    },
  );
});
