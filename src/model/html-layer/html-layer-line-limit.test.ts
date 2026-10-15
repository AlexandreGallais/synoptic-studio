import { describe, expect, it } from "vitest";

import { ARIAL_UNKERNED, HELLO_TEXT } from "../../../tests";

import { htmlLayerLineLimit } from "./html-layer-line-limit";

/** "Hello World" in Arial 16 in a box 50 × 40. */
const BOXED = { ...HELLO_TEXT, boxHeight: 40, boxWidth: 50, font: "Arial", isInBox: true };

describe("htmlLayerLineLimit (DERIV-text-box, step 5)", () => {
  it.each([
    // 40 tall: ⌊(40 − 18.3984375) / 18.3984375⌋ + 1 = 2 lines.
    { limit: 2, options: {} },
    // A maximum of 1 line, under the 2 the box holds.
    { limit: 1, options: { maximumLines: 1 } },
    // Trimmed: ⌊(40 − 11.4609375) / 18.3984375⌋ + 1 = 2 lines.
    { limit: 2, options: { isTrimmed: true } },
    // 100 tall, line height 24: ⌊(100 − 24) / 24⌋ + 1 = 4 lines.
    { limit: 4, options: { boxHeight: 100, lineHeight: 24 } },
    // Shorter than a line: at least one.
    { limit: 1, options: { boxHeight: 5 } },
  ])("shows at most $limit lines in a box ($options)", ({ limit, options }) => {
    expect(htmlLayerLineLimit(ARIAL_UNKERNED, { ...BOXED, ...options })).toBe(limit);
  });

  it("sets no limit for a text that fits its content", () => {
    expect(htmlLayerLineLimit(ARIAL_UNKERNED, { ...BOXED, isInBox: false, maximumLines: 1 })).toBe(
      0,
    );
  });

  it("sets no limit for lines that take no height, without maximum, and the maximum with one", () => {
    const flat = { ...ARIAL_UNKERNED, ascender: 0, descender: 0, lineGap: 0 };

    expect(htmlLayerLineLimit(flat, BOXED)).toBe(0);
    expect(htmlLayerLineLimit(flat, { ...BOXED, maximumLines: 3 })).toBe(3);
  });
});
