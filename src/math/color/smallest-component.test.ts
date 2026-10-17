import { test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ANY_RGB, rgbOfBytes } from "../../../tests";

import { smallestComponent } from "./smallest-component";

describe("smallestComponent (REF-WIKI-HSL-HSV, DERIV-color-spaces notation)", () => {
  it.each([
    // #ff8000: min(1, 0.50196, 0).
    { expected: 0, rgb: rgbOfBytes(255, 128, 0) },
    // #336699: min(0.2, 0.4, 0.6).
    { expected: 0.2, rgb: rgbOfBytes(51, 102, 153) },
    // #ffffff: 1.
    { expected: 1, rgb: rgbOfBytes(255, 255, 255) },
  ])("[F16.AC3] is $expected", ({ expected, rgb }) => {
    expect(smallestComponent(rgb)).toBeCloseTo(expected, 12);
  });

  test.prop({ rgb: ANY_RGB })("is one of the components, none below it", ({ rgb }) => {
    const components = [rgb.red, rgb.green, rgb.blue];

    expect(components).toContain(smallestComponent(rgb));
    expect(components.every((component) => component >= smallestComponent(rgb))).toBe(true);
  });
});
