import { describe, expect, it } from "vitest";

import { toShownHue } from "./to-shown-hue";

describe("toShownHue (DERIV-color-spaces step 9)", () => {
  it.each([
    // round(301.1765) / 10.
    { expected: 30.1, hue: 30.117647 },
    // #ff0001, the largest hue a color has: round(3597.6471) / 10.
    { expected: 359.8, hue: 359.764706 },
    // round(3599.4) / 10 = 359.9, below 360.0.
    { expected: 359.9, hue: 359.94 },
    // round(3599.6) / 10 = 360.0, the same direction as 0.0.
    { expected: 0, hue: 359.96 },
    { expected: 0, hue: 360 },
    { expected: 0, hue: 0 },
  ])("[F16.AC3] shows $hue° as $expected°", ({ expected, hue }) => {
    expect(toShownHue(hue)).toBe(expected);
  });
});
