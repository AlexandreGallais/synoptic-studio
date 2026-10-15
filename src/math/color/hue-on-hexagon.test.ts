import { test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ANY_RGB, rgbOfBytes } from "../../../tests";

import { chroma } from "./chroma";
import { hueOnHexagon } from "./hue-on-hexagon";

describe("hueOnHexagon (REF-WIKI-HSL-HSV, DERIV-color-spaces step 3)", () => {
  it.each([
    // #ff8000, red largest: (128/255 − 0) / 1 mod 6 = 0.50196.
    { expected: 128 / 255, rgb: rgbOfBytes(255, 128, 0) },
    // #ff0080, red largest, green below blue: (0 − 128/255) / 1 mod 6 = 6 − 0.50196.
    { expected: 6 - 128 / 255, rgb: rgbOfBytes(255, 0, 128) },
    // #00ff80, green largest: (128/255 − 0) / 1 + 2.
    { expected: 2 + 128 / 255, rgb: rgbOfBytes(0, 255, 128) },
    // #336699, blue largest: (0.2 − 0.4) / 0.4 + 4 = 3.5.
    { expected: 3.5, rgb: rgbOfBytes(51, 102, 153) },
    // #ffff00, red and green largest, red tested first: (1 − 0) / 1 mod 6 = 1, yellow.
    { expected: 1, rgb: rgbOfBytes(255, 255, 0) },
    // #00ffff, green and blue largest, green tested first: (1 − 0) / 1 + 2 = 3, cyan.
    { expected: 3, rgb: rgbOfBytes(0, 255, 255) },
    // #ff00ff, red and blue largest, red tested first: (0 − 1) / 1 mod 6 = 5, magenta.
    { expected: 5, rgb: rgbOfBytes(255, 0, 255) },
  ])("[F16.AC3] is $expected sixths of a turn", ({ expected, rgb }) => {
    expect(hueOnHexagon(rgb, chroma(rgb))).toBeCloseTo(expected, 12);
  });

  test.prop({ rgb: ANY_RGB.filter((rgb) => chroma(rgb) > 0) })(
    "is from 0 included to 6 excluded",
    ({ rgb }) => {
      const sixths = hueOnHexagon(rgb, chroma(rgb));

      expect(sixths).toBeGreaterThanOrEqual(0);
      expect(sixths).toBeLessThan(6);
    },
  );
});
