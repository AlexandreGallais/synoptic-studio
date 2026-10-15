import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ARIAL_UNKERNED } from "../../../tests";

import { contentAreaHeight } from "./content-area-height";

describe("contentAreaHeight (REF-CSS-INLINE-3)", () => {
  it("[F14.AC3] spans Arial 16 from its ascender to its descender", () => {
    // (1854 + 434) × 16 / 2048 = 2288 / 128 = 17.875.
    expect(contentAreaHeight(ARIAL_UNKERNED, 16)).toBe(17.875);
  });

  test.prop({ size: fc.integer({ max: 100_000, min: 1 }) })(
    "grows with the size, in proportion",
    ({ size }) => {
      // 2288 font units of 2048 to the em.
      expect(contentAreaHeight(ARIAL_UNKERNED, size)).toBeCloseTo((2288 * size) / 2048, 6);
    },
  );
});
