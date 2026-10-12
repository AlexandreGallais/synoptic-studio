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
