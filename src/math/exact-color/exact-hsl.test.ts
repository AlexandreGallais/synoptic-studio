import { test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ANY_BYTE_RGB, rgbOfBytes } from "../../../tests";
import { rgbToHsl } from "../color";

import { exactHsl } from "./exact-hsl";

describe("exactHsl (DERIV-color-spaces steps 4 and 12)", () => {
  it("[F16.AC2] reads #336699 as 210°, 102 / 204 and 204 / 510", () => {
    // M + m = 204, min(204, 306) = 204, C = 102: S = 50 %, L = 40 %.
    expect(exactHsl({ blue: 153, green: 102, red: 51 })).toEqual({
      hue: { denominator: 102, numerator: 21_420 },
      level: { denominator: 510, numerator: 204 },
      saturation: { denominator: 204, numerator: 102 },
    });
  });

  it("[F16.AC2] reads #c0c0ff as 240°, 63 / 63, the sum past 255", () => {
    // M + m = 447, min(447, 63) = 63, C = 63: S = 100 %; hue (192 − 192) + 4 · 63 = 252, 240°.
    expect(exactHsl({ blue: 255, green: 192, red: 192 })).toEqual({
      hue: { denominator: 63, numerator: 15_120 },
      level: { denominator: 510, numerator: 447 },
      saturation: { denominator: 63, numerator: 63 },
    });
  });

  it.each([
    // M + m = 0 or 510: min is 0, the saturation 0, not 0 / 0.
    { bytes: { blue: 0, green: 0, red: 0 }, sum: 0 },
    { bytes: { blue: 255, green: 255, red: 255 }, sum: 510 },
  ])("[F16.AC2] reads a lightness $sum / 510 as saturation 0 / 1", ({ bytes, sum }) => {
    expect(exactHsl(bytes)).toEqual({
      hue: { denominator: 1, numerator: 0 },
      level: { denominator: 510, numerator: sum },
      saturation: { denominator: 1, numerator: 0 },
    });
  });

  test.prop({ bytes: ANY_BYTE_RGB })("is rgbToHsl's values, in integers", ({ bytes }) => {
    const exact = exactHsl(bytes);
    const hsl = rgbToHsl(rgbOfBytes(bytes.red, bytes.green, bytes.blue));

    expect(
      (100 * exact.saturation.numerator) / exact.saturation.denominator - hsl.saturation,
    ).toBeCloseTo(0, 10);

    expect((100 * exact.level.numerator) / exact.level.denominator - hsl.lightness).toBeCloseTo(
      0,
      10,
    );
  });
});
