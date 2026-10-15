import { describe, expect, it } from "vitest";

import { rgbOfBytes } from "../../../tests";
import { hsbToRgb, hslToRgb } from "../../math";

import { rgbToHex } from "./rgb-to-hex";

describe("rgbToHex (REF-CSS-COLOR-4 §16.2.1, DERIV-color-spaces step 8)", () => {
  it.each([
    { expected: "#336699", rgb: rgbOfBytes(51, 102, 153) },
    // Small bytes keep their leading zero: 1 is `01`, 10 is `0a`.
    { expected: "#00010a", rgb: rgbOfBytes(0, 1, 10) },
    { expected: "#ffffff", rgb: rgbOfBytes(255, 255, 255) },
  ])("[F16.AC1] writes $expected in six lower-case digits", ({ expected, rgb }) => {
    expect(rgbToHex(rgb)).toBe(expected);
  });

  it.each([
    // f(5) = 0.6 − 0.6 · 0.667 = 0.1998, 255 · 0.1998 = 50.949 → 51, `33`.
    { expected: "#336699", hsb: { brightness: 60, hue: 210, saturation: 66.7 } },
    // f(5) = 0.198, 255 · 0.198 = 50.49 → 50, `32`.
    { expected: "#326699", hsb: { brightness: 60, hue: 210, saturation: 67 } },
  ])("[F16.AC3] gives HSB $hsb.saturation % back as $expected", ({ expected, hsb }) => {
    expect(rgbToHex(hsbToRgb(hsb))).toBe(expected);
  });

  it("[F16.AC3] gives HSL(30°, 100 %, 50 %) back as #ff8000, the green 127.5 rounded up", () => {
    expect(rgbToHex(hslToRgb({ hue: 30, lightness: 50, saturation: 100 }))).toBe("#ff8000");
  });
});
