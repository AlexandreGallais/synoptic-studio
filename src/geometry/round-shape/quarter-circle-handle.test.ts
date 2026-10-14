import { describe, expect, it } from "vitest";

import { QUARTER_CIRCLE_HANDLE } from "./quarter-circle-handle";

describe("QUARTER_CIRCLE_HANDLE (DERIV-round-shape-curve)", () => {
  it("is 4(√2 − 1)/3, about 0.55228 (REF-POMAX-BEZIER)", () => {
    expect(QUARTER_CIRCLE_HANDLE).toBeCloseTo(0.5522847498, 10);
  });

  it("puts the middle of the quarter on the circle: (4 + 3κ)/8 = √2/2", () => {
    // B(½) of (1, 0), (1, κ), (κ, 1), (0, 1) is ((4 + 3κ)/8, (4 + 3κ)/8) (step 1).
    expect((4 + 3 * QUARTER_CIRCLE_HANDLE) / 8).toBeCloseTo(Math.SQRT1_2, 15);
  });
});
