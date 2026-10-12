import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { EPSILON } from "../math";

import { edgeLengths } from "./edge-lengths";
import { effectiveRadii } from "./effective-radii";

import type { Corner } from "./corner";

/** Decimals compared, from `EPSILON = 1e-9` (Q10). */
const DECIMALS = 9;

/**
 * Corners of a rectangle clockwise from the top-left vertex (Q11).
 *
 * @param width - width, >= 0
 * @param height - height, >= 0
 * @param radii - radius requested at each corner, in drawing order
 * @returns the four corners
 */
function rectangle(width: number, height: number, radii: readonly number[]): Corner[] {
  const points = [
    { x: 0, y: 0 },
    { x: width, y: 0 },
    { x: width, y: height },
    { x: 0, y: height },
  ];

  return points.map((point, index) => ({ point, radius: radii[index] ?? 0 }));
}

/** Integer size of a side (Q15: sizes >= 0). */
const SIZE = fc.integer({ max: 1000, min: 0 });

/** Requested radii of the four corners, integers >= 0, often larger than the sides. */
const RADII = fc.array(fc.integer({ max: 2000, min: 0 }), { maxLength: 4, minLength: 4 });

describe("effectiveRadii (DERIV-local-radius-clamp, ADR-0007)", () => {
  it("[F01.AC3] caps a single corner of 1000 on a square of 100 at 100", () => {
    // f = 100 / 1000 on both edges of corner 0: 0.1 × 1000 = 100.
    const radii = effectiveRadii(rectangle(100, 100, [1000, 0, 0, 0]));

    expect(radii[0]).toBeCloseTo(100, DECIMALS);
    expect(radii.slice(1)).toEqual([0, 0, 0]);
  });

  it("[F01.AC3] gives 12.5 to two corners of 100 on an edge of 25", () => {
    // Rectangle 100 × 25: S = 200 on the short edges → g = 0.125 → 12.5 (12.500000000000004 in floats).
    const radii = effectiveRadii(rectangle(100, 25, [100, 100, 100, 100]));

    for (const radius of radii) {
      expect(radius).toBeCloseTo(12.5, DECIMALS);
    }
  });

  it("[F01.AC3] gives the same circle with radius 1000 on every corner of a square of 100", () => {
    // S = 1000 + 1000 = 2000 on every edge → f = 100 / 2000 = 0.05 → r′ = 0.05 × 1000 = 50.
    const radii = effectiveRadii(rectangle(100, 100, [1000, 1000, 1000, 1000]));

    for (const radius of radii) {
      expect(radius).toBeCloseTo(50, DECIMALS);
    }
  });

  it("[F01.AC3] keeps 50 on a square of 100: a circle", () => {
    // S = 100 = L on every edge → f = 1.
    const radii = effectiveRadii(rectangle(100, 100, [50, 50, 50, 50]));

    for (const radius of radii) {
      expect(radius).toBeCloseTo(50, DECIMALS);
    }
  });

  it("keeps the requested radii on a rectangle of size 0, without NaN (Q15)", () => {
    // No setback anywhere (τ = 0): every factor is 1; no arc will be drawn.
    expect(effectiveRadii(rectangle(0, 50, [10, 10, 10, 10]))).toEqual([10, 10, 10, 10]);
  });

  test.prop({ height: SIZE, radii: RADII, width: SIZE })(
    "never increases a radius and never makes it negative",
    ({ height, radii, width }) => {
      const effective = effectiveRadii(rectangle(width, height, radii));

      for (const [index, radius] of effective.entries()) {
        expect(radius).toBeGreaterThanOrEqual(0);
        expect(radius).toBeLessThanOrEqual((radii[index] ?? 0) * (1 + EPSILON));
      }
    },
  );

  test.prop({ height: SIZE, radii: RADII, width: SIZE })(
    "fits the setbacks of every edge in its length: arcs never overlap",
    ({ height, radii, width }) => {
      const corners = rectangle(width, height, radii);
      const effective = effectiveRadii(corners);
      // Independent of the production setbacks: on a rectangle with both sides > 0 every corner
      // turns by π/2, so s = r′ · tan(π/4) = r′; with a side of 0 every τ is 0, so s = 0.
      const setbacks = effective.map((radius) => (width > 0 && height > 0 ? radius : 0));

      const lengths = edgeLengths(corners.map((corner) => corner.point));

      for (const [index, length] of lengths.entries()) {
        const demand = (setbacks[index] ?? 0) + (setbacks[(index + 1) % 4] ?? 0);

        expect(demand).toBeLessThanOrEqual(length * (1 + EPSILON) + EPSILON);
      }
    },
  );
});
