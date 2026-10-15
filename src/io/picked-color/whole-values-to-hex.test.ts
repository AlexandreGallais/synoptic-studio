import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ANY_BYTE_RGB, changedColorCount } from "../../../tests";
import { hsbToRgb, hslToRgb } from "../../math";
import { bytesToHex, hexToBytes, hexToRgb, rgbToHex } from "../color";

import { hexToExactColor } from "./hex-to-exact-color";
import { pickedColorToHex } from "./picked-color-to-hex";
import { toWholeColor } from "./to-whole-color";
import { wholeValuesToHex } from "./whole-values-to-hex";

import type { ExactColor, IntegerRatio, Rgb } from "../../math";
import type { ColorModel, PickedColor } from "../../model";

/**
 * A color given its shown values again, in a model: the round trip of a field left as it is.
 *
 * @param rgb - the color's components
 * @param model - the model the picker shows
 * @returns the components of the color rebuilt
 */
function unchanged(rgb: Rgb, model: ColorModel): Rgb {
  const exact = hexToExactColor(rgbToHex(rgb), model, 0);

  return hexToRgb(wholeValuesToHex(exact, model, toWholeColor(exact)));
}

/**
 * An exact value as a number in its unit.
 *
 * @param ratio - the value as a fraction
 * @param scale - 1 for degrees, 100 for percent
 * @returns the value in degrees or in percent
 */
function valueOf(ratio: IntegerRatio, scale: number): number {
  return (scale * ratio.numerator) / ratio.denominator;
}

/**
 * The components of exact values with one of them replaced by an integer, in binary64 through
 * the library's other path (`hsbToRgb`, `hslToRgb`): the reference the bytes are checked against.
 *
 * @param exact - the color's exact values
 * @param model - HSB or HSL
 * @param given - the integers given, one of them moved
 * @returns the components, from 0 to 1, unrounded
 */
function referenceRgb(exact: ExactColor, model: ColorModel, given: PickedColor): Rgb {
  const shown = toWholeColor(exact);
  const hue = given.hue === shown.hue ? valueOf(exact.hue, 1) : given.hue;
  const saturation =
    given.saturation === shown.saturation ? valueOf(exact.saturation, 100) : given.saturation;
  const level = given.level === shown.level ? valueOf(exact.level, 100) : given.level;

  return model === "hsb"
    ? hsbToRgb({ brightness: level, hue, saturation })
    : hslToRgb({ hue, lightness: level, saturation });
}

describe("wholeValuesToHex (DERIV-color-spaces step 12, ADR-0058)", () => {
  it("[F16.AC2] gives #34689c for #336699 with its brightness set to 61, not #33679c", () => {
    // Shown 210, 67, 60; 61 typed: 210°, 102 / 153 = 66.66667 %, 61 % (exactColorToHex's case):
    // red 51.85 → 52, green 103.7 → 104, blue 155.55 → 156.
    const exact = hexToExactColor("#336699", "hsb", 0);

    expect(wholeValuesToHex(exact, "hsb", { hue: 210, level: 61, saturation: 67 })).toBe("#34689c");
    // The three integers shown: red 0.61 · 0.33 = 0.2013, 51.33 → 51; green 0.61 · (1 − 0.335)
    // = 0.40565, 103.44 → 103; blue 156.
    expect(pickedColorToHex({ hue: 210, level: 61, saturation: 67 }, "hsb")).toBe("#33679c");
  });

  it("[F16.AC2] keeps #336699 when its saturation is set to 67, the value shown", () => {
    // 67 = the shown value: the exact 102 / 153 is used, and every value is exact.
    const exact = hexToExactColor("#336699", "hsb", 0);

    expect(wholeValuesToHex(exact, "hsb", { hue: 210, level: 60, saturation: 67 })).toBe("#336699");
  });

  it("[F16.AC2] gives #ff8000 for HSL 30°, 100 %, 50 % set from black, the green 127.5 up", () => {
    // Black shows 0, 0, 0: every value differs, so 30 / 1, 100 / 100, 50 / 100 (exactColorToHex).
    const exact = hexToExactColor("#000000", "hsl", 0);

    expect(wholeValuesToHex(exact, "hsl", { hue: 30, level: 50, saturation: 100 })).toBe("#ff8000");
  });

  it("[F16.AC2] keeps #ff0001 given the hue 360, the 0 it shows", () => {
    // #ff0001 at 359.76471° shows 0; 360 mod 360 = 0, so the exact hue is kept.
    const exact = hexToExactColor("#ff0001", "hsb", 0);

    expect(wholeValuesToHex(exact, "hsb", { hue: 360, level: 100, saturation: 100 })).toBe(
      "#ff0001",
    );
  });

  it("[F16.AC2] gives #406b80 for #808080 at the hue 200° held, its saturation set to 50", () => {
    // 200°, 50 %, V = 128 / 255: red 128 · (1 − 0.5) = 64 (40); green k = 1/3,
    // 128 · (1 − 0.5 / 3) = 106.67 → 107 (6b); blue 128 (80).
    const exact = hexToExactColor("#808080", "hsb", 200);

    expect(wholeValuesToHex(exact, "hsb", { hue: 200, level: 50, saturation: 50 })).toBe("#406b80");
  });

  it.each(["hsb", "hsl"] as const)(
    "[F16.AC2] gives one color in 31 and every sector edge back, its %s values left as shown",
    { timeout: 30_000 },
    (model) => {
      expect(changedColorCount((rgb) => unchanged(rgb, model))).toBe(0);
    },
  );

  test.prop({
    bytes: ANY_BYTE_RGB,
    key: fc.constantFrom("hue" as const, "level" as const, "saturation" as const),
    model: fc.constantFrom("hsb" as const, "hsl" as const),
  })(
    "[F16.AC2] gives, for one value moved by 1, the bytes of that integer and the others exact",
    ({ bytes, key, model }) => {
      const exact = hexToExactColor(bytesToHex([bytes.red, bytes.green, bytes.blue]), model, 0);
      const shown = toWholeColor(exact);
      const moved = key !== "hue" && shown[key] === 100 ? 99 : shown[key] + 1;
      const given = { ...shown, [key]: moved };
      const rebuilt = hexToBytes(wholeValuesToHex(exact, model, given));
      const reference = referenceRgb(exact, model, given);

      expect(Math.abs(rebuilt.red - 255 * reference.red)).toBeLessThanOrEqual(0.5 + 1e-9);
      expect(Math.abs(rebuilt.green - 255 * reference.green)).toBeLessThanOrEqual(0.5 + 1e-9);
      expect(Math.abs(rebuilt.blue - 255 * reference.blue)).toBeLessThanOrEqual(0.5 + 1e-9);
    },
  );
});
