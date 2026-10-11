import { describe, expect, it } from "vitest";

import { alignmentFactor } from "./alignment-factor";

describe("alignmentFactor (DERIV-regular-polygon-fit step 5)", () => {
  it("puts no room before the shape at min, half at mid, all at max", () => {
    // SVG 2 §8.2: xMin adds nothing, xMid adds (e-width − vb-width × scale) / 2, xMax all of it.
    expect(alignmentFactor("min")).toBe(0);
    expect(alignmentFactor("mid")).toBe(0.5);
    expect(alignmentFactor("max")).toBe(1);
  });
});
