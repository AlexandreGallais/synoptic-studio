import { describe, expect, it } from "vitest";

import { hexToPickedColor } from "./hex-to-picked-color";

describe("hexToPickedColor (DERIV-color-spaces steps 3 and 9)", () => {
  it("[F16.AC2] shows #ff8000 at 30.1°, 100 %, 100 % in HSB and 30.1°, 100 %, 50 % in HSL", () => {
    // Hue 60 · 128 / 255 = 30.11765, shown 30.1; V = 1, S_V = 1; L = 0.5, S_L = 1.
    expect(hexToPickedColor("#ff8000", "hsb", 0)).toEqual({
      hue: 30.1,
      level: 100,
      saturation: 100,
    });

    expect(hexToPickedColor("#ff8000", "hsl", 0)).toEqual({
      hue: 30.1,
      level: 50,
      saturation: 100,
    });
  });

  it("[F16.AC2] shows #336699 at 210°, 66.7 %, 60 % in HSB, the hue held not used", () => {
    // V = 0.6, S_V = 0.4 / 0.6 = 66.66667 %, shown 66.7.
    expect(hexToPickedColor("#336699", "hsb", 200)).toEqual({
      hue: 210,
      level: 60,
      saturation: 66.7,
    });
  });

  it("[F16.AC2] keeps the hue held for a grey: #808080 at 200°, 0 %, 50.2 %", () => {
    // C = 0; V = L = 128 / 255 = 0.50196, shown 50.2.
    expect(hexToPickedColor("#808080", "hsb", 200)).toEqual({
      hue: 200,
      level: 50.2,
      saturation: 0,
    });

    expect(hexToPickedColor("#808080", "hsl", 0)).toEqual({ hue: 0, level: 50.2, saturation: 0 });
  });
});
