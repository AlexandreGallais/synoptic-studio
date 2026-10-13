import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { mirrorAcrossAxis } from "./mirror-across-axis";

describe("mirrorAcrossAxis (DERIV-stretched-polygon step 1)", () => {
  it("sends the right end of the half base to the left end", () => {
    // (x, y) ↦ (−x, y): (1/2, 0) ↦ (−1/2, 0).
    expect(mirrorAcrossAxis({ x: 0.5, y: 0 })).toEqual({ x: -0.5, y: 0 });
  });

  it("keeps a point of the axis", () => {
    expect(mirrorAcrossAxis({ x: 0, y: 2 })).toEqual({ x: -0, y: 2 });
  });
});

describe("mirrorAcrossAxis (properties)", () => {
  test.prop({ x: fc.integer(), y: fc.integer() })(
    "gives back the point when applied twice, its height unchanged",
    ({ x, y }) => {
      expect(mirrorAcrossAxis(mirrorAcrossAxis({ x, y }))).toEqual({ x, y });
      expect(mirrorAcrossAxis({ x, y }).y).toBe(y);
      expect(x + mirrorAcrossAxis({ x, y }).x).toBe(0);
    },
  );
});
