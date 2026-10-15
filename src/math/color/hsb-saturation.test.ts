import { test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ANY_RGB, rgbOfBytes } from "../../../tests";

import { hsbSaturation } from "./hsb-saturation";

describe("hsbSaturation (REF-WIKI-HSL-HSV, DERIV-color-spaces step 5)", () => {
  it.each([
    // #ff8000: C / V = 1 / 1.
    { expected: 1, rgb: rgbOfBytes(255, 128, 0) },
    // #336699: 0.4 / 0.6 = 0.66667.
    { expected: 2 / 3, rgb: rgbOfBytes(51, 102, 153) },
    // #808080, a grey: 0 / 0.50196.
    { expected: 0, rgb: rgbOfBytes(128, 128, 128) },
    // #ffffff: 0 / 1.
    { expected: 0, rgb: rgbOfBytes(255, 255, 255) },
  ])("[F16.AC3] is $expected", ({ expected, rgb }) => {
    expect(hsbSaturation(rgb)).toBeCloseTo(expected, 12);
  });

  it("[F16.AC3] is 0 for #000000, whose brightness is 0", () => {
    expect(hsbSaturation(rgbOfBytes(0, 0, 0))).toBe(0);
  });

  test.prop({ rgb: ANY_RGB })("is from 0 to 1", ({ rgb }) => {
    expect(hsbSaturation(rgb)).toBeGreaterThanOrEqual(0);
    expect(hsbSaturation(rgb)).toBeLessThanOrEqual(1);
  });
});
