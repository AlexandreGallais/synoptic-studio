import { describe, expect, it } from "vitest";

import { changedColorCount } from "../../../tests";
import { hexToRgb, rgbToHex } from "../color";

import { exactColorToHex } from "./exact-color-to-hex";
import { hexToExactColor } from "./hex-to-exact-color";

import type { Rgb } from "../../math";
import type { ColorModel } from "../../model";

/**
 * A color's components through its exact values and back, in a model.
 *
 * @param rgb - the color's components
 * @param model - HSB or HSL
 * @returns the components of the color rebuilt
 */
function exactRoundTrip(rgb: Rgb, model: ColorModel): Rgb {
  const exact = hexToExactColor(rgbToHex(rgb), model, 0);

  return hexToRgb(exactColorToHex(exact, model));
}

describe("exactColorToHex (DERIV-color-spaces steps 11 and 12)", () => {
  it("[F16.AC2] gives #004080 for HSB 210°, 100 %, 50 %", () => {
    // Red 0; green 0.5 − 0.5 · 0.5 = 0.25, 63.75 → 64 (40); blue 0.5, 127.5 → 128 (80).
    expect(
      exactColorToHex(
        {
          hue: { denominator: 1, numerator: 210 },
          level: { denominator: 100, numerator: 50 },
          saturation: { denominator: 100, numerator: 100 },
        },
        "hsb",
      ),
    ).toBe("#004080");
  });

  it("[F16.AC2] gives #34689c for #336699's exact 210°, 102 / 153 and the brightness 61 %", () => {
    // Red 0.61 · (1 − 2/3) = 0.20333, 51.85 → 52 (34); green 0.61 · (1 − 2/3 · 1/2) = 0.40667,
    // 103.7 → 104 (68); blue 0.61, 155.55 → 156 (9c).
    expect(
      exactColorToHex(
        {
          hue: { denominator: 102, numerator: 21_420 },
          level: { denominator: 100, numerator: 61 },
          saturation: { denominator: 153, numerator: 102 },
        },
        "hsb",
      ),
    ).toBe("#34689c");
  });

  it("[F16.AC2] gives #ff8000 for HSL 30°, 100 %, 50 %, the green 127.5 rounded up", () => {
    // Green: k = (8 + 30 / 30) mod 12 = 9, min(k − 3, 9 − k, 1) = 0, f(8) = L = 0.5, 127.5 → 128
    // (80); red: k = 1, clamped to −1, f = 0.5 + 0.5 = 1 (ff); blue: k = 5, 1, f = 0 (00).
    expect(
      exactColorToHex(
        {
          hue: { denominator: 1, numerator: 30 },
          level: { denominator: 100, numerator: 50 },
          saturation: { denominator: 100, numerator: 100 },
        },
        "hsl",
      ),
    ).toBe("#ff8000");
  });

  // The sample of DERIV-color-spaces (cross-check): one color in 31 and every sector edge, about
  // 731 000 colors, where a property's random colors would seldom fall on an edge.
  it.each(["hsb", "hsl"] as const)(
    "[F16.AC2] gives one color in 31 and every sector edge back from its exact %s values",
    // alone: 1655 ms, the median of 5 runs of its file for HSB, 1653 ms for HSL (EN-129); 30 s
    // is 18 times it. Four times it is over Vitest's 5 s: the sample keeps its size.
    { timeout: 30_000 },
    (model) => {
      expect(changedColorCount((rgb) => exactRoundTrip(rgb, model))).toBe(0);
    },
  );
});
