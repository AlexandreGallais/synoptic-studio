import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { hslComponent } from "../color";

import { hslComponentInTenths } from "./hsl-component-in-tenths";

/** A color in tenths, as the picker holds it. */
const TENTHS_COLOR = fc.record({
  hue: fc.integer({ max: 3599, min: 0 }),
  level: fc.integer({ max: 1000, min: 0 }),
  saturation: fc.integer({ max: 1000, min: 0 }),
});

describe("hslComponentInTenths (DERIV-color-spaces step 11)", () => {
  it.each([
    // HSL 0°, 75 %, 40 %, red: K = 0, q = −300, a = 750 · 400, N = 1.2 · 10⁸ + 300 000 · 300.
    { color: { hue: 0, level: 400, saturation: 750 }, n: 0, numerator: 210_000_000 },
    // Green: K = 2400, q = min(1500, 300) = 300, N = 1.2 · 10⁸ − 9 · 10⁷.
    { color: { hue: 0, level: 400, saturation: 750 }, n: 8, numerator: 30_000_000 },
    // HSL 0°, 75 %, 60 %, blue: K = 1200, q = 300, a = 750 · 400, N = 1.8 · 10⁸ − 9 · 10⁷.
    { color: { hue: 0, level: 600, saturation: 750 }, n: 4, numerator: 90_000_000 },
  ])("[F16.AC3] gives $numerator / 3 · 10⁸ for n = $n", ({ color, n, numerator }) => {
    expect(hslComponentInTenths(color, n)).toEqual({ denominator: 300_000_000, numerator });
  });

  test.prop({ color: TENTHS_COLOR, n: fc.constantFrom(0, 4, 8) })(
    "is the component of hslComponent, exactly",
    ({ color, n }) => {
      const { denominator, numerator } = hslComponentInTenths(color, n);
      const hsl = {
        hue: color.hue / 10,
        lightness: color.level / 10,
        saturation: color.saturation / 10,
      };

      expect(Number.isSafeInteger(255 * numerator)).toBe(true);
      expect(numerator / denominator).toBeCloseTo(hslComponent(hsl, n), 12);
    },
  );
});
