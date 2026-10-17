import { test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ANY_RGB, rgbOfBytes } from "../../../tests";

import { lightness } from "./lightness";

describe("lightness (REF-CSS-COLOR-4 §7.2, DERIV-color-spaces step 4)", () => {
  it.each([
    // #ff8000: (1 + 0) / 2.
    { expected: 0.5, rgb: rgbOfBytes(255, 128, 0) },
    // #336699: (0.6 + 0.2) / 2.
    { expected: 0.4, rgb: rgbOfBytes(51, 102, 153) },
    // #808080: (128/255 + 128/255) / 2 = 0.50196.
    { expected: 128 / 255, rgb: rgbOfBytes(128, 128, 128) },
    { expected: 0, rgb: rgbOfBytes(0, 0, 0) },
    { expected: 1, rgb: rgbOfBytes(255, 255, 255) },
  ])("[F16.AC3] is $expected", ({ expected, rgb }) => {
    expect(lightness(rgb)).toBeCloseTo(expected, 12);
  });

  test.prop({ rgb: ANY_RGB })("lies between the smallest and largest component", ({ rgb }) => {
    const components = [rgb.red, rgb.green, rgb.blue];

    expect(lightness(rgb)).toBeGreaterThanOrEqual(Math.min(...components));
    expect(lightness(rgb)).toBeLessThanOrEqual(Math.max(...components));
  });
});
