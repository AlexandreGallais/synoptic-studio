import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { bisection } from "./bisection";

describe("bisection (REF-MATHWORLD-BISECTION)", () => {
  it("finds √2 as the end of the values whose square is below 2", () => {
    // The bracket [0, 2] halved 60 times: an error below 2 · 2⁻⁶⁰ ≈ 1.7e-18.
    expect(bisection((value) => value * value < 2, 0, 2)).toBeCloseTo(Math.SQRT2, 15);
  });

  it("gives the upper end when the predicate is always true, the lower end when always false", () => {
    expect(bisection(() => true, -3, 0)).toBeCloseTo(0, 15);
    expect(bisection(() => false, -3, 0)).toBeCloseTo(-3, 15);
  });

  test.prop([fc.double({ max: 1000, min: -1000, noNaN: true })])(
    "finds any boundary of a bracket within 1e-12",
    (boundary) => {
      expect(Math.abs(bisection((value) => value < boundary, -1000, 1000) - boundary)).toBeLessThan(
        1e-12,
      );
    },
  );
});
