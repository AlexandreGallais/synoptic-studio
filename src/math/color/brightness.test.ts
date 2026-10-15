import { test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ANY_RGB, rgbOfBytes } from "../../../tests";

import { brightness } from "./brightness";

describe("brightness (REF-WIKI-HSL-HSV, DERIV-color-spaces step 5)", () => {
  it.each([
    // #ff8000: max(1, 0.50196, 0).
    { expected: 1, rgb: rgbOfBytes(255, 128, 0) },
    // #336699: max(0.2, 0.4, 0.6).
    { expected: 0.6, rgb: rgbOfBytes(51, 102, 153) },
    // #808080: 128/255.
    { expected: 128 / 255, rgb: rgbOfBytes(128, 128, 128) },
    { expected: 0, rgb: rgbOfBytes(0, 0, 0) },
  ])("[F16.AC3] is $expected", ({ expected, rgb }) => {
    expect(brightness(rgb)).toBeCloseTo(expected, 12);
  });

  test.prop({ rgb: ANY_RGB })("is one of the components, none above it", ({ rgb }) => {
    const components = [rgb.red, rgb.green, rgb.blue];

    expect(components).toContain(brightness(rgb));
    expect(components.every((component) => component <= brightness(rgb))).toBe(true);
  });
});
