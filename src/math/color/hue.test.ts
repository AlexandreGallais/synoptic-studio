import { test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ANY_RGB, rgbOfBytes } from "../../../tests";

import { hue } from "./hue";

describe("hue (REF-WIKI-HSL-HSV, DERIV-color-spaces step 3)", () => {
  it.each([
    // #ff8000: 60 · 128/255 = 30.11765.
    { expected: 30.117647, rgb: rgbOfBytes(255, 128, 0) },
    // #336699: 60 · 3.5.
    { expected: 210, rgb: rgbOfBytes(51, 102, 153) },
    // The primaries: red 0, green 60 · 2, blue 60 · 4.
    { expected: 0, rgb: rgbOfBytes(255, 0, 0) },
    { expected: 120, rgb: rgbOfBytes(0, 255, 0) },
    { expected: 240, rgb: rgbOfBytes(0, 0, 255) },
    // #ff0001, just below a turn: 60 · (6 − 1/255) = 359.76471.
    { expected: 359.764706, rgb: rgbOfBytes(255, 0, 1) },
  ])("[F16.AC3] is $expected°", ({ expected, rgb }) => {
    expect(hue(rgb)).toBeCloseTo(expected, 5);
  });

  it.each([
    { name: "#808080", rgb: rgbOfBytes(128, 128, 128) },
    { name: "#000000", rgb: rgbOfBytes(0, 0, 0) },
    { name: "#ffffff", rgb: rgbOfBytes(255, 255, 255) },
  ])("[F16.AC3] is 0 for $name, a grey, whose hue is undefined", ({ rgb }) => {
    expect(hue(rgb)).toBe(0);
  });

  test.prop({ rgb: ANY_RGB })("is from 0° included to 360° excluded", ({ rgb }) => {
    expect(hue(rgb)).toBeGreaterThanOrEqual(0);
    expect(hue(rgb)).toBeLessThan(360);
  });
});
