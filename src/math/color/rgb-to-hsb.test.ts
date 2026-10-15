import { describe, expect, it } from "vitest";

import { rgbOfBytes } from "../../../tests";

import { rgbToHsb } from "./rgb-to-hsb";

describe("rgbToHsb (REF-WIKI-HSL-HSV, DERIV-color-spaces steps 3 and 5)", () => {
  it.each([
    // #ff8000: H = 60 · 128/255 = 30.11765, S = 1 / 1, V = 1.
    {
      expected: { brightness: 100, hue: 30.117647, saturation: 100 },
      rgb: rgbOfBytes(255, 128, 0),
    },
    // #336699: H = 210, S = 0.4 / 0.6 = 66.66667 %, V = 0.6.
    {
      expected: { brightness: 60, hue: 210, saturation: 66.666667 },
      rgb: rgbOfBytes(51, 102, 153),
    },
    // #808080, a grey: hue 0, S = 0, V = 128/255 = 50.19608 %.
    { expected: { brightness: 50.196078, hue: 0, saturation: 0 }, rgb: rgbOfBytes(128, 128, 128) },
    // #000000 and #ffffff: hue 0, saturation 0.
    { expected: { brightness: 0, hue: 0, saturation: 0 }, rgb: rgbOfBytes(0, 0, 0) },
    { expected: { brightness: 100, hue: 0, saturation: 0 }, rgb: rgbOfBytes(255, 255, 255) },
  ])(
    "[F16.AC3] gives HSB $expected.hue°, $expected.saturation %, $expected.brightness %",
    ({ expected, rgb }) => {
      const hsb = rgbToHsb(rgb);

      expect(hsb.hue).toBeCloseTo(expected.hue, 5);
      expect(hsb.saturation).toBeCloseTo(expected.saturation, 5);
      expect(hsb.brightness).toBeCloseTo(expected.brightness, 5);
    },
  );
});
