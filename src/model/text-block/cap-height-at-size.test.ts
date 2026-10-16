import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ARIAL_UNKERNED } from "../../../tests";

import { capHeightAtSize } from "./cap-height-at-size";

describe("capHeightAtSize (DERIV-text-box step 7)", () => {
  it("[F14.AC5] puts Arial's capitals 1467 / 128 above the baseline at 16, twice that at 32", () => {
    // 1467 × 16 / 2048 = 11.4609375; 1467 × 32 / 2048 = 22.921875.
    expect(capHeightAtSize(ARIAL_UNKERNED, 16)).toBe(11.4609375);
    expect(capHeightAtSize(ARIAL_UNKERNED, 32)).toBe(22.921875);
  });

  test.prop({ size: fc.integer({ max: 100_000, min: 1 }) })(
    "grows with the size, in proportion",
    ({ size }) => {
      expect(capHeightAtSize(ARIAL_UNKERNED, size)).toBeCloseTo((1467 * size) / 2048, 6);
    },
  );
});
