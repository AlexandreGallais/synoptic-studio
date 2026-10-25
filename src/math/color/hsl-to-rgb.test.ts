import { test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ANY_RGB, changedColorCount } from "../../../tests";

import { hslToRgb } from "./hsl-to-rgb";
import { rgbToHsl } from "./rgb-to-hsl";

describe("hslToRgb (REF-CSS-COLOR-4 §7.1, DERIV-color-spaces steps 6 and 7)", () => {
  it("[F16.AC3] gives HSL(210°, 50 %, 40 %) as 0.2, 0.4, 0.6, #336699", () => {
    // a = 0.5 · 0.4 = 0.2; k = 7, 3, 11: f(0) = 0.4 − 0.2 = 0.2, f(8) = 0.4 − 0.2 · 0 = 0.4,
    // f(4) = 0.4 + 0.2 = 0.6.
    const rgb = hslToRgb({ hue: 210, lightness: 40, saturation: 50 });

    expect(rgb.red).toBeCloseTo(0.2, 12);
    expect(rgb.green).toBeCloseTo(0.4, 12);
    expect(rgb.blue).toBeCloseTo(0.6, 12);
  });

  test.prop({ rgb: ANY_RGB })(
    "[F16.AC3] gives back the components HSL was read from (step 7)",
    ({ rgb }) => {
      const back = hslToRgb(rgbToHsl(rgb));

      expect(back.red).toBeCloseTo(rgb.red, 12);
      expect(back.green).toBeCloseTo(rgb.green, 12);
      expect(back.blue).toBeCloseTo(rgb.blue, 12);
    },
  );

  it(
    "[F16.AC3] gives one color in 31 and every sector edge back from its HSL, unrounded",
    // alone: 1006 ms, the median of 5 runs of its file; 20 s is 19 times it.
    { timeout: 20_000 },
    () => {
      expect(changedColorCount((rgb) => hslToRgb(rgbToHsl(rgb)))).toBe(0);
    },
  );
});
