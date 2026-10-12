import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { cyclicItem } from "./cyclic-item";
import { vertexFactors } from "./vertex-factors";

describe("vertexFactors (DERIV-local-radius-clamp step 3)", () => {
  it("takes the smaller factor of the two edges of each vertex", () => {
    // Edges [top, right, bottom, left] = [0.1, 1, 1, 0.1]: vertex 0 between left and top → 0.1,
    // vertex 1 between top and right → 0.1, vertex 2 → 1, vertex 3 between bottom and left → 0.1.
    expect(vertexFactors([0.1, 1, 1, 0.1])).toEqual([0.1, 0.1, 1, 0.1]);
  });

  it("gives nothing for an empty contour", () => {
    expect(vertexFactors([])).toEqual([]);
  });
});

describe("vertexFactors (properties)", () => {
  test.prop({ factors: fc.array(fc.double({ max: 1, min: 0, noNaN: true }), { minLength: 1 }) })(
    "never exceeds the factor of either edge of the vertex",
    ({ factors }) => {
      for (const [index, factor] of vertexFactors(factors).entries()) {
        expect(factor).toBeLessThanOrEqual(cyclicItem(factors, index, 1));
        expect(factor).toBeLessThanOrEqual(cyclicItem(factors, index - 1, 1));
      }
    },
  );
});
