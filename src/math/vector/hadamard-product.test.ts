import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { integerPoint } from "../../../tests";

import { hadamardProduct } from "./hadamard-product";
import { scale } from "./scale";

describe("hadamardProduct (REF-WIKI-HADAMARD)", () => {
  it("multiplies each component by its own factor", () => {
    // (4, 5) ⊙ (2, 3) = (8, 15).
    expect(hadamardProduct({ x: 4, y: 5 }, { x: 2, y: 3 })).toEqual({ x: 8, y: 15 });
  });

  test.prop({ factor: fc.integer({ max: 100, min: -100 }), v: integerPoint(1e4) })(
    "is a scale when both factors are equal",
    ({ factor, v }) => {
      expect(hadamardProduct(v, { x: factor, y: factor })).toEqual(scale(v, factor));
    },
  );
});
