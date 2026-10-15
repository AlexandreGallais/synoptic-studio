import { describe, expect, it } from "vitest";

import { rgbToHsb, rgbToHsl } from "../../math";

import { hexToRgb } from "./hex-to-rgb";

describe("hexToRgb (REF-CSS-COLOR-4 §5.2, DERIV-color-spaces notation)", () => {
  it("[F16.AC3] reads #336699 as 0.2, 0.4, 0.6", () => {
    // 0x33 = 51, 51 / 255 = 0.2; 0x66 = 102, 0.4; 0x99 = 153, 0.6.
    expect(hexToRgb("#336699")).toEqual({ blue: 153 / 255, green: 102 / 255, red: 51 / 255 });
  });

  it("[F16.AC3] gives #ff8000 as HSL 30.11765°, 100 %, 50 % and HSB 30.11765°, 100 %, 100 %", () => {
    const hsl = rgbToHsl(hexToRgb("#ff8000"));
    const hsb = rgbToHsb(hexToRgb("#ff8000"));

    expect([hsl.hue, hsl.saturation, hsl.lightness]).toEqual([60 * (128 / 255), 100, 50]);
    expect([hsb.hue, hsb.saturation, hsb.brightness]).toEqual([60 * (128 / 255), 100, 100]);
  });
});
