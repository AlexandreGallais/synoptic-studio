import { test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ANY_RGB, changedColorCount } from "../../../tests";
import { hslToRgb, rgbToHsl } from "../../math";

import { hexToRgb } from "./hex-to-rgb";
import { rgbToHex } from "./rgb-to-hex";
import { toShownHsl } from "./to-shown-hsl";

describe("toShownHsl (DERIV-color-spaces step 9, ADR-0056)", () => {
  it("[F16.AC3] shows #ff8000 as HSL 30.1°, 100 %, 50 %", () => {
    // 60 · 128/255 = 30.11765 → 30.1; 100; 50.
    const hsl = rgbToHsl(hexToRgb("#ff8000"));

    expect(toShownHsl(hsl)).toEqual({
      hue: 30.1,
      lightness: 50,
      saturation: 100,
    });
  });

  it("[F16.AC3] shows a hue that rounds to 360.0 as 0.0", () => {
    expect(toShownHsl({ hue: 359.96, lightness: 50, saturation: 100 })).toHaveProperty("hue", 0);
  });

  it("[F16.AC3] gives #ff8000 back from HSL 30.1°, 100 %, 50 %", () => {
    // f(8) = 0.5 + 0.5 · (30.1/30 − 1) = 0.50167, 255 · 0.50167 = 127.925 → 128.
    expect(rgbToHex(hslToRgb({ hue: 30.1, lightness: 50, saturation: 100 }))).toBe("#ff8000");
  });

  it(
    "[F16.AC3] gives one color in 31 and every sector edge back from its HSL, unrounded and shown",
    { timeout: 10_000 },
    () => {
      expect(changedColorCount((rgb) => hslToRgb(rgbToHsl(rgb)))).toBe(0);
      expect(changedColorCount((rgb) => hslToRgb(toShownHsl(rgbToHsl(rgb))))).toBe(0);
    },
  );

  test.prop({ rgb: ANY_RGB })(
    "[F16.AC3] gives any color's hexadecimal back from its HSL shown to one decimal",
    ({ rgb }) => {
      const shown = toShownHsl(rgbToHsl(rgb));

      expect(rgbToHex(hslToRgb(shown))).toBe(rgbToHex(rgb));
    },
  );
});
