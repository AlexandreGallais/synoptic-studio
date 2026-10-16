import { test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ANY_RGB, rgbOfBytes } from "../../../tests";

import { hslSaturation } from "./hsl-saturation";

describe("hslSaturation (REF-CSS-COLOR-4 §7.2, DERIV-color-spaces step 4)", () => {
  it.each([
    // #ff8000: L = 0.5, (1 − 0.5) / min(0.5, 0.5) = 1.
    { expected: 1, rgb: rgbOfBytes(255, 128, 0) },
    // #336699: L = 0.4, (0.6 − 0.4) / min(0.4, 0.6) = 0.5.
    { expected: 0.5, rgb: rgbOfBytes(51, 102, 153) },
    // #cc9999: L = (0.8 + 0.6) / 2 = 0.7, (0.8 − 0.7) / min(0.7, 0.3) = 1/3, past half lightness.
    { expected: 1 / 3, rgb: rgbOfBytes(204, 153, 153) },
    // #808080, a grey: (L − L) / min(L, 1 − L) = 0.
    { expected: 0, rgb: rgbOfBytes(128, 128, 128) },
  ])("[F16.AC3] is $expected", ({ expected, rgb }) => {
    expect(hslSaturation(rgb)).toBeCloseTo(expected, 12);
  });

  it.each([
    { name: "#000000", rgb: rgbOfBytes(0, 0, 0) },
    { name: "#ffffff", rgb: rgbOfBytes(255, 255, 255) },
  ])("[F16.AC3] is 0 for $name, its lightness 0 or 1", ({ rgb }) => {
    expect(hslSaturation(rgb)).toBe(0);
  });

  test.prop({ rgb: ANY_RGB })("is from 0 to 1", ({ rgb }) => {
    expect(hslSaturation(rgb)).toBeGreaterThanOrEqual(0);
    expect(hslSaturation(rgb)).toBeLessThanOrEqual(1 + 1e-12);
  });
});
