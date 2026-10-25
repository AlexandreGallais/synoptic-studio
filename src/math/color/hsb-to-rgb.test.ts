import { test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ANY_RGB, changedColorCount } from "../../../tests";

import { hsbToRgb } from "./hsb-to-rgb";
import { rgbToHsb } from "./rgb-to-hsb";

describe("hsbToRgb (REF-WIKI-HSL-HSV, DERIV-color-spaces steps 6 and 7)", () => {
  it("[F16.AC3] gives HSB(210°, 67 %, 60 %) as 0.198, 0.399, 0.6", () => {
    // V · S = 0.402; k = 2.5, 0.5, 4.5: f(5) = 0.6 − 0.402, f(3) = 0.6 − 0.201, f(1) = 0.6.
    const rgb = hsbToRgb({ brightness: 60, hue: 210, saturation: 67 });

    expect(rgb.red).toBeCloseTo(0.198, 12);
    expect(rgb.green).toBeCloseTo(0.399, 12);
    expect(rgb.blue).toBeCloseTo(0.6, 12);
  });

  test.prop({ rgb: ANY_RGB })(
    "[F16.AC3] gives back the components HSB was read from (step 7)",
    ({ rgb }) => {
      const back = hsbToRgb(rgbToHsb(rgb));

      expect(back.red).toBeCloseTo(rgb.red, 12);
      expect(back.green).toBeCloseTo(rgb.green, 12);
      expect(back.blue).toBeCloseTo(rgb.blue, 12);
    },
  );

  it(
    "[F16.AC3] gives one color in 31 and every sector edge back from its HSB, unrounded",
    // alone: 931 ms, the median of 5 runs of its file; 20 s is 21 times it.
    { timeout: 20_000 },
    () => {
      expect(changedColorCount((rgb) => hsbToRgb(rgbToHsb(rgb)))).toBe(0);
    },
  );
});
