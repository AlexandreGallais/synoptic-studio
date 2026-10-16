import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ANY_BYTE_RGB } from "../../../tests";
import { bytesToHex } from "../color";

import { hexToExactColor } from "./hex-to-exact-color";
import { toWholeColor } from "./to-whole-color";

import type { IntegerRatio } from "../../math";

/**
 * A fraction of integers.
 *
 * @param numerator - its numerator
 * @param denominator - its denominator, > 0
 * @returns the fraction
 */
function ratio(numerator: number, denominator: number): IntegerRatio {
  return { denominator, numerator };
}

describe("toWholeColor (DERIV-color-spaces steps 9 and 12, ADR-0058)", () => {
  it("[F16.AC2] shows #ff8000 as HSB 30°, 100 %, 100 % and HSL 30°, 100 %, 50 %", () => {
    // Hue 7680 / 255 = 30.11765, 30; S_V = 255 / 255; V = 255 / 255; S_L = 255 / 255; L = 255 / 510.
    expect(toWholeColor(hexToExactColor("#ff8000", "hsb", 0))).toEqual({
      hue: 30,
      level: 100,
      saturation: 100,
    });

    expect(toWholeColor(hexToExactColor("#ff8000", "hsl", 0))).toEqual({
      hue: 30,
      level: 50,
      saturation: 100,
    });
  });

  it("[F16.AC2] shows #336699 as HSB 210°, 67 %, 60 %", () => {
    // 100 · 102 / 153 = 66.66667, 67; 100 · 153 / 255 = 60.
    expect(toWholeColor(hexToExactColor("#336699", "hsb", 0))).toEqual({
      hue: 210,
      level: 60,
      saturation: 67,
    });
  });

  it("[F16.AC2] shows #ff0001, its hue 359.76471° rounding to 360, at 0°", () => {
    // 91 740 / 255 = 359.76471, round 360, 360 mod 360 = 0.
    expect(toWholeColor(hexToExactColor("#ff0001", "hsb", 0))).toHaveProperty("hue", 0);
  });

  it("[F16.AC2] rounds a tie up: #080100 at 7.5° shows 8°, #c8c7c7 at 0.5 % shows 1 %", () => {
    // #080100: C = 8, hue 60 · 1 / 8 = 7.5, up to 8; V = 100 · 8 / 255 = 3.137, 3.
    expect(toWholeColor(hexToExactColor("#080100", "hsb", 0))).toEqual({
      hue: 8,
      level: 3,
      saturation: 100,
    });

    // #c8c7c7: C = 1, S_V = 100 · 1 / 200 = 0.5, up to 1; V = 100 · 200 / 255 = 78.431, 78.
    expect(toWholeColor(hexToExactColor("#c8c7c7", "hsb", 0))).toEqual({
      hue: 0,
      level: 78,
      saturation: 1,
    });
  });

  it.each([
    // A hue of 360 is 0; 719 / 2 = 359.5, a tie, up to 360, so 0; 359.4 shows 359.
    { expected: 0, hue: ratio(360, 1) },
    { expected: 0, hue: ratio(719, 2) },
    { expected: 359, hue: ratio(1797, 5) },
  ])(
    "[F16.AC2] shows the hue $hue.numerator / $hue.denominator at $expected",
    ({ expected, hue }) => {
      expect(toWholeColor({ hue, level: ratio(0, 1), saturation: ratio(0, 1) })).toHaveProperty(
        "hue",
        expected,
      );
    },
  );

  test.prop({ bytes: ANY_BYTE_RGB, model: fc.constantFrom("hsb" as const, "hsl" as const) })(
    "shows integers within a half of the exact values, the hue from 0 to 359",
    ({ bytes, model }) => {
      const exact = hexToExactColor(bytesToHex([bytes.red, bytes.green, bytes.blue]), model, 0);
      const whole = toWholeColor(exact);
      const hue = exact.hue.numerator / exact.hue.denominator;

      expect(Number.isSafeInteger(whole.hue) && whole.hue >= 0 && whole.hue <= 359).toBe(true);

      expect(
        Math.min(Math.abs(whole.hue - hue), Math.abs(whole.hue + 360 - hue)),
      ).toBeLessThanOrEqual(0.5);

      expect(
        Math.abs(
          whole.saturation - (100 * exact.saturation.numerator) / exact.saturation.denominator,
        ),
      ).toBeLessThanOrEqual(0.5);

      expect(
        Math.abs(whole.level - (100 * exact.level.numerator) / exact.level.denominator),
      ).toBeLessThanOrEqual(0.5);
    },
  );
});
