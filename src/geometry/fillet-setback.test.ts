import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { filletSetback } from "./fillet-setback";

/** Decimals compared: one tangent of a double, correct to its last bits. */
const DECIMALS = 12;

describe("filletSetback (DERIV-fillet-setback check table)", () => {
  it("equals the radius at a right corner", () => {
    // 10 × tan(π/4) = 10.
    expect(filletSetback(10, Math.PI / 2)).toBeCloseTo(10, DECIMALS);
  });

  it("is 0 for aligned vertices: no arc", () => {
    // 30 × tan(0) = 0.
    expect(filletSetback(30, 0)).toBe(0);
  });

  it("is r / √3 for a turn of π/3", () => {
    // 10 × tan(π/6) = 10 / √3 ≈ 5.773503.
    expect(filletSetback(10, Math.PI / 3)).toBeCloseTo(10 / Math.sqrt(3), DECIMALS);
  });

  it("is 0 for a sharp corner (radius 0)", () => {
    expect(filletSetback(0, Math.PI / 2)).toBe(0);
  });

  it("only depends on the size of the turn, not its direction", () => {
    // A counter-clockwise quarter turn gives the same setback as a clockwise one.
    expect(filletSetback(10, -Math.PI / 2)).toBeCloseTo(10, DECIMALS);
  });
});

describe("filletSetback (properties)", () => {
  test.prop({
    angle: fc.double({ max: Math.PI - 0.01, min: -(Math.PI - 0.01), noNaN: true }),
    radius: fc.integer({ max: 1000, min: 0 }),
  })("is >= 0, even in the turn and proportional to the radius", ({ angle, radius }) => {
    expect(filletSetback(radius, angle)).toBeGreaterThanOrEqual(0);
    expect(filletSetback(radius, -angle)).toBe(filletSetback(radius, angle));
    expect(filletSetback(2 * radius, angle)).toBeCloseTo(2 * filletSetback(radius, angle), 6);
  });
});
