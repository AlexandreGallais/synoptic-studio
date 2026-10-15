import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ANY_EXACT_COLOR } from "../../../tests";
import { hsbComponent } from "../color";

import { exactHsbComponent } from "./exact-hsb-component";

/** HSB 30.1°, 90 %, 100 %, each value in tenths. */
const TIE = {
  hue: { denominator: 10, numerator: 301 },
  level: { denominator: 1000, numerator: 1000 },
  saturation: { denominator: 1000, numerator: 900 },
};

/** `#336699`'s exact hue and saturation, 210° and 102 / 153, with the brightness 61 %. */
const BRIGHTER = {
  hue: { denominator: 102, numerator: 21_420 },
  level: { denominator: 100, numerator: 61 },
  saturation: { denominator: 153, numerator: 102 },
};

describe("exactHsbComponent (DERIV-color-spaces steps 11 and 12)", () => {
  it.each([
    // Blue: t = 600, K = 600 + 301 = 901, p = min(901, 1499) = 600, N = 1000 · (6 · 10⁵ − 900 · 600).
    { color: TIE, denominator: 600_000_000, n: 1, numerator: 60_000_000 },
    // Green: K = 2101, p = min(2101, 299) = 299, N = 1000 · (600 000 − 900 · 299) = 330 900 000.
    { color: TIE, denominator: 600_000_000, n: 3, numerator: 330_900_000 },
    // Red: K = 3301, min(3301, −901) clamped to 0, N = 1000 · 600 000.
    { color: TIE, denominator: 600_000_000, n: 5, numerator: 600_000_000 },
    // Red: t = 6120, K = (30 600 + 21 420) mod 36 720 = 15 300, p = min(15 300, 9180) → 6120;
    // N = 61 · (153 − 102) · 6120 = 19 039 320, D = 100 · 153 · 6120: 0.20333, 51.85 → 52.
    { color: BRIGHTER, denominator: 93_636_000, n: 5, numerator: 19_039_320 },
    // Green: K = 18 360 + 21 420 = 39 780 mod 36 720 = 3060, p = 3060;
    // N = 61 · (153 · 6120 − 102 · 3060) = 38 078 640: 0.40667, 103.7 → 104.
    { color: BRIGHTER, denominator: 93_636_000, n: 3, numerator: 38_078_640 },
  ])(
    "[F16.AC2] gives $numerator / $denominator for n = $n",
    ({ color, denominator, n, numerator }) => {
      expect(exactHsbComponent(color, n)).toEqual({ denominator, numerator });
    },
  );

  test.prop({ color: ANY_EXACT_COLOR, n: fc.constantFrom(1, 3, 5) })(
    "is the component of hsbComponent, exactly, 255 times its numerator a safe integer",
    ({ color, n }) => {
      const { denominator, numerator } = exactHsbComponent(color, n);
      const hsb = {
        brightness: (100 * color.level.numerator) / color.level.denominator,
        hue: color.hue.numerator / color.hue.denominator,
        saturation: (100 * color.saturation.numerator) / color.saturation.denominator,
      };

      expect(Number.isSafeInteger(255 * numerator) && Number.isSafeInteger(denominator)).toBe(true);
      expect(numerator / denominator).toBeCloseTo(hsbComponent(hsb, n), 12);
    },
  );
});
