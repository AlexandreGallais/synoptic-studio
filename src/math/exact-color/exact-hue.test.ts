import { test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ANY_BYTE_RGB, rgbOfBytes } from "../../../tests";
import { hue } from "../color";

import { exactHue } from "./exact-hue";

describe("exactHue (DERIV-color-spaces steps 3 and 12)", () => {
  it.each([
    // #ff8000: M = R = 255, C = 255, (128 − 0) mod 1530 = 128; 60 · 128 / 255 = 30.11765°.
    { bytes: { blue: 0, green: 128, red: 255 }, denominator: 255, numerator: 7680 },
    // #336699: M = B = 153, C = 102, (51 − 102) + 4 · 102 = 357; 60 · 357 / 102 = 210°.
    { bytes: { blue: 153, green: 102, red: 51 }, denominator: 102, numerator: 21_420 },
    // #00ff80: M = G, C = 255, (128 − 0) + 2 · 255 = 638; 60 · 638 / 255 = 150.11765°.
    { bytes: { blue: 128, green: 255, red: 0 }, denominator: 255, numerator: 38_280 },
    // #ff0001: M = R, (0 − 1) mod 1530 = 1529; 60 · 1529 / 255 = 359.76471°, the largest hue.
    { bytes: { blue: 1, green: 0, red: 255 }, denominator: 255, numerator: 91_740 },
    // #ffff00: red and green both largest, red tested first: (255 − 0) mod 1530 = 255, 60°.
    { bytes: { blue: 0, green: 255, red: 255 }, denominator: 255, numerator: 15_300 },
    // #808080, a grey: C = 0, the hue 0.
    { bytes: { blue: 128, green: 128, red: 128 }, denominator: 1, numerator: 0 },
  ])("[F16.AC2] gives $numerator / $denominator degrees", ({ bytes, denominator, numerator }) => {
    expect(exactHue(bytes)).toEqual({ denominator, numerator });
  });

  test.prop({ bytes: ANY_BYTE_RGB })("is hue's value, in integers", ({ bytes }) => {
    const { denominator, numerator } = exactHue(bytes);

    expect(Number.isSafeInteger(numerator) && Number.isSafeInteger(denominator)).toBe(true);

    expect(numerator / denominator).toBeCloseTo(
      hue(rgbOfBytes(bytes.red, bytes.green, bytes.blue)),
      10,
    );
  });
});
