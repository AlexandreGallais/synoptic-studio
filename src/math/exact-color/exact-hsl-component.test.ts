import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ANY_EXACT_COLOR } from "../../../tests";
import { hslComponent } from "../color";

import { exactHslComponent } from "./exact-hsl-component";

import type { ExactColor } from "./exact-color";

/**
 * An HSL color in tenths, as fractions.
 *
 * @param lightness - the lightness, in tenths of a percent
 * @returns HSL 0°, 75 %, at that lightness
 */
function redAt(lightness: number): ExactColor {
  return {
    hue: { denominator: 10, numerator: 0 },
    level: { denominator: 1000, numerator: lightness },
    saturation: { denominator: 1000, numerator: 750 },
  };
}

describe("exactHslComponent (DERIV-color-spaces steps 11 and 12)", () => {
  it.each([
    // HSL 0°, 75 %, 40 %, red: t = 300, K = 0, q = −300, a = 750 · 400,
    // N = 400 · 1000 · 300 + 300 000 · 300.
    { color: redAt(400), denominator: 300_000_000, n: 0, numerator: 210_000_000 },
    // Green: K = 2400, q = min(1500, 300) = 300, N = 1.2 · 10⁸ − 9 · 10⁷.
    { color: redAt(400), denominator: 300_000_000, n: 8, numerator: 30_000_000 },
    // HSL 0°, 75 %, 60 %, blue: K = 1200, q = 300, a = 750 · 400, N = 1.8 · 10⁸ − 9 · 10⁷.
    { color: redAt(600), denominator: 300_000_000, n: 4, numerator: 90_000_000 },
    // #336699's exact 210°, 102 / 204 with the lightness 41 %, red: t = 3060, K = 21 420, q = 3060;
    // a = 102 · min(41, 59) = 4182, N = 41 · 204 · 3060 − 4182 · 3060, D = 100 · 204 · 3060:
    // 0.41 − 0.5 · 0.41 = 0.205, 52.275 → 52.
    {
      color: {
        hue: { denominator: 102, numerator: 21_420 },
        level: { denominator: 100, numerator: 41 },
        saturation: { denominator: 204, numerator: 102 },
      },
      denominator: 62_424_000,
      n: 0,
      numerator: 12_796_920,
    },
  ])(
    "[F16.AC2] gives $numerator / $denominator for n = $n",
    ({ color, denominator, n, numerator }) => {
      expect(exactHslComponent(color, n)).toEqual({ denominator, numerator });
    },
  );

  test.prop({ color: ANY_EXACT_COLOR, n: fc.constantFrom(0, 4, 8) })(
    "is the component of hslComponent, exactly, 255 times its numerator a safe integer",
    ({ color, n }) => {
      const { denominator, numerator } = exactHslComponent(color, n);
      const hsl = {
        hue: color.hue.numerator / color.hue.denominator,
        lightness: (100 * color.level.numerator) / color.level.denominator,
        saturation: (100 * color.saturation.numerator) / color.saturation.denominator,
      };

      expect(Number.isSafeInteger(255 * numerator) && Number.isSafeInteger(denominator)).toBe(true);
      expect(numerator / denominator).toBeCloseTo(hslComponent(hsl, n), 12);
    },
  );
});
