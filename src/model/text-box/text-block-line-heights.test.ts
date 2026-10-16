import { describe, expect, it } from "vitest";

import { ARIAL_BOX_METRICS, HELLO_TEXT } from "../../../tests";

import { textBlockLineHeights } from "./text-block-line-heights";

describe("textBlockLineHeights (DERIV-text-box steps 5 and 7)", () => {
  it("[F14.AC2] counts lines with the font's line height when automatic", () => {
    // (1854 + 434 + 67) × 16 / 2048 = 2355 / 128 = 18.3984375.
    expect(textBlockLineHeights(ARIAL_BOX_METRICS, HELLO_TEXT)).toEqual({
      firstLine: 18.3984375,
      line: 18.3984375,
    });
  });

  it("[F14.AC3] counts lines with the line height chosen", () => {
    expect(textBlockLineHeights(ARIAL_BOX_METRICS, { ...HELLO_TEXT, lineHeight: 24 })).toEqual({
      firstLine: 24,
      line: 24,
    });
  });

  it("[F14.AC5] starts a trimmed block's first line at the cap height", () => {
    // 1467 × 16 / 2048 = 11.4609375, the lines after it 18.3984375 apart.
    expect(textBlockLineHeights(ARIAL_BOX_METRICS, { ...HELLO_TEXT, isTrimmed: true })).toEqual({
      firstLine: 11.4609375,
      line: 18.3984375,
    });
  });
});
