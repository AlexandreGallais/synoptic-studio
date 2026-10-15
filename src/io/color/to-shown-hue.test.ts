import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { toShownHue } from "./to-shown-hue";

describe("toShownHue (DERIV-color-spaces steps 9 and 12, ADR-0058)", () => {
  it.each([
    // round(30.11765) = 30.
    { expected: 30, hue: 30.117647 },
    // A tie: 7.5 rounds up to 8.
    { expected: 8, hue: 7.5 },
    // #ff0001, the largest hue a color has: round(359.76471) = 360, shown 0.
    { expected: 0, hue: 359.764706 },
    // round(359.4) = 359, below 360.
    { expected: 359, hue: 359.4 },
    { expected: 0, hue: 360 },
    { expected: 0, hue: 0 },
  ])("[F16.AC2] shows $hue° as $expected°", ({ expected, hue }) => {
    expect(toShownHue(hue)).toBe(expected);
  });

  test.prop({ hue: fc.double({ max: 360, min: 0, noNaN: true }) })(
    "shows an integer from 0 to 359 within a half degree of the hue, around the turn",
    ({ hue }) => {
      const shown = toShownHue(hue);

      expect(Number.isSafeInteger(shown) && shown >= 0 && shown <= 359).toBe(true);
      expect(Math.min(Math.abs(shown - hue), Math.abs(shown + 360 - hue))).toBeLessThanOrEqual(0.5);
    },
  );
});
