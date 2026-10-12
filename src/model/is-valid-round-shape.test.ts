import { describe, expect, it } from "vitest";

import { isValidRoundShape } from "./is-valid-round-shape";

describe("isValidRoundShape (ADR-0036)", () => {
  it.each([
    { angle: 0, height: 80, isValid: true, width: 120, x: 0, y: 0 },
    { angle: 359, height: 0, isValid: true, width: 50, x: -3.5, y: 2 },
    { angle: 0, height: 80, isValid: false, width: -1, x: 0, y: 0 },
    { angle: 0, height: 80.5, isValid: false, width: 120, x: 0, y: 0 },
    { angle: 360, height: 80, isValid: false, width: 120, x: 0, y: 0 },
    { angle: 0, height: 80, isValid: false, width: 120, x: NaN, y: 0 },
    { angle: 0, height: 80, isValid: false, width: 120, x: 0, y: Infinity },
    { angle: 0, height: 80, isValid: false, width: 120.5, x: 0, y: 0 },
  ])("[F10.AC1] checks $width × $height at $angle° and ($x, $y)", ({ isValid, ...values }) => {
    expect(isValidRoundShape({ ...values, kind: "round" })).toBe(isValid);
  });
});
