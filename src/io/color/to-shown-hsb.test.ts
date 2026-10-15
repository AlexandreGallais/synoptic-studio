import { test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ANY_RGB, changedColorCount } from "../../../tests";
import { hsbToRgb, rgbToHsb } from "../../math";

import { hexToRgb } from "./hex-to-rgb";
import { rgbToHex } from "./rgb-to-hex";
import { toShownHsb } from "./to-shown-hsb";

describe("toShownHsb (DERIV-color-spaces step 9, ADR-0056)", () => {
  it("[F16.AC3] shows #ff8000 as HSB 30.1°, 100 %, 100 %", () => {
    // 60 · 128/255 = 30.11765 → 30.1; 100; 100.
    const hsb = rgbToHsb(hexToRgb("#ff8000"));

    expect(toShownHsb(hsb)).toEqual({
      brightness: 100,
      hue: 30.1,
      saturation: 100,
    });
  });

  it("[F16.AC3] shows #336699 as HSB 210°, 66.7 %, 60 %, which gives #336699 back", () => {
    // 0.4 / 0.6 = 66.66667 % → 66.7 %; f(5) = 0.6 − 0.6 · 0.667 = 0.1998, 50.949 → 51.
    const hsb = rgbToHsb(hexToRgb("#336699"));
    const shown = toShownHsb(hsb);

    expect(shown).toEqual({ brightness: 60, hue: 210, saturation: 66.7 });
    expect(rgbToHex(hsbToRgb(shown))).toBe("#336699");
  });

  it("[F16.AC3] shows a hue that rounds to 360.0 as 0.0", () => {
    expect(toShownHsb({ brightness: 50, hue: 359.96, saturation: 100 })).toHaveProperty("hue", 0);
  });

  it(
    "[F16.AC3] gives one color in 31 and every sector edge back from its HSB, unrounded",
    { timeout: 20_000 },
    () => {
      expect(changedColorCount((rgb) => hsbToRgb(rgbToHsb(rgb)))).toBe(0);
    },
  );

  it(
    "[F16.AC3] gives one color in 31 and every sector edge back from its HSB shown",
    { timeout: 20_000 },
    () => {
      expect(changedColorCount((rgb) => hsbToRgb(toShownHsb(rgbToHsb(rgb))))).toBe(0);
    },
  );

  test.prop({ rgb: ANY_RGB })(
    "[F16.AC3] gives any color's hexadecimal back from its HSB shown to one decimal",
    ({ rgb }) => {
      const shown = toShownHsb(rgbToHsb(rgb));

      expect(rgbToHex(hsbToRgb(shown))).toBe(rgbToHex(rgb));
    },
  );
});
