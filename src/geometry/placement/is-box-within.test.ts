import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { isBoxWithin } from "./is-box-within";
import { isPointInBox } from "./is-point-in-box";

/** The box of the arch of `DERIV-box-overlap`, from (0, −30) to (100, 0). */
const ARCH_BOX = { maxX: 100, maxY: 0, minX: 0, minY: -30 };

/** A box on integers, its minimum and its maximum drawn on each axis. */
const BOX = fc
  .tuple(
    fc.integer({ max: 50, min: -50 }),
    fc.integer({ max: 50, min: -50 }),
    fc.integer({ max: 50, min: 0 }),
    fc.integer({ max: 50, min: 0 }),
  )
  .map(([minX, minY, width, height]) => ({
    maxX: minX + width,
    maxY: minY + height,
    minX,
    minY,
  }));

describe("isBoxWithin (DERIV-box-overlap step 2)", () => {
  it("holds within a larger box and within itself", () => {
    // −1 ≤ 0, 100 ≤ 101, −31 ≤ −30, 0 ≤ 1.
    expect(isBoxWithin(ARCH_BOX, { maxX: 101, maxY: 1, minX: -1, minY: -31 })).toBe(true);
    expect(isBoxWithin(ARCH_BOX, ARCH_BOX)).toBe(true);
  });

  it("fails when one side passes beyond the box", () => {
    // minY −30 < −29; maxX 100 > 99; minX 0 < 1; maxY 0 > −1.
    expect(isBoxWithin(ARCH_BOX, { ...ARCH_BOX, minY: -29 })).toBe(false);
    expect(isBoxWithin(ARCH_BOX, { ...ARCH_BOX, maxX: 99 })).toBe(false);
    expect(isBoxWithin(ARCH_BOX, { ...ARCH_BOX, minX: 1 })).toBe(false);
    expect(isBoxWithin(ARCH_BOX, { ...ARCH_BOX, maxY: -1 })).toBe(false);
  });
});

describe("isBoxWithin (ADR-0003, EPSILON)", () => {
  it("takes a side beyond by a rounding error as on the box, not one beyond by more", () => {
    // The left of a rounded rectangle's contour box, from its arc, at −3.6 × 10⁻¹⁵ ≥ 0 − 10⁻⁹;
    // −10⁻⁶ < −10⁻⁹.
    const outer = { maxX: 100, maxY: 100, minX: 0, minY: 0 };

    expect(isBoxWithin({ ...outer, minX: -3.6e-15 }, outer)).toBe(true);
    // The other three sides, each past by 10⁻¹² < 10⁻⁹: 100 + 10⁻¹² ≤ 100 + 10⁻⁹,
    // −10⁻¹² ≥ 0 − 10⁻⁹.
    expect(isBoxWithin({ ...outer, maxX: 100 + 1e-12 }, outer)).toBe(true);
    expect(isBoxWithin({ ...outer, minY: -1e-12 }, outer)).toBe(true);
    expect(isBoxWithin({ ...outer, maxY: 100 + 1e-12 }, outer)).toBe(true);
    expect(isBoxWithin({ ...outer, minX: -1e-6 }, outer)).toBe(false);
    expect(isBoxWithin({ ...outer, maxY: 100 + 1e-6 }, outer)).toBe(false);
  });
});

describe("isBoxWithin (properties)", () => {
  test.prop({ inner: BOX, outer: BOX })(
    "holds exactly when both corners of the inner box are in the outer one",
    ({ inner, outer }) => {
      expect(isBoxWithin(inner, outer)).toBe(
        isPointInBox(outer, { x: inner.minX, y: inner.minY }) &&
          isPointInBox(outer, { x: inner.maxX, y: inner.maxY }),
      );
    },
  );
});
