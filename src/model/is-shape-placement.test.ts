import { describe, expect, it } from "vitest";

import { isShapePlacement } from "./is-shape-placement";

describe("isShapePlacement (ADR-0035)", () => {
  it.each([
    { angle: 0, isValid: true, x: 0, y: 0 },
    { angle: 359, isValid: true, x: -12.5, y: 1e6 },
    { angle: 360, isValid: false, x: 0, y: 0 },
    { angle: 0, isValid: false, x: NaN, y: 0 },
    { angle: 0, isValid: false, x: 0, y: -Infinity },
  ])("[F04.AC3] checks $angle° at ($x, $y)", ({ isValid, ...placement }) => {
    expect(isShapePlacement(placement)).toBe(isValid);
  });
});
