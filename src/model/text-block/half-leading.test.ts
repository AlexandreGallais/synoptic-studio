import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ARIAL_UNKERNED, HELLO_TEXT } from "../../../tests";

import { contentAreaHeight } from "./content-area-height";
import { halfLeading } from "./half-leading";

describe("halfLeading (REF-CSS-INLINE-3 §5.3)", () => {
  it("[F14.AC3] puts half of the room a line height of 24 leaves above Arial 16's letters", () => {
    // (24 − 17.875) / 2 = 3.0625.
    expect(halfLeading(ARIAL_UNKERNED, { ...HELLO_TEXT, lineHeight: 24 })).toBe(3.0625);
  });

  it("[F14.AC3] is negative for a line height smaller than the letters", () => {
    // "L may be negative": (10 − 17.875) / 2 = −3.9375.
    expect(halfLeading(ARIAL_UNKERNED, { ...HELLO_TEXT, lineHeight: 10 })).toBe(-3.9375);
  });

  test.prop({
    lineHeight: fc.integer({ max: 100_000, min: 1 }),
    size: fc.integer({ max: 100_000, min: 1 }),
  })("adds up, twice, with the letters to the line height", ({ lineHeight, size }) => {
    const text = { ...HELLO_TEXT, lineHeight, size };

    expect(
      2 * halfLeading(ARIAL_UNKERNED, text) + contentAreaHeight(ARIAL_UNKERNED, size),
    ).toBeCloseTo(lineHeight, 6);
  });
});
