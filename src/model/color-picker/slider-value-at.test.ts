import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { sliderPositionOf } from "./slider-position-of";
import { sliderValueAt } from "./slider-value-at";

describe("sliderValueAt (DERIV-color-spaces step 10)", () => {
  it.each([
    // 360 · 100 / 200 = 180.
    { expected: 180, length: 200, maximum: 360, position: 100 },
    // 360 · 0 / 200 = 0, 360 · 200 / 200 = 360: the right end is a whole turn, shown 0.0.
    { expected: 0, length: 200, maximum: 360, position: 0 },
    { expected: 360, length: 200, maximum: 360, position: 200 },
    // 100 · 50 / 200 = 25.
    { expected: 25, length: 200, maximum: 100, position: 50 },
    // A drag beyond an end counts at that end.
    { expected: 0, length: 200, maximum: 100, position: -30 },
    { expected: 100, length: 200, maximum: 100, position: 260 },
    // A slider of length 0, not laid out yet, holds 0.
    { expected: 0, length: 0, maximum: 100, position: 0 },
  ])(
    "[F16.AC2] holds $expected at $position on a slider $length long up to $maximum",
    ({ expected, length, maximum, position }) => {
      expect(sliderValueAt(position, length, maximum)).toBe(expected);
    },
  );

  test.prop({
    length: fc.integer({ max: 2000, min: 1 }),
    maximum: fc.constantFrom(100, 360),
    ratio: fc.double({ max: 1.5, min: -0.5, noNaN: true }),
  })("stays within its range and is read back where it sits", ({ length, maximum, ratio }) => {
    const position = Math.round(length * ratio);
    const value = sliderValueAt(position, length, maximum);

    expect(value).toBeGreaterThanOrEqual(0);
    expect(value).toBeLessThanOrEqual(maximum);

    expect(sliderPositionOf(value, length, maximum)).toBeCloseTo(
      Math.min(Math.max(position, 0), length),
      9,
    );
  });
});

describe("sliderPositionOf (DERIV-color-spaces step 10)", () => {
  it("[F16.AC2] places 30.1° at 30.1 on a hue slider 360 long, 50 % at 100 of 200", () => {
    // 360 · 30.1 / 360 = 30.1; 200 · 50 / 100 = 100.
    expect(sliderPositionOf(30.1, 360, 360)).toBeCloseTo(30.1, 12);
    expect(sliderPositionOf(50, 200, 100)).toBe(100);
    expect([sliderPositionOf(0, 200, 100), sliderPositionOf(100, 200, 100)]).toEqual([0, 200]);
  });
});
