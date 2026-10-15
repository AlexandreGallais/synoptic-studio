import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ARIAL_UNKERNED, HELLO_TEXT } from "../../../tests";
import { firstBaselineOffset } from "../text-layout";

import { cssFirstBaselineOffset } from "./css-first-baseline-offset";

/** A text in Arial 16. */
const ARIAL_TEXT = { ...HELLO_TEXT, font: "Arial" };

describe("cssFirstBaselineOffset (DERIV-html-layer, step 2)", () => {
  it.each([
    // k = 16 / 2048 = 0.0078125; ascent 0.0078125 · 1854 = 14.484375; content area
    // 0.0078125 · 2288 = 17.875. Automatic: λ = 0.0078125 · 2355 = 18.3984375, half-leading
    // (18.3984375 − 17.875) / 2 = 0.26171875, so 14.74609375 (the cross-check).
    { lineHeight: 0, offset: 14.74609375 },
    // Line height 24: (24 − 17.875) / 2 = 3.0625, so 17.546875, the library's too.
    { lineHeight: 24, offset: 17.546875 },
    // Line height 10, smaller than the content area: (10 − 17.875) / 2 = −3.9375, so 10.546875.
    { lineHeight: 10, offset: 10.546875 },
  ])("puts Arial 16's first baseline $offset down at a line height of $lineHeight", (row) => {
    expect(
      cssFirstBaselineOffset(ARIAL_UNKERNED, { ...ARIAL_TEXT, lineHeight: row.lineHeight }),
    ).toBe(row.offset);
  });

  it("lies half the line gap below the library's for an automatic line height", () => {
    // 14.74609375 − 14.484375 = 0.26171875 = 0.0078125 · 67 / 2.
    expect(
      cssFirstBaselineOffset(ARIAL_UNKERNED, ARIAL_TEXT) -
        firstBaselineOffset(ARIAL_UNKERNED, ARIAL_TEXT),
    ).toBe(0.26171875);
  });

  it("meets the library's for a face without line gap, as Inter 4.1", () => {
    const noGap = { ...ARIAL_UNKERNED, lineGap: 0 };

    expect(cssFirstBaselineOffset(noGap, ARIAL_TEXT)).toBe(firstBaselineOffset(noGap, ARIAL_TEXT));
  });

  test.prop({
    lineHeight: fc.integer({ max: 1000, min: 1 }),
    size: fc.integer({ max: 1000, min: 1 }),
  })("meets the library's first baseline for any line height chosen, untrimmed", (typed) => {
    const text = { ...ARIAL_TEXT, ...typed };

    expect(cssFirstBaselineOffset(ARIAL_UNKERNED, text)).toBeCloseTo(
      firstBaselineOffset(ARIAL_UNKERNED, text),
      9,
    );
  });
});
