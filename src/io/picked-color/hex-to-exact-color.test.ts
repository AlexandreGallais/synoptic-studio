import { describe, expect, it } from "vitest";

import { hexToExactColor } from "./hex-to-exact-color";

describe("hexToExactColor (DERIV-color-spaces step 12, ADR-0058)", () => {
  it("[F16.AC2] reads #336699 in HSB as 210°, 102 / 153, 153 / 255, the hue held not used", () => {
    // C = 102 > 0: its own hue, 60 · 357 / 102 = 210°.
    expect(hexToExactColor("#336699", "hsb", 200)).toEqual({
      hue: { denominator: 102, numerator: 21_420 },
      level: { denominator: 255, numerator: 153 },
      saturation: { denominator: 153, numerator: 102 },
    });
  });

  it("[F16.AC2] reads #336699 in HSL as 210°, 102 / 204, 204 / 510", () => {
    expect(hexToExactColor("#336699", "hsl", 0)).toEqual({
      hue: { denominator: 102, numerator: 21_420 },
      level: { denominator: 510, numerator: 204 },
      saturation: { denominator: 204, numerator: 102 },
    });
  });

  it("[F16.AC2] keeps the hue held for a grey: #808080 at 200°", () => {
    // C = 0: no hue of its own; S_V = 0 / 128, V = 128 / 255; S_L = 0 / min(256, 254), L = 256 / 510.
    expect(hexToExactColor("#808080", "hsb", 200)).toEqual({
      hue: { denominator: 1, numerator: 200 },
      level: { denominator: 255, numerator: 128 },
      saturation: { denominator: 128, numerator: 0 },
    });

    expect(hexToExactColor("#808080", "hsl", 200)).toEqual({
      hue: { denominator: 1, numerator: 200 },
      level: { denominator: 510, numerator: 256 },
      saturation: { denominator: 254, numerator: 0 },
    });
  });
});
