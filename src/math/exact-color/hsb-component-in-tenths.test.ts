import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { hsbComponent } from "../color";

import { hsbComponentInTenths } from "./hsb-component-in-tenths";

/** A color in tenths, as the picker holds it. */
const TENTHS_COLOR = fc.record({
  hue: fc.integer({ max: 3599, min: 0 }),
  level: fc.integer({ max: 1000, min: 0 }),
  saturation: fc.integer({ max: 1000, min: 0 }),
});

describe("hsbComponentInTenths (DERIV-color-spaces step 11)", () => {
  it.each([
    // HSB 30.1°, 90 %, 100 %, blue: K = 901, p = 600, N = 1000 · (600 000 − 900 · 600) = 6 · 10⁷.
    { color: { hue: 301, level: 1000, saturation: 900 }, n: 1, numerator: 60_000_000 },
    // Green: K = 2101, p = min(2101, 299) = 299, N = 1000 · (600 000 − 900 · 299) = 330 900 000.
    { color: { hue: 301, level: 1000, saturation: 900 }, n: 3, numerator: 330_900_000 },
    // Red: K = 3301, min(3301, −901) clamped to 0, N = 1000 · 600 000.
    { color: { hue: 301, level: 1000, saturation: 900 }, n: 5, numerator: 600_000_000 },
    // HSB 200°, 50 %, 100 %, green: K = 3800 mod 3600 = 200, N = 1000 · (600 000 − 500 · 200).
    { color: { hue: 2000, level: 1000, saturation: 500 }, n: 3, numerator: 500_000_000 },
  ])("[F16.AC3] gives $numerator / 6 · 10⁸ for n = $n", ({ color, n, numerator }) => {
    expect(hsbComponentInTenths(color, n)).toEqual({ denominator: 600_000_000, numerator });
  });

  test.prop({ color: TENTHS_COLOR, n: fc.constantFrom(1, 3, 5) })(
    "is the component of hsbComponent, exactly",
    ({ color, n }) => {
      const { denominator, numerator } = hsbComponentInTenths(color, n);
      const hsb = {
        brightness: color.level / 10,
        hue: color.hue / 10,
        saturation: color.saturation / 10,
      };

      expect(Number.isSafeInteger(255 * numerator)).toBe(true);
      expect(numerator / denominator).toBeCloseTo(hsbComponent(hsb, n), 12);
    },
  );
});
