import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { hexToPickedColor } from "./hex-to-picked-color";
import { pickedColorToHex } from "./picked-color-to-hex";
import { rgbToHex } from "./rgb-to-hex";

describe("pickedColorToHex (DERIV-color-spaces steps 6 and 8)", () => {
  it.each([
    // HSB 210°, 100 %, 50 %: red f(5) = 0.5 − 0.5 = 0; green k = 0.5, 0.5 − 0.25 = 0.25, 63.75
    // gives 64 (40); blue k = 4.5, 0.5, 127.5 gives 128 (80). The step 10 example.
    { expected: "#004080", model: "hsb", picked: { hue: 210, level: 50, saturation: 100 } },
    // HSB 210°, 66.7 %, 60 %: red 0.6 − 0.6 · 0.667 = 0.1998, 50.949 gives 51 (33).
    { expected: "#336699", model: "hsb", picked: { hue: 210, level: 60, saturation: 66.7 } },
    // HSL 30.1°, 100 %, 50 %: green 0.50167, 127.925 gives 128 (80).
    { expected: "#ff8000", model: "hsl", picked: { hue: 30.1, level: 50, saturation: 100 } },
    // HSB 30.1°, 100 %, 100 %: the same color.
    { expected: "#ff8000", model: "hsb", picked: { hue: 30.1, level: 100, saturation: 100 } },
    // A grey keeps no hue: HSB 200°, 0 %, 50.2 %, 0.502 · 255 = 128.01.
    { expected: "#808080", model: "hsb", picked: { hue: 200, level: 50.2, saturation: 0 } },
  ] as const)("[F16.AC2] gives $expected for $picked in $model", ({ expected, model, picked }) => {
    expect(pickedColorToHex(picked, model)).toBe(expected);
  });

  test.prop({
    blue: fc.integer({ max: 255, min: 0 }),
    green: fc.integer({ max: 255, min: 0 }),
    model: fc.constantFrom("hsb" as const, "hsl" as const),
    red: fc.integer({ max: 255, min: 0 }),
  })("gives back every color shown to one decimal", ({ blue, green, model, red }) => {
    const color = rgbToHex({ blue: blue / 255, green: green / 255, red: red / 255 });

    expect(pickedColorToHex(hexToPickedColor(color, model, 0), model)).toBe(color);
  });
});
