import { describe, expect, it } from "vitest";

import { hsbComponent } from "./hsb-component";

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

  it("[F16.AC3] gives 0 for a brightness of 0, black", () => {
    expect(hsbComponent({ brightness: 0, hue: 90, saturation: 100 }, 3)).toBe(0);
  });
});
