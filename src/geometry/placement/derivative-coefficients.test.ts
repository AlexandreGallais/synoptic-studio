import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { derivativeCoefficients } from "./derivative-coefficients";

/** Four integer values of one axis. */
const WEIGHTS = fc.tuple(
  fc.integer({ max: 1e4, min: -1e4 }),
  fc.integer({ max: 1e4, min: -1e4 }),
  fc.integer({ max: 1e4, min: -1e4 }),
  fc.integer({ max: 1e4, min: -1e4 }),
);

describe("derivativeCoefficients (DERIV-cubic-sides step 2)", () => {
  it("gives a, b, c of the side (100, 0) → (200, 0) through (130, −15)", () => {
    // x: 3(−100 + 390 − 600 + 200) = −330, 6(100 − 260 + 200) = 240, 3 · 30 = 90.
    expect(derivativeCoefficients([100, 130, 200, 200])).toEqual({ a: -330, b: 240, c: 90 });
    // y: 3(0 − 45 + 0 + 0) = −135, 6(0 + 30 + 0) = 180, 3 · (−15) = −45.
    expect(derivativeCoefficients([0, -15, 0, 0])).toEqual({ a: -135, b: 180, c: -45 });
  });

  test.prop({ weights: WEIGHTS })(
    "gives B′(0) = 3(w₁ − w₀) and B′(1) = 3(w₃ − w₂)",
    ({ weights }) => {
      const { a, b, c } = derivativeCoefficients(weights);
      const [w0, w1, w2, w3] = weights;

      expect(c).toBe(3 * (w1 - w0));
      expect(a + b + c).toBe(3 * (w3 - w2));
    },
  );
});
