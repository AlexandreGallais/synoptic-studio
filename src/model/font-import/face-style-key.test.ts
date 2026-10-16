import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { faceStyleKey } from "./face-style-key";

describe("faceStyleKey (REF-CSS-FONTS-4 §4)", () => {
  it.each([
    { expected: "400/normal", isItalic: false, weight: 400 },
    { expected: "700/italic", isItalic: true, weight: 700 },
    { expected: "450/normal", isItalic: false, weight: 450 },
  ])("keys weight $weight, italic $isItalic as $expected", ({ expected, isItalic, weight }) => {
    expect(faceStyleKey({ isItalic, weight })).toBe(expected);
  });

  test.prop({
    first: fc.record({ isItalic: fc.boolean(), weight: fc.integer({ max: 900, min: 100 }) }),
    second: fc.record({ isItalic: fc.boolean(), weight: fc.integer({ max: 900, min: 100 }) }),
  })("gives two styles one key exactly when weight and style are equal", ({ first, second }) => {
    const isSameStyle = first.weight === second.weight && first.isItalic === second.isItalic;

    expect(faceStyleKey(first) === faceStyleKey(second)).toBe(isSameStyle);
  });
});
