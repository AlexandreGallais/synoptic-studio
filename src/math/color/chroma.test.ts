import { test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ANY_RGB, rgbOfBytes } from "../../../tests";

import { chroma } from "./chroma";

describe("chroma (REF-WIKI-HSL-HSV, DERIV-color-spaces step 3)", () => {
  it.each([
    // #ff8000: 1 − 0.
    { expected: 1, rgb: rgbOfBytes(255, 128, 0) },
    // #336699: 0.6 − 0.2.
    { expected: 0.4, rgb: rgbOfBytes(51, 102, 153) },
    // #808080, a grey: 128/255 − 128/255.
    { expected: 0, rgb: rgbOfBytes(128, 128, 128) },
    { expected: 0, rgb: rgbOfBytes(0, 0, 0) },
    { expected: 0, rgb: rgbOfBytes(255, 255, 255) },
  ])("[F16.AC3] is $expected", ({ expected, rgb }) => {
    expect(chroma(rgb)).toBeCloseTo(expected, 12);
  });

  test.prop({ rgb: ANY_RGB })("is from 0 to 1, and 0 only for a grey", ({ rgb }) => {
    const chromaOfColor = chroma(rgb);

    expect(chromaOfColor).toBeGreaterThanOrEqual(0);
    expect(chromaOfColor).toBeLessThanOrEqual(1);
    expect(chromaOfColor === 0).toBe(rgb.red === rgb.green && rgb.green === rgb.blue);
  });

  test.prop({ rgb: ANY_RGB })("is the same whatever the order of the components", ({ rgb }) => {
    const turned = { blue: rgb.red, green: rgb.blue, red: rgb.green };

    expect(chroma(turned)).toBe(chroma(rgb));
  });
});
