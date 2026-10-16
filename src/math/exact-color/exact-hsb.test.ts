import { test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ANY_BYTE_RGB, rgbOfBytes } from "../../../tests";
import { rgbToHsb } from "../color";

import { exactHsb } from "./exact-hsb";

describe("exactHsb (DERIV-color-spaces steps 5 and 12)", () => {
  it("[F16.AC2] reads #336699 as 210°, 102 / 153 and 153 / 255", () => {
    // M = 153, m = 51, C = 102: S = 102 / 153 = 66.66667 %, V = 153 / 255 = 60 %.
    expect(exactHsb({ blue: 153, green: 102, red: 51 })).toEqual({
      hue: { denominator: 102, numerator: 21_420 },
      level: { denominator: 255, numerator: 153 },
      saturation: { denominator: 153, numerator: 102 },
    });
  });

  it("[F16.AC2] reads black as saturation 0 / 1 and brightness 0 / 255", () => {
    // M = 0: S_V is 0, not 0 / 0.
    expect(exactHsb({ blue: 0, green: 0, red: 0 })).toEqual({
      hue: { denominator: 1, numerator: 0 },
      level: { denominator: 255, numerator: 0 },
      saturation: { denominator: 1, numerator: 0 },
    });
  });

  test.prop({ bytes: ANY_BYTE_RGB })("is rgbToHsb's values, in integers", ({ bytes }) => {
    const exact = exactHsb(bytes);
    const hsb = rgbToHsb(rgbOfBytes(bytes.red, bytes.green, bytes.blue));

    expect(
      (100 * exact.saturation.numerator) / exact.saturation.denominator - hsb.saturation,
    ).toBeCloseTo(0, 10);

    expect((100 * exact.level.numerator) / exact.level.denominator - hsb.brightness).toBeCloseTo(
      0,
      10,
    );
  });
});
