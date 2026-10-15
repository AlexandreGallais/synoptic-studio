import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { hslComponent } from "./hsl-component";

/** Any HSL a picker may give: a hue from 0 to 360, a saturation and lightness from 0 to 100. */
const ANY_HSL = fc.record({
  hue: fc.double({ max: 360, min: 0, noNaN: true }),
  lightness: fc.double({ max: 100, min: 0, noNaN: true }),
  saturation: fc.double({ max: 100, min: 0, noNaN: true }),
});

describe("hslComponent (REF-CSS-COLOR-4 §7.1, DERIV-color-spaces step 6)", () => {
  /** HSL(30°, 100 %, 50 %): a = 1 · min(0.5, 0.5) = 0.5. */
  const orange = { hue: 30, lightness: 50, saturation: 100 };

  it.each([
    // n = 0: k = (0 + 1) mod 12 = 1, max(−1, min(−2, 8, 1)) = −1, 0.5 + 0.5 = 1.
    { expected: 1, n: 0 },
    // n = 8: k = 9, min(6, 0, 1) = 0, 0.5 − 0 = 0.5.
    { expected: 0.5, n: 8 },
    // n = 4: k = 5, min(2, 4, 1) = 1, 0.5 − 0.5 = 0.
    { expected: 0, n: 4 },
  ])("[F16.AC3] gives $expected for n = $n of HSL(30°, 100 %, 50 %)", ({ expected, n }) => {
    expect(hslComponent(orange, n)).toBeCloseTo(expected, 12);
  });

  it("[F16.AC3] gives 0.50167 for the green of HSL(30.1°, 100 %, 50 %)", () => {
    // k = (8 + 30.1 / 30) mod 12 = 9.00333, 9 − k = −0.00333, 0.5 + 0.5 · 0.00333 = 0.50167.
    expect(hslComponent({ ...orange, hue: 30.1 }, 8)).toBeCloseTo(0.501667, 6);
  });

  it("[F16.AC3] gives the lightness for a saturation of 0, a grey", () => {
    // a = 0: f(n) = L for every n.
    expect(hslComponent({ hue: 200, lightness: 40, saturation: 0 }, 8)).toBeCloseTo(0.4, 12);
  });

  it("[F16.AC3] takes a hue of −240° as 120°, green: its red is 0", () => {
    // k = (0 − 240 / 30) mod 12 = 4, min(1, 5, 1) = 1: 0.5 − 0.5 · 1 = 0, as at 120°.
    const green = { ...orange, hue: -240 };

    expect(hslComponent(green, 0)).toBe(0);
    expect(hslComponent(green, 8)).toBe(hslComponent({ ...orange, hue: 120 }, 8));
  });

  it("[F16.AC3] takes a hue of 360° as 0°", () => {
    // k = (0 + 12) mod 12 = 0: the same as hue 0, red 1 for HSL(360°, 100 %, 50 %).
    expect(hslComponent({ ...orange, hue: 360 }, 0)).toBeCloseTo(1, 12);
    expect(hslComponent({ ...orange, hue: 360 }, 8)).toBeCloseTo(0, 12);
  });

  test.prop({ hsl: ANY_HSL, n: fc.constantFrom(0, 8, 4) })(
    "is a component from 0 to 1",
    ({ hsl, n }) => {
      expect(hslComponent(hsl, n)).toBeGreaterThanOrEqual(-1e-15);
      expect(hslComponent(hsl, n)).toBeLessThanOrEqual(1 + 1e-15);
    },
  );
});
