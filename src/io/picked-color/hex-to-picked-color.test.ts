import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ANY_BYTE_RGB } from "../../../tests";
import { bytesToHex } from "../color";

import { hexToPickedColor } from "./hex-to-picked-color";

describe("hexToPickedColor (DERIV-color-spaces steps 3 and 12, ADR-0058)", () => {
  it("[F16.AC2] shows #ff8000 at 30°, 100 %, 100 % in HSB and 30°, 100 %, 50 % in HSL", () => {
    // Hue 60 · 128 / 255 = 30.11765, shown 30; V = 1, S_V = 1; L = 0.5, S_L = 1.
    expect(hexToPickedColor("#ff8000", "hsb", 0)).toEqual({
      hue: 30,
      level: 100,
      saturation: 100,
    });

    expect(hexToPickedColor("#ff8000", "hsl", 0)).toEqual({
      hue: 30,
      level: 50,
      saturation: 100,
    });
  });

  it("[F16.AC2] shows #336699 at 210°, 67 %, 60 % in HSB, the hue held not used", () => {
    // V = 0.6, S_V = 102 / 153 = 66.66667 %, shown 67.
    expect(hexToPickedColor("#336699", "hsb", 200)).toEqual({
      hue: 210,
      level: 60,
      saturation: 67,
    });
  });

  it("[F16.AC2] keeps the hue held for a grey: #808080 at 200°, 0 %, 50 %", () => {
    // C = 0; V = 128 / 255 = 50.196 %, L = 256 / 510 = 50.196 %, shown 50.
    expect(hexToPickedColor("#808080", "hsb", 200)).toEqual({
      hue: 200,
      level: 50,
      saturation: 0,
    });

    expect(hexToPickedColor("#808080", "hsl", 200)).toEqual({
      hue: 200,
      level: 50,
      saturation: 0,
    });
  });

  test.prop({
    bytes: ANY_BYTE_RGB,
    keptHue: fc.integer({ max: 359, min: 0 }),
    model: fc.constantFrom("hsb" as const, "hsl" as const),
  })(
    "shows whole values, the hue from 0 to 359, the others from 0 to 100, a grey's hue kept",
    ({ bytes, keptHue, model }) => {
      const picked = hexToPickedColor(
        bytesToHex([bytes.red, bytes.green, bytes.blue]),
        model,
        keptHue,
      );
      const isGrey = bytes.red === bytes.green && bytes.green === bytes.blue;

      expect(
        [picked.hue, picked.saturation, picked.level].every((value) => Number.isSafeInteger(value)),
      ).toBe(true);

      expect(picked.hue).toBeGreaterThanOrEqual(0);
      expect(picked.hue).toBeLessThanOrEqual(359);
      expect(Math.min(picked.saturation, picked.level)).toBeGreaterThanOrEqual(0);
      expect(Math.max(picked.saturation, picked.level)).toBeLessThanOrEqual(100);
      expect(isGrey ? picked.hue : keptHue).toBe(keptHue);
    },
  );
});
