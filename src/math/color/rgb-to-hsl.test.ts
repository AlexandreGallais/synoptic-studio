import { describe, expect, it } from "vitest";

import { rgbOfBytes } from "../../../tests";

import { rgbToHsl } from "./rgb-to-hsl";

describe("rgbToHsl (REF-CSS-COLOR-4 §7.2, DERIV-color-spaces steps 3 and 4)", () => {
  it.each([
    // #ff8000: H = 60 · 128/255 = 30.11765, S = 1, L = 0.5.
    { expected: { hue: 30.117647, lightness: 50, saturation: 100 }, rgb: rgbOfBytes(255, 128, 0) },
    // #336699: H = 60 · 3.5, S = 0.2 / 0.4, L = 0.4.
    { expected: { hue: 210, lightness: 40, saturation: 50 }, rgb: rgbOfBytes(51, 102, 153) },
    // #808080, a grey: hue 0, S = 0, L = 128/255 = 50.19608 %.
    { expected: { hue: 0, lightness: 50.196078, saturation: 0 }, rgb: rgbOfBytes(128, 128, 128) },
    // #000000 and #ffffff: hue 0, saturation 0.
    { expected: { hue: 0, lightness: 0, saturation: 0 }, rgb: rgbOfBytes(0, 0, 0) },
    { expected: { hue: 0, lightness: 100, saturation: 0 }, rgb: rgbOfBytes(255, 255, 255) },
  ])(
    "[F16.AC3] gives HSL $expected.hue°, $expected.saturation %, $expected.lightness %",
    ({ expected, rgb }) => {
      const hsl = rgbToHsl(rgb);

      expect(hsl.hue).toBeCloseTo(expected.hue, 5);
      expect(hsl.saturation).toBeCloseTo(expected.saturation, 5);
      expect(hsl.lightness).toBeCloseTo(expected.lightness, 5);
    },
  );
});
