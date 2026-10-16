import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { hsbComponent } from "./hsb-component";

/** Any HSB a picker may give: a hue from 0 to 360, a saturation and brightness from 0 to 100. */
const ANY_HSB = fc.record({
  hue: fc.double({ max: 360, min: 0, noNaN: true }),
  brightness: fc.double({ max: 100, min: 0, noNaN: true }),
  saturation: fc.double({ max: 100, min: 0, noNaN: true }),
});

describe("hsbComponent (REF-WIKI-HSL-HSV, DERIV-color-spaces step 6)", () => {
  /** HSB(210°, 67 %, 60 %): H / 60° = 3.5, V · S = 0.402. */
  const blueGrey = { brightness: 60, hue: 210, saturation: 67 };

  it.each([
    // n = 5: k = 8.5 mod 6 = 2.5, min(2.5, 1.5, 1) = 1, 0.6 − 0.402 = 0.198.
    { expected: 0.198, n: 5 },
    // n = 3: k = 6.5 mod 6 = 0.5, min(0.5, 3.5, 1) = 0.5, 0.6 − 0.201 = 0.399.
    { expected: 0.399, n: 3 },
    // n = 1: k = 4.5, min(4.5, −0.5, 1) = −0.5, max(0, −0.5) = 0, 0.6.
    { expected: 0.6, n: 1 },
  ])("[F16.AC3] gives $expected for n = $n of HSB(210°, 67 %, 60 %)", ({ expected, n }) => {
    expect(hsbComponent(blueGrey, n)).toBeCloseTo(expected, 12);
  });

  it("[F16.AC3] gives the brightness for a saturation of 0, a grey", () => {
    // V · S = 0: f(n) = V for every n.
    expect(hsbComponent({ brightness: 25, hue: 90, saturation: 0 }, 5)).toBeCloseTo(0.25, 12);
  });

  it("[F16.AC3] takes a hue of −240° as 120°, green: its red is 0", () => {
    // k = (5 − 240 / 60) mod 6 = 1, min(1, 3, 1) = 1: 1 − 1 · 1 · 1 = 0, as at 120°.
    const green = { brightness: 100, hue: -240, saturation: 100 };

    expect(hsbComponent(green, 5)).toBe(0);
    expect(hsbComponent(green, 3)).toBe(hsbComponent({ ...green, hue: 120 }, 3));
  });

  it("[F16.AC3] gives 0 for a brightness of 0, black", () => {
    expect(hsbComponent({ brightness: 0, hue: 90, saturation: 100 }, 3)).toBe(0);
  });

  test.prop({ hsb: ANY_HSB, n: fc.constantFrom(5, 3, 1) })(
    "is a component from 0 to 1",
    ({ hsb, n }) => {
      expect(hsbComponent(hsb, n)).toBeGreaterThanOrEqual(-1e-15);
      expect(hsbComponent(hsb, n)).toBeLessThanOrEqual(1 + 1e-15);
    },
  );
});
