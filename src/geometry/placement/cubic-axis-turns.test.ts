import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { cubicAxisTurns } from "./cubic-axis-turns";
import { derivativeCoefficients } from "./derivative-coefficients";

/** A value of an axis, an integer. */
const WEIGHT = fc.integer({ max: 1000, min: -1000 });

describe("cubicAxisTurns (DERIV-cubic-sides steps 2 and 3)", () => {
  it("finds the arch's top at t = ½, a = 0", () => {
    // 0, −40, −40, 0: a = 3(0 − 120 + 120 + 0) = 0, b = 6(0 + 80 − 40) = 240, c = 3 · (−40) =
    // −120; t₁ = q / a infinite, t₂ = c / q = −120 / −240 = ½.
    expect(cubicAxisTurns([0, -40, -40, 0])).toEqual([0.5]);
  });

  it("finds an S's two turns, (3 ± √3) / 6, in the order of the roots", () => {
    // 0, −30, 30, 0: a = −540, b = 540, c = −90, so 6t² − 6t + 1 = 0;
    // q = −(540 + √97 200) / 2 ≈ −425.885, t₁ = q / a ≈ 0.788675, t₂ = c / q ≈ 0.211325.
    const turns = cubicAxisTurns([0, -30, 30, 0]);

    expect(turns).toHaveLength(2);
    expect(turns[0]).toBeCloseTo((3 + Math.sqrt(3)) / 6, 12);
    expect(turns[1]).toBeCloseTo((3 - Math.sqrt(3)) / 6, 12);
  });

  it("leaves out a turn at an end: 0, −15, 0, 0 turns at ⅓ and at 1", () => {
    // a = 3(0 − 45 − 0 + 0) = −135, b = 6(0 + 30 + 0) = 180, c = −45: −135t² + 180t − 45 = 0
    // at t = ⅓ and t = 1, the end left out.
    const turns = cubicAxisTurns([0, -15, 0, 0]);

    expect(turns).toHaveLength(1);
    expect(turns[0]).toBeCloseTo(1 / 3, 12);
  });

  it("gives none for a constant axis (NaN roots) nor for an even one (a NaN and an infinity)", () => {
    // 5, 5, 5, 5: a = b = c = 0, q = 0, both roots 0 / 0. 0, 10, 20, 30: a = b = 0, c = 30,
    // q = −0, t₁ = −0 / 0, t₂ = 30 / −0 = −∞.
    expect(cubicAxisTurns([5, 5, 5, 5])).toEqual([]);
    expect(cubicAxisTurns([0, 10, 20, 30])).toEqual([]);
  });
});

describe("cubicAxisTurns (properties)", () => {
  test.prop({ weights: fc.tuple(WEIGHT, WEIGHT, WEIGHT, WEIGHT) })(
    "gives at most two values, each strictly between 0 and 1 where the derivative is 0",
    ({ weights }) => {
      const { a, b, c } = derivativeCoefficients(weights);
      const turns = cubicAxisTurns(weights);

      expect(turns.length).toBeLessThanOrEqual(2);

      for (const t of turns) {
        expect(t).toBeGreaterThan(0);
        expect(t).toBeLessThan(1);

        expect(Math.abs(a * t * t + b * t + c)).toBeLessThanOrEqual(
          1e-9 * (Math.abs(a) + Math.abs(b) + Math.abs(c)),
        );
      }
    },
  );
});
