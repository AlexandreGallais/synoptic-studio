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

describe("edgeFactors (properties)", () => {
  test.prop({ pairs: fc.array(fc.tuple(SIZE, SIZE), { minLength: 1 }) })(
    "gives each edge a factor in [0, 1] that fits its two reduced setbacks in it",
    ({ pairs }) => {
      const lengths = pairs.map(([length]) => length);
      const setbacks = pairs.map(([, setback]) => setback);
      const factors = edgeFactors(lengths, setbacks);

      expect(factors).toHaveLength(lengths.length);

      for (const [index, factor] of factors.entries()) {
        // Edge i carries the setbacks of corners i and i + 1, the last edge those of the last and the first.
        const demand = (setbacks[index] ?? 0) + (setbacks[(index + 1) % setbacks.length] ?? 0);

        expect(factor).toBeGreaterThanOrEqual(0);
        expect(factor).toBeLessThanOrEqual(1);
        expect(factor * demand).toBeLessThanOrEqual((lengths[index] ?? 0) * (1 + 1e-12));
      }
    },
  );
});
