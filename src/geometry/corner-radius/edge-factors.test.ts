import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { edgeFactors } from "./edge-factors";

describe("edgeFactors (DERIV-local-radius-clamp step 2)", () => {
  it("reduces only the edges next to a too large corner", () => {
    // Square 100, one corner at 1000: the top edge (corners 0 and 1) and the left edge
    // (corners 3 and 0) carry 1000 → 100 / 1000; the two others carry nothing.
    expect(edgeFactors([100, 100, 100, 100], [1000, 0, 0, 0])).toEqual([0.1, 1, 1, 0.1]);
  });

  it("sums the setbacks of both ends of an edge", () => {
    // Rectangle 100 × 25, setbacks 100: S = 200 on every edge → 100 / 200 and 25 / 200.
    expect(edgeFactors([100, 25, 100, 25], [100, 100, 100, 100])).toEqual([0.5, 0.125, 0.5, 0.125]);
  });

  it("gives nothing for an empty contour", () => {
    expect(edgeFactors([], [])).toEqual([]);
  });
});

/** Arbitrary length or setback. */
const SIZE = fc.integer({ max: 5000, min: 0 });

/**
 * Checks that a factor reduces a demand as little as fits in its edge: none when the demand fits,
 * else the reduced demand fills the edge.
 *
 * @param length - length of the edge
 * @param demand - sum of the two setbacks it carries
 * @param factor - factor computed for the edge
 */
function expectFittingFactor(length: number, demand: number, factor: number): void {
  const isFitting = demand <= length;

  expect(factor).toBeGreaterThanOrEqual(0);
  expect(factor).toBeLessThanOrEqual(1);
  expect(factor * demand).toBeLessThanOrEqual(length * (1 + 1e-12));
  expect(isFitting ? factor : factor * demand).toBeCloseTo(isFitting ? 1 : length, 9);
}

describe("edgeFactors (properties)", () => {
  test.prop({ pairs: fc.array(fc.tuple(SIZE, SIZE), { minLength: 1 }) })(
    "gives each edge a factor in [0, 1] that fits its two reduced setbacks in it",
    ({ pairs }) => {
      const lengths = pairs.map(([length]) => length);
      const setbacks = pairs.map(([, setback]) => setback);
      const factors = edgeFactors(lengths, setbacks);

      expect(factors).toHaveLength(lengths.length);

      // Edge i carries the setbacks of corners i and i + 1, the last edge those of the last and the first.
      const demands = pairs.map(
        ([, setback], index) => setback + (pairs[(index + 1) % pairs.length]?.[1] ?? 0),
      );

      for (const [index, [length]] of pairs.entries()) {
        expectFittingFactor(length, demands[index] ?? NaN, factors[index] ?? NaN);
      }
    },
  );
});
