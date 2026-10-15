import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ARIAL_UNKERNED } from "../../../tests";

import { contentAreaTop } from "./content-area-top";

describe("contentAreaTop (REF-CSS-INLINE-3 §5.3, §6.4)", () => {
  it("[F14.AC4] tops Arial 16's first line at 0 and its second at 18.3984375", () => {
    // Ascent 1854 × 16 / 2048 = 14.484375: 14.484375 − 14.484375 = 0 and
    // 32.8828125 − 14.484375 = 18.3984375.
    expect(contentAreaTop(ARIAL_UNKERNED, 16, 14.484375)).toBe(0);
    expect(contentAreaTop(ARIAL_UNKERNED, 16, 32.8828125)).toBe(18.3984375);
  });

  it("[F14.AC4] rises above 0 for a baseline closer to the top than the ascent", () => {
    // A trimmed first baseline 11.4609375: 11.4609375 − 14.484375 = −3.0234375.
    expect(contentAreaTop(ARIAL_UNKERNED, 16, 11.4609375)).toBe(-3.0234375);
  });

  test.prop({
    baseline: fc.integer({ max: 100_000, min: -100_000 }),
    size: fc.integer({ max: 100_000, min: 1 }),
  })("lies an ascent above the baseline", ({ baseline, size }) => {
    expect(baseline - contentAreaTop(ARIAL_UNKERNED, size, baseline)).toBeCloseTo(
      (1854 * size) / 2048,
      6,
    );
  });
});
