import { CUBIC_DEGREE } from "./cubic-degree";

/** Coefficients of a quadratic `a t² + b t + c`. */
export type Quadratic = {
  /** Coefficient of t². */
  readonly a: number;
  /** Coefficient of t. */
  readonly b: number;
  /** Constant term. */
  readonly c: number;
};

/**
 * Derivative of one axis of a cubic as a quadratic in t (`DERIV-cubic-sides` step 2).
 *
 * Formula: a = 3(−w₀ + 3w₁ − 3w₂ + w₃), b = 6(w₀ − 2w₁ + w₂), c = 3(w₁ − w₀).
 *
 * @kind math
 * @param weights - the four values of the axis, from the start to the end
 * @returns a, b and c
 * @see DERIV-cubic-sides
 */
export function derivativeCoefficients(
  weights: readonly [number, number, number, number],
): Quadratic {
  const [w0, w1, w2, w3] = weights;

  return {
    a: CUBIC_DEGREE * (-w0 + CUBIC_DEGREE * w1 - CUBIC_DEGREE * w2 + w3),
    b: 2 * CUBIC_DEGREE * (w0 - 2 * w1 + w2),
    c: CUBIC_DEGREE * (w1 - w0),
  };
}
