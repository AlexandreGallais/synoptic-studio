import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ARIAL_UNKERNED, HELLO_TEXT } from "../../../tests";

import { textLineHeight } from "./text-line-height";

describe("textLineHeight (DERIV-text-box step 7)", () => {
  it("[F14.AC6] gives the font's line height when automatic, as in F05", () => {
    // (1854 + 434 + 67) × 16 / 2048 = 2355 / 128 = 18.3984375.
    expect(textLineHeight(ARIAL_UNKERNED, HELLO_TEXT)).toBe(18.3984375);
  });

  it("[F14.AC3] gives the line height chosen, whatever the font", () => {
    expect(textLineHeight(ARIAL_UNKERNED, { ...HELLO_TEXT, lineHeight: 24 })).toBe(24);
  });

  test.prop({
    lineHeight: fc.integer({ max: 100_000, min: 1 }),
    size: fc.integer({ max: 100_000, min: 1 }),
  })("gives a chosen line height as it is, at any size", ({ lineHeight, size }) => {
    expect(textLineHeight(ARIAL_UNKERNED, { ...HELLO_TEXT, lineHeight, size })).toBe(lineHeight);
  });
});
